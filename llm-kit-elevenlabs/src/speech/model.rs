use super::api_types::{
    ElevenLabsSpeechRequest, PronunciationDictionaryLocator, TextNormalization, VoiceSettings,
};
use super::options::{ElevenLabsSpeechModelId, ElevenLabsSpeechVoiceId};
use super::provider_options::ElevenLabsSpeechProviderOptions;
use crate::config::ElevenLabsConfig;
use crate::error::parse_elevenlabs_error;
use async_trait::async_trait;
use llm_kit_provider::speech_model::{
    AudioData, SpeechModel, SpeechModelRequestMetadata, SpeechModelResponse,
    SpeechModelResponseMetadata, call_options::SpeechModelCallOptions,
    call_warning::SpeechModelCallWarning,
};
use std::collections::HashMap;
use std::time::SystemTime;

/// ElevenLabs speech model implementation.
pub struct ElevenLabsSpeechModel {
    model_id: ElevenLabsSpeechModelId,
    config: ElevenLabsConfig,
}

impl ElevenLabsSpeechModel {
    /// Create a new ElevenLabs speech model.
    pub fn new(model_id: ElevenLabsSpeechModelId, config: ElevenLabsConfig) -> Self {
        Self { model_id, config }
    }

    /// Map output format to ElevenLabs format.
    fn map_output_format(format: &str) -> String {
        let format_map: HashMap<&str, &str> = [
            ("mp3", "mp3_44100_128"),
            ("mp3_32", "mp3_44100_32"),
            ("mp3_64", "mp3_44100_64"),
            ("mp3_96", "mp3_44100_96"),
            ("mp3_128", "mp3_44100_128"),
            ("mp3_192", "mp3_44100_192"),
            ("pcm", "pcm_44100"),
            ("pcm_16000", "pcm_16000"),
            ("pcm_22050", "pcm_22050"),
            ("pcm_24000", "pcm_24000"),
            ("pcm_44100", "pcm_44100"),
            ("ulaw", "ulaw_8000"),
        ]
        .iter()
        .copied()
        .collect();

        format_map.get(format).unwrap_or(&format).to_string()
    }

    /// Build request body from call options.
    fn build_request_body(
        &self,
        options: &SpeechModelCallOptions,
        provider_options: Option<&ElevenLabsSpeechProviderOptions>,
    ) -> (ElevenLabsSpeechRequest, Vec<SpeechModelCallWarning>) {
        let mut warnings = Vec::new();
        let mut request = ElevenLabsSpeechRequest::new(&options.text);

        // Set model ID
        request.model_id = Some(self.model_id.clone());

        // Set language from call options
        if let Some(ref language) = options.language {
            request.language_code = Some(language.clone());
        }

        // Build voice settings
        let mut voice_settings = VoiceSettings {
            stability: None,
            similarity_boost: None,
            style: None,
            use_speaker_boost: None,
            speed: None,
        };

        // Add speed if provided
        if let Some(speed) = options.speed {
            voice_settings.speed = Some(speed as f32);
        }

        // Process provider-specific options
        if let Some(provider_opts) = provider_options {
            // Voice settings
            if let Some(ref opts_voice_settings) = provider_opts.voice_settings {
                voice_settings.stability = opts_voice_settings.stability;
                voice_settings.similarity_boost = opts_voice_settings.similarity_boost;
                voice_settings.style = opts_voice_settings.style;
                voice_settings.use_speaker_boost = opts_voice_settings.use_speaker_boost;
            }

            // Language code from provider options (if not already set)
            if request.language_code.is_none() {
                request.language_code = provider_opts.language_code.clone();
            }

            // Pronunciation dictionaries
            if let Some(ref dictionaries) = provider_opts.pronunciation_dictionary_locators {
                request.pronunciation_dictionary_locators = Some(
                    dictionaries
                        .iter()
                        .map(|d| PronunciationDictionaryLocator {
                            pronunciation_dictionary_id: d.pronunciation_dictionary_id.clone(),
                            version_id: d.version_id.clone(),
                        })
                        .collect(),
                );
            }

            // Other options
            request.seed = provider_opts.seed;
            request.previous_text = provider_opts.previous_text.clone();
            request.next_text = provider_opts.next_text.clone();
            request.previous_request_ids = provider_opts.previous_request_ids.clone();
            request.next_request_ids = provider_opts.next_request_ids.clone();

            // Text normalization
            if let Some(ref norm) = provider_opts.apply_text_normalization {
                request.apply_text_normalization = match norm.as_str() {
                    "auto" => Some(TextNormalization::Auto),
                    "on" => Some(TextNormalization::On),
                    "off" => Some(TextNormalization::Off),
                    _ => None,
                };
            }

            request.apply_language_text_normalization =
                provider_opts.apply_language_text_normalization;
        }

        // Only add voice settings if at least one field is set
        if voice_settings.stability.is_some()
            || voice_settings.similarity_boost.is_some()
            || voice_settings.style.is_some()
            || voice_settings.use_speaker_boost.is_some()
            || voice_settings.speed.is_some()
        {
            request.voice_settings = Some(voice_settings);
        }

        // Warn about unsupported instructions
        if options.instructions.is_some() {
            warnings.push(SpeechModelCallWarning::UnsupportedSetting {
                setting: "instructions".to_string(),
                details: Some(
                    "ElevenLabs speech models do not support instructions parameter".to_string(),
                ),
            });
        }

        (request, warnings)
    }
}

#[async_trait]
impl SpeechModel for ElevenLabsSpeechModel {
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
        let provider_options: Option<ElevenLabsSpeechProviderOptions> =
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

        // Build request body and collect warnings
        let (request_body, warnings) = self.build_request_body(&options, provider_options.as_ref());

        // Get voice ID (default to Rachel)
        let voice_id: ElevenLabsSpeechVoiceId = options
            .voice
            .unwrap_or_else(|| "21m00Tcm4TlvDq8ikWAM".to_string());

        // Build query parameters
        let mut query_params = Vec::new();

        // Add output format
        let output_format = options.output_format.as_deref().unwrap_or("mp3_44100_128");
        query_params.push(("output_format", Self::map_output_format(output_format)));

        // Add enable_logging if specified
        if let Some(ref opts) = provider_options
            && let Some(enable_logging) = opts.enable_logging
        {
            query_params.push(("enable_logging", enable_logging.to_string()));
        }

        // Build URL with query parameters
        let mut url = self.config.url(&format!("/v1/text-to-speech/{}", voice_id));
        if !query_params.is_empty() {
            let query_string = query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&");
            url = format!("{}?{}", url, query_string);
        }

        // Merge headers
        let mut headers = self.config.headers().clone();
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
            let error_message = parse_elevenlabs_error(&error_body)
                .unwrap_or_else(|| format!("HTTP {}: {}", status, error_body));
            return Err(error_message.into());
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
    fn test_map_output_format() {
        assert_eq!(
            ElevenLabsSpeechModel::map_output_format("mp3"),
            "mp3_44100_128"
        );
        assert_eq!(ElevenLabsSpeechModel::map_output_format("pcm"), "pcm_44100");
        assert_eq!(
            ElevenLabsSpeechModel::map_output_format("custom_format"),
            "custom_format"
        );
    }
}
