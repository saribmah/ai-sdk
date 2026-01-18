use async_trait::async_trait;
use llm_kit_provider::transcription_model::call_options::TranscriptionModelCallOptions;
use llm_kit_provider::transcription_model::{
    TranscriptSegment, TranscriptionModelResponse, TranscriptionModelResponseMetadata,
};
use llm_kit_provider::TranscriptionModel as TranscriptionModelTrait;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use std::collections::HashMap;
use std::time::SystemTime;
use tokio::time::{sleep, Duration};

use super::api_types::{
    AssemblyAISubmitResponse, AssemblyAITranscriptionResponse, AssemblyAIUploadResponse,
    TranscriptionStatus,
};
use super::options::{AssemblyAITranscriptionModelId, AssemblyAITranscriptionOptions};
use crate::error::{parse_error_response, AssemblyAIError};

/// Configuration for the AssemblyAI transcription model.
pub struct AssemblyAITranscriptionConfig {
    pub provider: String,
    pub base_url: String,
    pub api_key: Option<String>,
    pub headers: HashMap<String, String>,
    pub polling_interval_ms: u64,
}

/// AssemblyAI transcription model implementation.
pub struct AssemblyAITranscriptionModel {
    model_id: AssemblyAITranscriptionModelId,
    config: AssemblyAITranscriptionConfig,
    client: reqwest::Client,
}

impl AssemblyAITranscriptionModel {
    /// Create a new AssemblyAI transcription model.
    pub fn new(
        model_id: AssemblyAITranscriptionModelId,
        config: AssemblyAITranscriptionConfig,
    ) -> Self {
        Self {
            model_id,
            config,
            client: reqwest::Client::new(),
        }
    }

    /// Build headers for API requests.
    fn build_headers(&self) -> Result<HeaderMap, Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();

        // Add API key authorization
        if let Some(api_key) = &self.config.api_key {
            let header_name: reqwest::header::HeaderName = AUTHORIZATION;
            let header_value: reqwest::header::HeaderValue = api_key.parse()?;
            headers.insert(header_name, header_value);
        } else if let Ok(api_key) = std::env::var("ASSEMBLYAI_API_KEY") {
            let header_name: reqwest::header::HeaderName = AUTHORIZATION;
            let header_value: reqwest::header::HeaderValue = api_key.parse()?;
            headers.insert(header_name, header_value);
        }

        // Add custom headers
        for (key, value) in &self.config.headers {
            let header_name: reqwest::header::HeaderName = key.parse()?;
            let header_value: reqwest::header::HeaderValue = value.parse()?;
            headers.insert(header_name, header_value);
        }

        // Add user agent
        let user_agent = format!("ai-sdk/assemblyai/{}", crate::VERSION);
        let header_name: reqwest::header::HeaderName = "user-agent".parse()?;
        let header_value: reqwest::header::HeaderValue = user_agent.parse()?;
        headers.insert(header_name, header_value);

        Ok(headers)
    }

    /// Upload audio data to AssemblyAI.
    async fn upload_audio(
        &self,
        audio_data: &[u8],
        abort_signal: Option<&tokio::sync::watch::Receiver<bool>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}/v2/upload", self.config.base_url);
        let mut headers = self.build_headers()?;
        headers.insert(CONTENT_TYPE, "application/octet-stream".parse()?);

        let request = self
            .client
            .post(&url)
            .headers(headers)
            .body(audio_data.to_vec());

        // Check abort signal before sending
        if let Some(rx) = abort_signal {
            if *rx.borrow() {
                return Err(Box::new(AssemblyAIError::Aborted));
            }
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            let body = response.text().await?;
            let error =
                parse_error_response(&body).unwrap_or_else(|_| AssemblyAIError::UploadFailed(body));
            return Err(Box::new(error));
        }

        let upload_response: AssemblyAIUploadResponse = response.json().await?;
        Ok(upload_response.upload_url)
    }

    /// Submit transcription request.
    async fn submit_transcription(
        &self,
        upload_url: String,
        options: &AssemblyAITranscriptionOptions,
        abort_signal: Option<&tokio::sync::watch::Receiver<bool>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}/v2/transcript", self.config.base_url);
        let headers = self.build_headers()?;

        // Build request body
        let mut body = serde_json::json!({
            "audio_url": upload_url,
            "speech_model": self.model_id.as_str()
        });

        // Add all provider options to the body
        let options_map = serde_json::to_value(options)?;
        if let serde_json::Value::Object(map) = options_map {
            for (key, value) in map {
                // Convert camelCase to snake_case
                let snake_key = camel_to_snake(&key);
                if let serde_json::Value::Object(obj) = &mut body {
                    obj.insert(snake_key, value);
                }
            }
        }

        // Check abort signal
        if let Some(rx) = abort_signal {
            if *rx.borrow() {
                return Err(Box::new(AssemblyAIError::Aborted));
            }
        }

        let response = self
            .client
            .post(&url)
            .headers(headers)
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let body_text = response.text().await?;
            let error = parse_error_response(&body_text).unwrap_or_else(|_| {
                AssemblyAIError::InvalidResponse(format!(
                    "Failed to submit transcription: {}",
                    body_text
                ))
            });
            return Err(Box::new(error));
        }

        let submit_response: AssemblyAISubmitResponse = response.json().await?;
        Ok(submit_response.id)
    }

    /// Poll for transcription completion.
    async fn wait_for_completion(
        &self,
        transcript_id: &str,
        abort_signal: Option<&tokio::sync::watch::Receiver<bool>>,
    ) -> Result<AssemblyAITranscriptionResponse, Box<dyn std::error::Error>> {
        let url = format!("{}/v2/transcript/{}", self.config.base_url, transcript_id);
        let headers = self.build_headers()?;
        let polling_interval = Duration::from_millis(self.config.polling_interval_ms);

        loop {
            // Check abort signal
            if let Some(rx) = abort_signal {
                if *rx.borrow() {
                    return Err(Box::new(AssemblyAIError::Aborted));
                }
            }

            let response = self
                .client
                .get(&url)
                .headers(headers.clone())
                .send()
                .await?;

            if !response.status().is_success() {
                let body = response.text().await?;
                let error = parse_error_response(&body)
                    .unwrap_or_else(|_| AssemblyAIError::InvalidResponse(body));
                return Err(Box::new(error));
            }

            let transcript: AssemblyAITranscriptionResponse = response.json().await?;

            match transcript.status {
                TranscriptionStatus::Completed => return Ok(transcript),
                TranscriptionStatus::Error => {
                    let error_msg = transcript
                        .error
                        .unwrap_or_else(|| "Unknown error".to_string());
                    return Err(Box::new(AssemblyAIError::TranscriptionFailed(error_msg)));
                }
                TranscriptionStatus::Queued | TranscriptionStatus::Processing => {
                    sleep(polling_interval).await;
                }
            }
        }
    }

    /// Parse provider options from provider options map.
    fn parse_provider_options(
        &self,
        provider_options: Option<
            &std::collections::HashMap<
                String,
                std::collections::HashMap<String, serde_json::Value>,
            >,
        >,
    ) -> Result<AssemblyAITranscriptionOptions, Box<dyn std::error::Error>> {
        match provider_options {
            Some(opts) => {
                // Get assemblyai specific options
                if let Some(assemblyai_opts) = opts.get("assemblyai") {
                    let value = serde_json::to_value(assemblyai_opts)?;
                    let options: AssemblyAITranscriptionOptions = serde_json::from_value(value)?;
                    Ok(options)
                } else {
                    Ok(AssemblyAITranscriptionOptions::default())
                }
            }
            None => Ok(AssemblyAITranscriptionOptions::default()),
        }
    }
}

