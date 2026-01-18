use serde::{Deserialize, Serialize};

/// Options for Groq text-to-speech generation.
///
/// These options are passed via the `provider_options` field in `GenerateSpeech`.
///
/// # Example
///
/// ```no_run
/// use llm_kit_groq::GroqSpeechOptions;
/// use std::collections::HashMap;
/// use serde_json::Value;
///
/// let groq_options = GroqSpeechOptions::new()
///     .with_sample_rate(48000);
///
/// let mut provider_options: HashMap<String, HashMap<String, Value>> = HashMap::new();
/// let mut groq_opts_map = HashMap::new();
/// let groq_value = serde_json::to_value(&groq_options).unwrap();
/// if let Value::Object(map) = groq_value {
///     for (k, v) in map {
///         groq_opts_map.insert(k, v);
///     }
/// }
/// provider_options.insert("groq".to_string(), groq_opts_map);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GroqSpeechOptions {
    /// The sample rate for generated audio.
    ///
    /// Allowed values: 8000, 16000, 22050, 24000, 32000, 44100, 48000
    ///
    /// Default: 48000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<u32>,
}

impl GroqSpeechOptions {
    /// Create a new instance with default settings.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_groq::GroqSpeechOptions;
    ///
    /// let options = GroqSpeechOptions::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the sample rate for generated audio.
    ///
    /// # Arguments
    ///
    /// * `sample_rate` - Sample rate in Hz (8000, 16000, 22050, 24000, 32000, 44100, 48000)
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_groq::GroqSpeechOptions;
    ///
    /// let options = GroqSpeechOptions::new()
    ///     .with_sample_rate(24000);
    /// ```
    pub fn with_sample_rate(mut self, sample_rate: u32) -> Self {
        self.sample_rate = Some(sample_rate);
        self
    }

    /// Validate the options.
    pub(crate) fn validate(&self) -> Result<(), String> {
        // Validate sample rate
        if let Some(rate) = self.sample_rate {
            let valid_rates = [8000, 16000, 22050, 24000, 32000, 44100, 48000];
            if !valid_rates.contains(&rate) {
                return Err(format!(
                    "Invalid sample_rate: {}. Must be one of: {:?}",
                    rate, valid_rates
                ));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let options = GroqSpeechOptions::default();
        assert!(options.sample_rate.is_none());
    }

    #[test]
    fn test_with_sample_rate() {
        let options = GroqSpeechOptions::new().with_sample_rate(24000);
        assert_eq!(options.sample_rate, Some(24000));
    }

    #[test]
    fn test_validate_valid() {
        let options = GroqSpeechOptions::new().with_sample_rate(48000);
        assert!(options.validate().is_ok());
    }

    #[test]
    fn test_validate_invalid_sample_rate() {
        let options = GroqSpeechOptions::new().with_sample_rate(12345);
        assert!(options.validate().is_err());
    }

    #[test]
    fn test_serialize() {
        let options = GroqSpeechOptions::new().with_sample_rate(24000);
        let json = serde_json::to_value(&options).unwrap();
        assert_eq!(json["sample_rate"], 24000);
    }
}
