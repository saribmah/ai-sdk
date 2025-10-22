use crate::error::AISDKError;
use std::collections::HashMap;
use tokio_util::sync::CancellationToken;

/// Prepared call settings after validation.
///
/// This is a subset of `CallSettings` that excludes `abort_signal`, `headers`, and `max_retries`,
/// which are handled separately during the actual API call.
#[derive(Debug, Clone, Default)]
pub struct PreparedCallSettings {
    pub max_output_tokens: Option<u32>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub top_k: Option<u32>,
    pub presence_penalty: Option<f64>,
    pub frequency_penalty: Option<f64>,
    pub stop_sequences: Option<Vec<String>>,
    pub seed: Option<u32>,
}

/// Settings for language model calls.
///
/// This type provides configuration options for text generation, including
/// sampling parameters, token limits, retries, and HTTP headers.
#[derive(Debug, Clone, Default)]
pub struct CallSettings {
    /// Maximum number of tokens to generate.
    pub max_output_tokens: Option<u32>,

    /// Temperature setting. The range depends on the provider and model.
    ///
    /// It is recommended to set either `temperature` or `top_p`, but not both.
    pub temperature: Option<f64>,

    /// Nucleus sampling. This is a number between 0 and 1.
    ///
    /// E.g. 0.1 would mean that only tokens with the top 10% probability mass
    /// are considered.
    ///
    /// It is recommended to set either `temperature` or `top_p`, but not both.
    pub top_p: Option<f64>,

    /// Only sample from the top K options for each subsequent token.
    ///
    /// Used to remove "long tail" low probability responses.
    /// Recommended for advanced use cases only. You usually only need to use temperature.
    pub top_k: Option<u32>,

    /// Presence penalty setting. It affects the likelihood of the model to
    /// repeat information that is already in the prompt.
    ///
    /// The presence penalty is a number between -1 (increase repetition)
    /// and 1 (maximum penalty, decrease repetition). 0 means no penalty.
    pub presence_penalty: Option<f64>,

    /// Frequency penalty setting. It affects the likelihood of the model
    /// to repeatedly use the same words or phrases.
    ///
    /// The frequency penalty is a number between -1 (increase repetition)
    /// and 1 (maximum penalty, decrease repetition). 0 means no penalty.
    pub frequency_penalty: Option<f64>,

    /// Stop sequences.
    /// If set, the model will stop generating text when one of the stop sequences is generated.
    /// Providers may have limits on the number of stop sequences.
    pub stop_sequences: Option<Vec<String>>,

    /// The seed (integer) to use for random sampling. If set and supported
    /// by the model, calls will generate deterministic results.
    pub seed: Option<u32>,

    /// Maximum number of retries. Set to 0 to disable retries.
    ///
    /// Default: 2
    pub max_retries: Option<u32>,

    /// Abort signal for cancelling the request.
    pub abort_signal: Option<CancellationToken>,

    /// Additional HTTP headers to be sent with the request.
    /// Only applicable for HTTP-based providers.
    pub headers: Option<HashMap<String, String>>,
}

impl CallSettings {
    /// Creates a new `CallSettings` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum number of output tokens.
    pub fn with_max_output_tokens(mut self, max_output_tokens: u32) -> Self {
        self.max_output_tokens = Some(max_output_tokens);
        self
    }

    /// Sets the temperature.
    pub fn with_temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Sets the top_p (nucleus sampling).
    pub fn with_top_p(mut self, top_p: f64) -> Self {
        self.top_p = Some(top_p);
        self
    }

    /// Sets the top_k.
    pub fn with_top_k(mut self, top_k: u32) -> Self {
        self.top_k = Some(top_k);
        self
    }

    /// Sets the presence penalty.
    pub fn with_presence_penalty(mut self, presence_penalty: f64) -> Self {
        self.presence_penalty = Some(presence_penalty);
        self
    }

    /// Sets the frequency penalty.
    pub fn with_frequency_penalty(mut self, frequency_penalty: f64) -> Self {
        self.frequency_penalty = Some(frequency_penalty);
        self
    }

    /// Sets the stop sequences.
    pub fn with_stop_sequences(mut self, stop_sequences: Vec<String>) -> Self {
        self.stop_sequences = Some(stop_sequences);
        self
    }