#[async_trait]
impl TranscriptionModelTrait for AssemblyAITranscriptionModel {
    fn provider(&self) -> &str {
        &self.config.provider
    }

    fn model_id(&self) -> &str {
        self.model_id.as_str()
    }

    async fn do_generate(
        &self,
        options: TranscriptionModelCallOptions,
    ) -> Result<TranscriptionModelResponse, Box<dyn std::error::Error>> {
        let _start_time = SystemTime::now();

        // Parse provider options
        let provider_opts = self.parse_provider_options(options.provider_options.as_ref())?;

        // Convert abort signal to watch channel if provided
        let abort_receiver = options.abort_signal.as_ref().map(|signal| {
            let (tx, rx) = tokio::sync::watch::channel(false);
            // Spawn a task to watch the abort signal
            let signal_clone = signal.clone();
            tokio::spawn(async move {
                signal_clone.cancelled().await;
                let _ = tx.send(true);
            });
            rx
        });

        // Convert audio data to bytes
        let audio_bytes = match &options.audio {
            llm_kit_provider::transcription_model::call_options::TranscriptionAudioData::Binary(
                bytes,
            ) => bytes.clone(),
            llm_kit_provider::transcription_model::call_options::TranscriptionAudioData::Base64(
                s,
            ) => {
                // Decode base64 if needed
                base64::Engine::decode(&base64::engine::general_purpose::STANDARD, s).map_err(
                    |e| {
                        Box::new(AssemblyAIError::InvalidResponse(format!(
                            "Invalid base64 audio: {}",
                            e
                        ))) as Box<dyn std::error::Error>
                    },
                )?
            }
        };

        // Step 1: Upload audio
        let upload_url = self
            .upload_audio(&audio_bytes, abort_receiver.as_ref())
            .await?;

        // Step 2: Submit transcription request
        let transcript_id = self
            .submit_transcription(upload_url, &provider_opts, abort_receiver.as_ref())
            .await?;

        // Step 3: Poll for completion
        let transcript = self
            .wait_for_completion(&transcript_id, abort_receiver.as_ref())
            .await?;

        // Build response
        let text = transcript.text.clone().unwrap_or_default();
        let segments = transcript
            .words
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(|word| TranscriptSegment::new(word.text, word.start, word.end))
            .collect();

        let response_metadata = TranscriptionModelResponseMetadata::new(self.model_id.as_str())
            .with_body(serde_json::to_value(&transcript)?);

        let mut result =
            TranscriptionModelResponse::new(text, response_metadata).with_segments(segments);

        if let Some(lang) = transcript.language_code.clone() {
            result = result.with_language(lang);
        }

        if let Some(duration) = transcript.audio_duration {
            result = result.with_duration(duration);
        }

        Ok(result)
    }
}

/// Convert camelCase to snake_case.
fn camel_to_snake(s: &str) -> String {
    let mut result = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(ch.to_ascii_lowercase());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_to_snake() {
        assert_eq!(camel_to_snake("audioEndAt"), "audio_end_at");
        assert_eq!(camel_to_snake("speakerLabels"), "speaker_labels");
        assert_eq!(camel_to_snake("test"), "test");
    }
}
