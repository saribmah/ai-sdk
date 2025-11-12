use super::api_types::{ElevenLabsTranscriptionResponse, WordType};
use super::options::ElevenLabsTranscriptionModelId;
use super::provider_options::ElevenLabsTranscriptionProviderOptions;
use crate::config::ElevenLabsConfig;
use crate::error::parse_elevenlabs_error;
use ai_sdk_provider::transcription_model::{
    TranscriptSegment, TranscriptionModel, TranscriptionModelResponse,
    TranscriptionModelResponseMetadata, call_options::TranscriptionModelCallOptions,
};
use async_trait::async_trait;
use reqwest::multipart::{Form, Part};
use std::collections::HashMap;
use std::time::SystemTime;

/// ElevenLabs transcription model implementation.
pub struct ElevenLabsTranscriptionModel {
    model_id: ElevenLabsTranscriptionModelId,
    config: ElevenLabsConfig,
}

impl ElevenLabsTranscriptionModel {
    /// Create a new ElevenLabs transcription model.
    pub fn new(model_id: ElevenLabsTranscriptionModelId, config: ElevenLabsConfig) -> Self {
        Self { model_id, config }
    }

    /// Get file extension from media type.
    fn media_type_to_extension(media_type: &str) -> &str {
        match media_type {
            "audio/mpeg" => "mp3",
            "audio/mp3" => "mp3",
            "audio/wav" => "wav",
            "audio/wave" => "wav",
            "audio/x-wav" => "wav",
            "audio/ogg" => "ogg",
            "audio/flac" => "flac",
            "audio/x-flac" => "flac",
            "audio/mp4" => "m4a",
            "audio/x-m4a" => "m4a",
            "audio/webm" => "webm",
            _ => "audio",
        }
    }

    /// Convert audio data to bytes.
    fn audio_to_bytes(
        audio: &ai_sdk_provider::transcription_model::call_options::TranscriptionAudioData,
    ) -> Result<Vec<u8>, String> {
        match audio {
            ai_sdk_provider::transcription_model::call_options::TranscriptionAudioData::Binary(
                bytes,
            ) => Ok(bytes.clone()),
            ai_sdk_provider::transcription_model::call_options::TranscriptionAudioData::Base64(
                base64_str,
            ) => {
                // Decode base64
                use base64::{Engine as _, engine::general_purpose};
                general_purpose::STANDARD
                    .decode(base64_str)
                    .map_err(|e| format!("Failed to decode base64 audio: {}", e))
            }
        }
    }
}

#[async_trait]
impl TranscriptionModel for ElevenLabsTranscriptionModel {
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
        options: TranscriptionModelCallOptions,
    ) -> Result<TranscriptionModelResponse, Box<dyn std::error::Error>> {
        let timestamp = SystemTime::now();

        // Parse provider options
        let provider_options: Option<ElevenLabsTranscriptionProviderOptions> =
            if let Some(ref provider_opts) = options.provider_options {
                if let Some(elevenlabs_opts) = provider_opts.get("elevenlabs") {
                    Some(serde_json::from_value(serde_json::to_value(
                        elevenlabs_opts,
                    )?)?)
                } else {
                    None
                }
            } else {
                None
            };

        // Validate provider options
        if let Some(ref opts) = provider_options {
            opts.validate()?;
        }

        // Convert audio to bytes
        let audio_bytes = Self::audio_to_bytes(&options.audio)?;

        // Build multipart form
        let file_extension = Self::media_type_to_extension(&options.media_type);
        let filename = format!("audio.{}", file_extension);

        let audio_part = Part::bytes(audio_bytes)
            .file_name(filename)
            .mime_str(&options.media_type)?;

        let mut form = Form::new()
            .text("model_id", self.model_id.clone())
            .part("file", audio_part);

        // Add provider-specific options
        if let Some(ref opts) = provider_options {
            // Diarize (always include, default to false)
            let diarize = opts.diarize.unwrap_or(false);
            form = form.text("diarize", diarize.to_string());

            if let Some(ref language_code) = opts.language_code {
                form = form.text("language_code", language_code.clone());
            }

            if let Some(tag_audio_events) = opts.tag_audio_events {
                form = form.text("tag_audio_events", tag_audio_events.to_string());
            }

            if let Some(num_speakers) = opts.num_speakers {
                form = form.text("num_speakers", num_speakers.to_string());
            }

            if let Some(ref granularity) = opts.timestamps_granularity {
                form = form.text("timestamps_granularity", granularity.clone());
            }

            if let Some(ref file_format) = opts.file_format {
                form = form.text("file_format", file_format.clone());
            }
        } else {
            // Default diarize to true (as per reference implementation)
            form = form.text("diarize", "true");
        }

        // Build URL
        let url = self.config.url("/v1/speech-to-text");

        // Merge headers
        let mut headers = self.config.headers().clone();
        if let Some(ref opts_headers) = options.headers {
            headers.extend(opts_headers.clone());
        }

        // Make the API request
        let client = reqwest::Client::new();
        let mut request_builder = client.post(&url).multipart(form);

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
            let error_message = parse_elevenlabs_error(&error_body)
                .unwrap_or_else(|| format!("HTTP {}: {}", status, error_body));
            return Err(error_message.into());
        }

        // Parse response
        let response_text = response.text().await?;
        let api_response: ElevenLabsTranscriptionResponse = serde_json::from_str(&response_text)?;

        // Convert words to segments (filter out spacing and audio events)
        let segments: Vec<TranscriptSegment> = api_response
            .words
            .as_ref()
            .map(|words| {
                words
                    .iter()
                    .filter(|w| matches!(w.word_type, WordType::Word))
                    .map(|word| TranscriptSegment {
                        text: word.text.clone(),
                        start_second: word.start.unwrap_or(0.0),
                        end_second: word.end.unwrap_or(0.0),
                    })
                    .collect()
            })
            .unwrap_or_default();

        // Get duration from last word's end time
        let duration = api_response
            .words
            .as_ref()
            .and_then(|words| words.last())
            .and_then(|word| word.end);

        // Build response metadata
        let response_metadata = TranscriptionModelResponseMetadata {
            timestamp,
            model_id: self.model_id.clone(),
            headers: Some(response_headers),
            body: Some(serde_json::from_str(&response_text)?),
        };

        let mut transcription_response =
            TranscriptionModelResponse::new(api_response.text, response_metadata)
                .with_segments(segments)
                .with_language(api_response.language_code);

        if let Some(duration) = duration {
            transcription_response = transcription_response.with_duration(duration);
        }

        Ok(transcription_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_type_to_extension() {
        assert_eq!(
            ElevenLabsTranscriptionModel::media_type_to_extension("audio/mpeg"),
            "mp3"
        );
        assert_eq!(
            ElevenLabsTranscriptionModel::media_type_to_extension("audio/wav"),
            "wav"
        );
        assert_eq!(
            ElevenLabsTranscriptionModel::media_type_to_extension("audio/unknown"),
            "audio"
        );
    }
}