    /// Sets a single stop sequence.
    pub fn with_stop_sequence(mut self, stop_sequence: String) -> Self {
        let mut sequences = self.stop_sequences.unwrap_or_default();
        sequences.push(stop_sequence);
        self.stop_sequences = Some(sequences);
        self
    }

    /// Sets the seed for deterministic generation.
    pub fn with_seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Sets the maximum number of retries.
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    /// Sets the abort signal.
    pub fn with_abort_signal(mut self, abort_signal: CancellationToken) -> Self {
        self.abort_signal = Some(abort_signal);
        self
    }

    /// Sets the HTTP headers.
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Adds a single HTTP header.
    pub fn with_header(mut self, key: String, value: String) -> Self {
        let mut headers = self.headers.unwrap_or_default();
        headers.insert(key, value);
        self.headers = Some(headers);
        self
    }
}

/// Validates call settings and returns a prepared settings object.
///
/// This function validates the provided settings and returns a `PreparedCallSettings`
/// object that excludes `abort_signal`, `headers`, and `max_retries`.
///
/// # Arguments
/// * `settings` - The call settings to validate
///
/// # Returns
/// Returns a `PreparedCallSettings` with validated values.
///
/// # Errors
/// Returns an error if:
/// - `max_output_tokens` is less than 1
/// - Any numeric value is not finite (NaN or infinity)
pub fn prepare_call_settings(settings: &CallSettings) -> Result<PreparedCallSettings, AISDKError> {
    // Validate max_output_tokens
    if let Some(max_tokens) = settings.max_output_tokens {
        if max_tokens < 1 {
            return Err(AISDKError::invalid_argument(
                "maxOutputTokens",
                max_tokens,
                "maxOutputTokens must be >= 1",
            ));
        }
    }

    // Validate temperature (must be finite)
    if let Some(temp) = settings.temperature {
        if !temp.is_finite() {
            return Err(AISDKError::invalid_argument(
                "temperature",
                temp,
                "temperature must be a finite number",
            ));
        }
    }

    // Validate top_p (must be finite)
    if let Some(top_p) = settings.top_p {
        if !top_p.is_finite() {
            return Err(AISDKError::invalid_argument(
                "topP",
                top_p,
                "topP must be a finite number",
            ));
        }
    }

    // Validate presence_penalty (must be finite)
    if let Some(penalty) = settings.presence_penalty {
        if !penalty.is_finite() {
            return Err(AISDKError::invalid_argument(
                "presencePenalty",
                penalty,
                "presencePenalty must be a finite number",
            ));
        }
    }

    // Validate frequency_penalty (must be finite)
    if let Some(penalty) = settings.frequency_penalty {
        if !penalty.is_finite() {
            return Err(AISDKError::invalid_argument(
                "frequencyPenalty",
                penalty,
                "frequencyPenalty must be a finite number",
            ));
        }
    }

    // Note: Rust's u32 type ensures top_k and seed are already valid integers,
    // so no additional validation needed for those fields.

    Ok(PreparedCallSettings {
        max_output_tokens: settings.max_output_tokens,
        temperature: settings.temperature,
        top_p: settings.top_p,
        top_k: settings.top_k,
        presence_penalty: settings.presence_penalty,
        frequency_penalty: settings.frequency_penalty,
        stop_sequences: settings.stop_sequences.clone(),
        seed: settings.seed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_settings_default() {
        let settings = CallSettings::new();
        assert!(settings.max_output_tokens.is_none());
        assert!(settings.temperature.is_none());
        assert!(settings.max_retries.is_none());
    }

    #[test]
    fn test_call_settings_builder() {
        let settings = CallSettings::new()
            .with_max_output_tokens(1000)
            .with_temperature(0.7)
            .with_top_p(0.9)
            .with_max_retries(3);

        assert_eq!(settings.max_output_tokens, Some(1000));
        assert_eq!(settings.temperature, Some(0.7));
        assert_eq!(settings.top_p, Some(0.9));
        assert_eq!(settings.max_retries, Some(3));
    }

    #[test]
    fn test_call_settings_stop_sequences() {
        let settings = CallSettings::new()
            .with_stop_sequence("STOP".to_string())
            .with_stop_sequence("END".to_string());

        assert_eq!(
            settings.stop_sequences,
            Some(vec!["STOP".to_string(), "END".to_string()])
        );
    }

    #[test]
    fn test_call_settings_headers() {
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer token".to_string());

        let settings = CallSettings::new()
            .with_headers(headers)
            .with_header("Custom-Header".to_string(), "value".to_string());

        assert!(settings.headers.is_some());
        let headers = settings.headers.unwrap();
        assert_eq!(headers.get("Authorization"), Some(&"Bearer token".to_string()));
        assert_eq!(headers.get("Custom-Header"), Some(&"value".to_string()));
    }

    #[test]
    fn test_prepare_call_settings_valid() {
        let settings = CallSettings::new()
            .with_max_output_tokens(1000)
            .with_temperature(0.7)
            .with_top_p(0.9)
            .with_max_retries(3)
            .with_header("test".to_string(), "value".to_string());

        let prepared = prepare_call_settings(&settings).unwrap();

        assert_eq!(prepared.max_output_tokens, Some(1000));
        assert_eq!(prepared.temperature, Some(0.7));
        assert_eq!(prepared.top_p, Some(0.9));
        // Note: max_retries and headers are excluded from PreparedCallSettings
    }

    #[test]
    fn test_prepare_call_settings_invalid_max_output_tokens() {
        let settings = CallSettings::new().with_max_output_tokens(0);

        let result = prepare_call_settings(&settings);
        assert!(result.is_err());

        let err = result.unwrap_err();
        match err {
            AISDKError::InvalidArgument { parameter, .. } => {
                assert_eq!(parameter, "maxOutputTokens");
            }
        }
    }

    #[test]
    fn test_prepare_call_settings_invalid_temperature_nan() {
        let settings = CallSettings::new().with_temperature(f64::NAN);

        let result = prepare_call_settings(&settings);
        assert!(result.is_err());

        let err = result.unwrap_err();
        match err {
            AISDKError::InvalidArgument { parameter, .. } => {
                assert_eq!(parameter, "temperature");
            }
        }
    }

    #[test]
    fn test_prepare_call_settings_invalid_temperature_infinity() {
        let settings = CallSettings::new().with_temperature(f64::INFINITY);

        let result = prepare_call_settings(&settings);
        assert!(result.is_err());

        let err = result.unwrap_err();
        match err {
            AISDKError::InvalidArgument { parameter, .. } => {
                assert_eq!(parameter, "temperature");
            }
        }
    }

    #[test]
    fn test_prepare_call_settings_invalid_top_p() {
        let settings = CallSettings::new().with_top_p(f64::NAN);

        let result = prepare_call_settings(&settings);
        assert!(result.is_err());

        let err = result.unwrap_err();
        match err {
            AISDKError::InvalidArgument { parameter, .. } => {
                assert_eq!(parameter, "topP");
            }
        }
    }

    #[test]
    fn test_prepare_call_settings_invalid_presence_penalty() {
        let settings = CallSettings::new().with_presence_penalty(f64::INFINITY);

        let result = prepare_call_settings(&settings);
        assert!(result.is_err());

        let err = result.unwrap_err();
        match err {
            AISDKError::InvalidArgument { parameter, .. } => {
                assert_eq!(parameter, "presencePenalty");
            }
        }
    }

    #[test]
    fn test_prepare_call_settings_invalid_frequency_penalty() {
        let settings = CallSettings::new().with_frequency_penalty(f64::NAN);

        let result = prepare_call_settings(&settings);
        assert!(result.is_err());

        let err = result.unwrap_err();
        match err {
            AISDKError::InvalidArgument { parameter, .. } => {
                assert_eq!(parameter, "frequencyPenalty");
            }
        }
    }

    #[test]
    fn test_prepare_call_settings_all_fields() {
        let settings = CallSettings::new()
            .with_max_output_tokens(500)
            .with_temperature(0.8)
            .with_top_p(0.95)
            .with_top_k(40)
            .with_presence_penalty(0.5)
            .with_frequency_penalty(0.3)
            .with_stop_sequence("END".to_string())
            .with_seed(42);

        let prepared = prepare_call_settings(&settings).unwrap();

        assert_eq!(prepared.max_output_tokens, Some(500));
        assert_eq!(prepared.temperature, Some(0.8));
        assert_eq!(prepared.top_p, Some(0.95));
        assert_eq!(prepared.top_k, Some(40));
        assert_eq!(prepared.presence_penalty, Some(0.5));
        assert_eq!(prepared.frequency_penalty, Some(0.3));
        assert_eq!(prepared.seed, Some(42));
        assert_eq!(prepared.stop_sequences, Some(vec!["END".to_string()]));
    }
}
