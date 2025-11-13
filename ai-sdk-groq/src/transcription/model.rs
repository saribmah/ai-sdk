use ai_sdk_provider::transcription_model::call_options::TranscriptionModelCallOptions;
use ai_sdk_provider::transcription_model::call_warning::TranscriptionModelCallWarning;
use ai_sdk_provider::transcription_model::{
    TranscriptSegment, TranscriptionModel, TranscriptionModelResponse,
    TranscriptionModelResponseMetadata,
};
use async_trait::async_trait;
use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

use crate::transcription::options::{GroqTranscriptionModelId, GroqTranscriptionOptions};

/// Configuration for Groq transcription models.
pub struct GroqTranscriptionConfig {
    pub provider: String,
    pub base_url: String,
    pub headers: Box<dyn Fn() -> HashMap<String, String> + Send + Sync>,
}

/// Groq transcription model implementation.
///
/// Supports Whisper models for speech-to-text transcription.
pub struct GroqTranscriptionModel {
    model_id: GroqTranscriptionModelId,
    config: GroqTranscriptionConfig,
}

impl GroqTranscriptionModel {
    /// Create a new Groq transcription model.
    pub fn new(model_id: impl Into<String>, config: GroqTranscriptionConfig) -> Self {
        Self {
            model_id: model_id.into(),
            config,
        }
    }
}

#[async_trait]
impl TranscriptionModel for GroqTranscriptionModel {
    fn provider(&self) -> &str {
        &self.config.provider
    }

    fn model_id(&self) -> &str {
        &self.model_id
    }

    async fn do_generate(
        &self,
        options: TranscriptionModelCallOptions,
    ) -> Result<TranscriptionModelResponse, Box<dyn std::error::Error>> {
        let start_time = SystemTime::now();
        let warnings = Vec::new();

        // Extract Groq-specific options from provider_options
        let groq_options: Option<GroqTranscriptionOptions> =
            if let Some(provider_options) = &options.provider_options {
                if let Some(groq_opts) = provider_options.get("groq") {
                    Some(serde_json::from_value(serde_json::to_value(groq_opts)?)?)
                } else {
                    None
                }
            } else {
                None
            };

        // Build multipart form data
        let mut form = Form::new();

        // Add model
        form = form.text("model", self.model_id.clone());

        // Add audio file
        let audio_bytes = match &options.audio {
            ai_sdk_provider::transcription_model::call_options::TranscriptionAudioData::Base64(
                b64,
            ) => {
                // Decode base64

                base64::Engine::decode(&base64::engine::general_purpose::STANDARD, b64.as_bytes())?
            }
            ai_sdk_provider::transcription_model::call_options::TranscriptionAudioData::Binary(
                bytes,
            ) => bytes.clone(),
        };

        // Get file extension from media type
        let extension = media_type_to_extension(&options.media_type);
        let filename = format!("audio.{}", extension);

        let file_part = Part::bytes(audio_bytes)
            .file_name(filename.clone())
            .mime_str(&options.media_type)?;

        form = form.part("file", file_part);

        // Add Groq-specific options
        if let Some(opts) = groq_options {
            if let Some(language) = opts.language {
                form = form.text("language", language);
            }
            if let Some(prompt) = opts.prompt {
                form = form.text("prompt", prompt);
            }
            if let Some(response_format) = opts.response_format {
                form = form.text("response_format", response_format);
            }
            if let Some(temperature) = opts.temperature {
                form = form.text("temperature", temperature.to_string());
            }
            if let Some(timestamp_granularities) = opts.timestamp_granularities {
                // Groq expects comma-separated values
                form = form.text(
                    "timestamp_granularities[]",
                    timestamp_granularities.join(","),
                );
            }
        }

        // Build request
        let url = format!("{}/audio/transcriptions", self.config.base_url);
        let headers = (self.config.headers)();

        let client = reqwest::Client::new();
        let mut request = client.post(&url).multipart(form);

        // Add headers
        for (key, value) in headers {
            request = request.header(key, value);
        }

        // Add custom headers from options
        if let Some(custom_headers) = &options.headers {
            for (key, value) in custom_headers {
                request = request.header(key, value);
            }
        }

        // Add abort signal if present
        if let Some(abort_signal) = &options.abort_signal {
            let token = abort_signal.clone();
            tokio::select! {
                response = request.send() => {
                    let response = response?;
                    process_response(response, &self.model_id, start_time, warnings).await
                }
                _ = token.cancelled() => {
                    Err("Request was cancelled".into())
                }
            }
        } else {
            let response = request.send().await?;
            process_response(response, &self.model_id, start_time, warnings).await
        }
    }
}

