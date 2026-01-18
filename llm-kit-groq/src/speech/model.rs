use super::options::GroqSpeechOptions;
use async_trait::async_trait;
use llm_kit_provider::speech_model::{
    AudioData, SpeechModel, SpeechModelRequestMetadata, SpeechModelResponse,
    SpeechModelResponseMetadata, call_options::SpeechModelCallOptions,
    call_warning::SpeechModelCallWarning,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;

/// Configuration for Groq speech models.
#[derive(Clone)]
pub struct GroqSpeechConfig {
    pub provider: String,
    pub base_url: String,
    pub headers: Arc<dyn Fn() -> HashMap<String, String> + Send + Sync>,
}

/// Groq speech model implementation.
///
/// Implements text-to-speech using Groq's API with PlayAI models.
pub struct GroqSpeechModel {
    model_id: String,
    config: GroqSpeechConfig,
}

impl GroqSpeechModel {
    /// Create a new Groq speech model.
    pub fn new(model_id: String, config: GroqSpeechConfig) -> Self {
        Self { model_id, config }
    }

    /// Build the request body for Groq TTS API.
    fn build_request_body(
        &self,
        options: &SpeechModelCallOptions,
        provider_options: Option<&GroqSpeechOptions>,
    ) -> (GroqSpeechRequest, Vec<SpeechModelCallWarning>) {
        let mut warnings = Vec::new();
        let mut request = GroqSpeechRequest {
            model: self.model_id.clone(),
            input: options.text.clone(),
            voice: options
                .voice
                .clone()
                .unwrap_or_else(|| "Fritz-PlayAI".to_string()),
            response_format: Some(
                options
                    .output_format
                    .clone()
                    .unwrap_or_else(|| "wav".to_string()),
            ),
            speed: options.speed.map(|s| s as f32),
            sample_rate: None,
        };

        // Apply provider-specific options
        if let Some(groq_opts) = provider_options
            && let Some(sample_rate) = groq_opts.sample_rate
        {
            request.sample_rate = Some(sample_rate);
        }

        // Warn about unsupported options
        if options.instructions.is_some() {
            warnings.push(SpeechModelCallWarning::UnsupportedSetting {
                setting: "instructions".to_string(),
                details: Some(
                    "Groq speech models do not support instructions parameter".to_string(),
                ),
            });
        }

        if options.language.is_some() {
            warnings.push(SpeechModelCallWarning::UnsupportedSetting {
                setting: "language".to_string(),
                details: Some("Groq speech models do not support language parameter".to_string()),
            });
        }

        (request, warnings)
    }
}

/// Request body for Groq text-to-speech API.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GroqSpeechRequest {
    /// The text to generate audio for.
    input: String,
    /// One of the available TTS models.
    model: String,
    /// The voice to use when generating the audio.
    voice: String,
    /// The format of the generated audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<String>,
    /// The speed of the generated audio (0.5 - 5.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    speed: Option<f32>,
    /// The sample rate for generated audio.
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_rate: Option<u32>,
}

#[async_trait]
impl SpeechModel for GroqSpeechModel {
    fn specification_version(&self) -> &str {
        "v3"
    }

    fn provider(&self) -> &str {
        &self.config.provider
    }

    fn model_id(&self) -> &str {
        &self.model_id
    }

