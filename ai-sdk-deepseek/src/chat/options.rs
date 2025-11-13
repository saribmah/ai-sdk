/// DeepSeek chat model identifier.
///
/// Reference: <https://api-docs.deepseek.com/quick_start/pricing>
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeepSeekChatModelId {
    /// DeepSeek Chat model - Main chat model
    DeepSeekChat,
    /// DeepSeek Reasoner model (R1) - Advanced reasoning model
    DeepSeekReasoner,
    /// Custom model ID for future models
    Custom(String),
}

impl DeepSeekChatModelId {
    /// Returns the string identifier for the model.
    pub fn as_str(&self) -> &str {
        match self {
            Self::DeepSeekChat => "deepseek-chat",
            Self::DeepSeekReasoner => "deepseek-reasoner",
            Self::Custom(id) => id,
        }
    }
}

impl From<&str> for DeepSeekChatModelId {
    fn from(s: &str) -> Self {
        match s {
            "deepseek-chat" => Self::DeepSeekChat,
            "deepseek-reasoner" => Self::DeepSeekReasoner,
            _ => Self::Custom(s.to_string()),
        }
    }
}

impl From<String> for DeepSeekChatModelId {
    fn from(s: String) -> Self {
        DeepSeekChatModelId::from(s.as_str())
    }
}

impl AsRef<str> for DeepSeekChatModelId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// DeepSeek-specific provider options.
///
/// Currently empty as DeepSeek uses standard OpenAI-compatible options,
/// but provided for future extensibility.
#[derive(Debug, Clone, Default)]
pub struct DeepSeekProviderOptions {}

impl DeepSeekProviderOptions {
    /// Creates a new instance of DeepSeek provider options.
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_id_as_str() {
        assert_eq!(DeepSeekChatModelId::DeepSeekChat.as_str(), "deepseek-chat");
        assert_eq!(
            DeepSeekChatModelId::DeepSeekReasoner.as_str(),
            "deepseek-reasoner"
        );
        assert_eq!(
            DeepSeekChatModelId::Custom("custom-model".to_string()).as_str(),
            "custom-model"
        );
    }

    #[test]
    fn test_model_id_from_str() {
        let model_id: DeepSeekChatModelId = "deepseek-chat".into();
        assert_eq!(model_id, DeepSeekChatModelId::DeepSeekChat);

        let model_id: DeepSeekChatModelId = "deepseek-reasoner".into();
        assert_eq!(model_id, DeepSeekChatModelId::DeepSeekReasoner);

        let model_id: DeepSeekChatModelId = "custom-model".into();
        assert_eq!(
            model_id,
            DeepSeekChatModelId::Custom("custom-model".to_string())
        );
    }

    #[test]
    fn test_model_id_from_string() {
        let model_id: DeepSeekChatModelId = "deepseek-chat".to_string().into();
        assert_eq!(model_id, DeepSeekChatModelId::DeepSeekChat);
    }

    #[test]
    fn test_provider_options() {
        let options = DeepSeekProviderOptions::new();
        assert!(std::mem::size_of_val(&options) == 0); // Empty struct
    }
}