/// Process the HTTP response from Groq API.
async fn process_response(
    response: reqwest::Response,
    model_id: &str,
    start_time: SystemTime,
    warnings: Vec<TranscriptionModelCallWarning>,
) -> Result<TranscriptionModelResponse, Box<dyn std::error::Error>> {
    let status = response.status();
    let headers_map = response
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();

    if !status.is_success() {
        let error_text = response.text().await?;
        return Err(format!("Groq API error ({}): {}", status, error_text).into());
    }

    let body_text = response.text().await?;
    let groq_response: GroqTranscriptionResponse = serde_json::from_str(&body_text)?;

    // Build response metadata
    let response_metadata =
        TranscriptionModelResponseMetadata::with_timestamp(model_id, start_time)
            .with_headers(headers_map)
            .with_body(serde_json::from_str(&body_text)?);

    // Convert segments if present
    let segments = if let Some(groq_segments) = groq_response.segments {
        groq_segments
            .into_iter()
            .map(|seg| TranscriptSegment::new(seg.text, seg.start, seg.end))
            .collect()
    } else {
        Vec::new()
    };

    Ok(
        TranscriptionModelResponse::new(groq_response.text, response_metadata)
            .with_segments(segments)
            .with_language(groq_response.language.unwrap_or_default())
            .with_duration(groq_response.duration.unwrap_or(0.0))
            .with_warnings(warnings),
    )
}

/// Convert IANA media type to file extension.
fn media_type_to_extension(media_type: &str) -> &str {
    match media_type {
        "audio/mpeg" => "mp3",
        "audio/mp3" => "mp3",
        "audio/wav" => "wav",
        "audio/wave" => "wav",
        "audio/x-wav" => "wav",
        "audio/ogg" => "ogg",
        "audio/flac" => "flac",
        "audio/webm" => "webm",
        "audio/mp4" => "m4a",
        "audio/m4a" => "m4a",
        "audio/x-m4a" => "m4a",
        _ => "bin", // fallback
    }
}

/// Groq API transcription response.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GroqTranscriptionResponse {
    /// The transcribed text.
    text: String,

    /// Groq-specific response metadata.
    #[serde(default)]
    x_groq: Option<GroqMetadata>,

    /// The task type (transcribe or translate).
    #[serde(default)]
    task: Option<String>,

    /// Detected language (ISO-639-1 code).
    #[serde(default)]
    language: Option<String>,

    /// Duration in seconds.
    #[serde(default)]
    duration: Option<f64>,

    /// Transcript segments (only with verbose_json format).
    #[serde(default)]
    segments: Option<Vec<GroqSegment>>,
}

/// Groq-specific metadata in responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GroqMetadata {
    /// Groq request ID.
    id: String,
}

/// A transcript segment from Groq.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GroqSegment {
    /// Segment ID.
    id: i32,

    /// Seek position.
    seek: i32,

    /// Start time in seconds.
    start: f64,

    /// End time in seconds.
    end: f64,

    /// Segment text.
    text: String,

    /// Token IDs.
    #[serde(default)]
    tokens: Vec<i32>,

    /// Temperature used.
    #[serde(default)]
    temperature: f64,

    /// Average log probability.
    #[serde(default)]
    avg_logprob: f64,

    /// Compression ratio.
    #[serde(default)]
    compression_ratio: f64,

    /// No speech probability.
    #[serde(default)]
    no_speech_prob: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_type_to_extension() {
        assert_eq!(media_type_to_extension("audio/mpeg"), "mp3");
        assert_eq!(media_type_to_extension("audio/wav"), "wav");
        assert_eq!(media_type_to_extension("audio/ogg"), "ogg");
        assert_eq!(media_type_to_extension("audio/flac"), "flac");
        assert_eq!(media_type_to_extension("audio/webm"), "webm");
        assert_eq!(media_type_to_extension("audio/mp4"), "m4a");
        assert_eq!(media_type_to_extension("audio/m4a"), "m4a");
        assert_eq!(media_type_to_extension("audio/unknown"), "bin");
    }
}