    async fn do_generate(
        &self,
        options: SpeechModelCallOptions,
    ) -> Result<SpeechModelResponse, Box<dyn std::error::Error>> {
        let timestamp = SystemTime::now();

        // Parse provider options
        let provider_options: Option<GroqSpeechOptions> =
            if let Some(ref provider_opts) = options.provider_options {
                if let Some(groq_opts) = provider_opts.get("groq") {
                    Some(serde_json::from_value(serde_json::to_value(groq_opts)?)?)
                } else {
                    None
                }
            } else {
                None
            };

        // Validate provider options
        if let Some(ref opts) = provider_options {
            opts.validate()
                .map_err(|e| format!("Invalid provider options: {}", e))?;
        }

        // Build request body and collect warnings
        let (request_body, warnings) = self.build_request_body(&options, provider_options.as_ref());

        // Build URL
        let url = format!("{}/audio/speech", self.config.base_url);

        // Get headers
        let mut headers = (self.config.headers)();
        if let Some(ref opts_headers) = options.headers {
            headers.extend(opts_headers.clone());
        }

        // Make the API request
        let client = reqwest::Client::new();
        let mut request_builder = client.post(&url).json(&request_body);

        // Add headers
        for (key, value) in headers {
            request_builder = request_builder.header(key, value);
        }

        let response = request_builder.send().await?;
        let status = response.status();
        let response_headers: HashMap<String, String> = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        // Check for errors
        if !status.is_success() {
            let error_body = response.text().await?;
            return Err(format!("Groq TTS API error ({}): {}", status, error_body).into());
        }

        // Get audio data as bytes
        let audio_bytes = response.bytes().await?;
        let audio_data = AudioData::Binary(audio_bytes.to_vec());

        // Build response
        let response_metadata = SpeechModelResponseMetadata {
            timestamp,
            model_id: self.model_id.clone(),
            headers: Some(response_headers),
            body: None, // Binary response, no JSON body
        };

        let mut speech_response = SpeechModelResponse::new(audio_data, response_metadata);

        // Add request metadata
        speech_response.request =
            Some(SpeechModelRequestMetadata::new().with_body(serde_json::to_value(&request_body)?));

        // Add warnings
        if !warnings.is_empty() {
            speech_response.warnings = warnings;
        }

        Ok(speech_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_groq_speech_model_creation() {
        let config = GroqSpeechConfig {
            provider: "groq.speech".to_string(),
            base_url: "https://api.groq.com/openai/v1".to_string(),
            headers: Arc::new(|| {
                let mut headers = HashMap::new();
                headers.insert("Authorization".to_string(), "Bearer test-key".to_string());
                headers
            }),
        };

        let model = GroqSpeechModel::new("playai-tts".to_string(), config);
        assert_eq!(model.model_id(), "playai-tts");
        assert_eq!(model.provider(), "groq.speech");
    }

    #[test]
    fn test_build_request_body() {
        let config = GroqSpeechConfig {
            provider: "groq.speech".to_string(),
            base_url: "https://api.groq.com/openai/v1".to_string(),
            headers: Arc::new(HashMap::new),
        };

        let model = GroqSpeechModel::new("playai-tts".to_string(), config);
        let options = SpeechModelCallOptions::new("Hello, world!".to_string())
            .with_voice("Fritz-PlayAI".to_string())
            .with_output_format("mp3".to_string())
            .with_speed(1.5);

        let (request, warnings) = model.build_request_body(&options, None);

        assert_eq!(request.input, "Hello, world!");
        assert_eq!(request.voice, "Fritz-PlayAI");
        assert_eq!(request.response_format, Some("mp3".to_string()));
        assert_eq!(request.speed, Some(1.5));
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_build_request_with_provider_options() {
        let config = GroqSpeechConfig {
            provider: "groq.speech".to_string(),
            base_url: "https://api.groq.com/openai/v1".to_string(),
            headers: Arc::new(HashMap::new),
        };

        let model = GroqSpeechModel::new("playai-tts".to_string(), config);
        let options = SpeechModelCallOptions::new("Test".to_string());
        let groq_options = GroqSpeechOptions::new().with_sample_rate(24000);

        let (request, _) = model.build_request_body(&options, Some(&groq_options));

        assert_eq!(request.sample_rate, Some(24000));
    }

    #[test]
    fn test_unsupported_options_warnings() {
        let config = GroqSpeechConfig {
            provider: "groq.speech".to_string(),
            base_url: "https://api.groq.com/openai/v1".to_string(),
            headers: Arc::new(HashMap::new),
        };

        let model = GroqSpeechModel::new("playai-tts".to_string(), config);
        let options = SpeechModelCallOptions::new("Test".to_string())
            .with_instructions("Speak clearly".to_string())
            .with_language("en".to_string());

        let (_, warnings) = model.build_request_body(&options, None);

        assert_eq!(warnings.len(), 2);
    }
}
