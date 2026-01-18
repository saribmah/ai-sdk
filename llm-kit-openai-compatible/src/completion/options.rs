use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type alias for OpenAI-compatible completion model identifiers
pub type OpenAICompatibleCompletionModelId = String;

/// OpenAI-compatible provider options for text completions.
///
/// These options can be passed to configure provider-specific behavior
/// for OpenAI-compatible completion models.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpenAICompatibleCompletionProviderOptions {
    /// Echo back the prompt in addition to the completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<bool>,

    /// Modify the likelihood of specified tokens appearing in the completion.
    ///
    /// Accepts a map that maps tokens (specified by their token ID in
    /// the GPT tokenizer) to an associated bias value from -100 to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, f64>>,

    /// The suffix that comes after a completion of inserted text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,

    /// A unique identifier representing your end-user, which can help providers
    /// to monitor and detect abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl OpenAICompatibleCompletionProviderOptions {
    /// Creates a new empty `OpenAICompatibleCompletionProviderOptions`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether to echo back the prompt.
    pub fn with_echo(mut self, echo: bool) -> Self {
        self.echo = Some(echo);
        self
    }

    /// Sets the logit bias map.
    pub fn with_logit_bias(mut self, logit_bias: HashMap<String, f64>) -> Self {
        self.logit_bias = Some(logit_bias);
        self
    }

    /// Sets the suffix.
    pub fn with_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    /// Sets the user identifier.
    pub fn with_user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_default() {
        let options = OpenAICompatibleCompletionProviderOptions::default();
        assert_eq!(options.echo, None);
        assert_eq!(options.logit_bias, None);
        assert_eq!(options.suffix, None);
        assert_eq!(options.user, None);
    }

    #[test]
    fn test_new() {
        let options = OpenAICompatibleCompletionProviderOptions::new();
        assert_eq!(options.echo, None);
        assert_eq!(options.logit_bias, None);
        assert_eq!(options.suffix, None);
        assert_eq!(options.user, None);
    }

    #[test]
    fn test_with_echo() {
        let options = OpenAICompatibleCompletionProviderOptions::new().with_echo(true);

        assert_eq!(options.echo, Some(true));
        assert_eq!(options.logit_bias, None);
        assert_eq!(options.suffix, None);
        assert_eq!(options.user, None);
    }

    #[test]
    fn test_with_logit_bias() {
        let mut bias_map = HashMap::new();
        bias_map.insert("50256".to_string(), -100.0);
        bias_map.insert("198".to_string(), 50.0);

        let options =
            OpenAICompatibleCompletionProviderOptions::new().with_logit_bias(bias_map.clone());

        assert_eq!(options.echo, None);
        assert_eq!(options.logit_bias, Some(bias_map));
        assert_eq!(options.suffix, None);
        assert_eq!(options.user, None);
    }

    #[test]
    fn test_with_suffix() {
        let options =
            OpenAICompatibleCompletionProviderOptions::new().with_suffix("\n\nEnd of text");

        assert_eq!(options.echo, None);
        assert_eq!(options.logit_bias, None);
        assert_eq!(options.suffix, Some("\n\nEnd of text".to_string()));
        assert_eq!(options.user, None);
    }

    #[test]
    fn test_with_user() {
        let options = OpenAICompatibleCompletionProviderOptions::new().with_user("user-123");

        assert_eq!(options.echo, None);
        assert_eq!(options.logit_bias, None);
        assert_eq!(options.suffix, None);
        assert_eq!(options.user, Some("user-123".to_string()));
    }

    #[test]
    fn test_builder_pattern() {
        let mut bias_map = HashMap::new();
        bias_map.insert("100".to_string(), 25.5);

        let options = OpenAICompatibleCompletionProviderOptions::new()
            .with_echo(false)
            .with_logit_bias(bias_map.clone())
            .with_suffix("...")
            .with_user("user-456");

        assert_eq!(options.echo, Some(false));
        assert_eq!(options.logit_bias, Some(bias_map));
        assert_eq!(options.suffix, Some("...".to_string()));
        assert_eq!(options.user, Some("user-456".to_string()));
    }

    #[test]
    fn test_serialize_all_fields() {
        let mut bias_map = HashMap::new();
        bias_map.insert("123".to_string(), 75.0);

        let options = OpenAICompatibleCompletionProviderOptions {
            echo: Some(true),
            logit_bias: Some(bias_map),
            suffix: Some("END".to_string()),
            user: Some("user-789".to_string()),
        };

        let json = serde_json::to_value(&options).unwrap();
        assert_eq!(json["echo"], true);
        assert_eq!(json["logitBias"]["123"], 75.0);
        assert_eq!(json["suffix"], "END");
        assert_eq!(json["user"], "user-789");
    }

    #[test]
    fn test_serialize_empty() {
        let options = OpenAICompatibleCompletionProviderOptions::default();
        let json = serde_json::to_value(&options).unwrap();

        // Empty fields should not be serialized
        assert_eq!(json, json!({}));
    }

    #[test]
    fn test_serialize_partial() {
        let options = OpenAICompatibleCompletionProviderOptions {
            echo: Some(false),
            logit_bias: None,
            suffix: Some("DONE".to_string()),
            user: None,
        };

        let json = serde_json::to_value(&options).unwrap();
        assert_eq!(json["echo"], false);
        assert_eq!(json["suffix"], "DONE");
        assert!(json.get("logitBias").is_none());
        assert!(json.get("user").is_none());
    }

    #[test]
    fn test_deserialize_all_fields() {
        let json = json!({
            "echo": true,
            "logitBias": {
                "50256": -100.0,
                "198": 50.0
            },
            "suffix": "...",
            "user": "user-xyz"
        });

        let options: OpenAICompatibleCompletionProviderOptions =
            serde_json::from_value(json).unwrap();
        assert_eq!(options.echo, Some(true));

        let bias_map = options.logit_bias.unwrap();
        assert_eq!(bias_map.get("50256"), Some(&-100.0));
        assert_eq!(bias_map.get("198"), Some(&50.0));

        assert_eq!(options.suffix, Some("...".to_string()));
        assert_eq!(options.user, Some("user-xyz".to_string()));
    }

    #[test]
    fn test_deserialize_empty() {
        let json = json!({});
        let options: OpenAICompatibleCompletionProviderOptions =
            serde_json::from_value(json).unwrap();

        assert_eq!(options.echo, None);
        assert_eq!(options.logit_bias, None);
        assert_eq!(options.suffix, None);
        assert_eq!(options.user, None);
    }

    #[test]
    fn test_deserialize_partial() {
        let json = json!({
            "echo": false,
            "suffix": "PARTIAL"
        });

        let options: OpenAICompatibleCompletionProviderOptions =
            serde_json::from_value(json).unwrap();
        assert_eq!(options.echo, Some(false));
        assert_eq!(options.logit_bias, None);
        assert_eq!(options.suffix, Some("PARTIAL".to_string()));
        assert_eq!(options.user, None);
    }

    #[test]
    fn test_logit_bias_range() {
        let mut bias_map = HashMap::new();
        bias_map.insert("min".to_string(), -100.0);
        bias_map.insert("max".to_string(), 100.0);
        bias_map.insert("mid".to_string(), 0.0);

        let options =
            OpenAICompatibleCompletionProviderOptions::new().with_logit_bias(bias_map.clone());

        assert_eq!(options.logit_bias, Some(bias_map));
    }

    #[test]
    fn test_clone() {
        let options1 = OpenAICompatibleCompletionProviderOptions::new()
            .with_echo(true)
            .with_user("clone-user");
        let options2 = options1.clone();

        assert_eq!(options1, options2);
        assert_eq!(options2.echo, Some(true));
        assert_eq!(options2.user, Some("clone-user".to_string()));
    }

    #[test]
    fn test_debug() {
        let options = OpenAICompatibleCompletionProviderOptions::new().with_user("debug-user");

        let debug_str = format!("{:?}", options);
        assert!(debug_str.contains("user"));
        assert!(debug_str.contains("debug-user"));
    }

    #[test]
    fn test_echo_false() {
        let options = OpenAICompatibleCompletionProviderOptions::new().with_echo(false);

        assert_eq!(options.echo, Some(false));
    }

    #[test]
    fn test_empty_logit_bias() {
        let bias_map = HashMap::new();
        let options =
            OpenAICompatibleCompletionProviderOptions::new().with_logit_bias(bias_map.clone());

        assert_eq!(options.logit_bias, Some(bias_map));
    }
}
