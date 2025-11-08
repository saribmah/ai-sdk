use serde::{Deserialize, Serialize};

/// A unique identifier for an OpenAI-compatible embedding model
pub type OpenAICompatibleEmbeddingModelId = String;

/// Provider-specific options for OpenAI-compatible embedding models
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpenAICompatibleEmbeddingProviderOptions {
    /// The number of dimensions the resulting output embeddings should have.
    /// Only supported in text-embedding-3 and later models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<u32>,

    /// A unique identifier representing your end-user, which can help providers to
    /// monitor and detect abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl OpenAICompatibleEmbeddingProviderOptions {
    /// Creates a new empty `OpenAICompatibleEmbeddingProviderOptions`
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the number of dimensions
    pub fn with_dimensions(mut self, dimensions: u32) -> Self {
        self.dimensions = Some(dimensions);
        self
    }

    /// Sets the user identifier
    pub fn with_user(mut self, user: String) -> Self {
        self.user = Some(user);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let options = OpenAICompatibleEmbeddingProviderOptions::new();
        assert_eq!(options.dimensions, None);
        assert_eq!(options.user, None);
    }

    #[test]
    fn test_default() {
        let options = OpenAICompatibleEmbeddingProviderOptions::default();
        assert_eq!(options.dimensions, None);
        assert_eq!(options.user, None);
    }

    #[test]
    fn test_with_dimensions() {
        let options = OpenAICompatibleEmbeddingProviderOptions::new().with_dimensions(256);

        assert_eq!(options.dimensions, Some(256));
        assert_eq!(options.user, None);
    }

    #[test]
    fn test_with_user() {
        let options =
            OpenAICompatibleEmbeddingProviderOptions::new().with_user("user-123".to_string());

        assert_eq!(options.dimensions, None);
        assert_eq!(options.user, Some("user-123".to_string()));
    }

    #[test]
    fn test_builder_chaining() {
        let options = OpenAICompatibleEmbeddingProviderOptions::new()
            .with_dimensions(512)
            .with_user("user-456".to_string());

        assert_eq!(options.dimensions, Some(512));
        assert_eq!(options.user, Some("user-456".to_string()));
    }

    #[test]
    fn test_serialization() {
        let options = OpenAICompatibleEmbeddingProviderOptions::new()
            .with_dimensions(1024)
            .with_user("user-789".to_string());

        let json = serde_json::to_string(&options).unwrap();

        // Check that camelCase is used in JSON
        assert!(json.contains("\"dimensions\""));
        assert!(json.contains("\"user\""));
    }

    #[test]
    fn test_serialization_skip_none() {
        let options = OpenAICompatibleEmbeddingProviderOptions::new().with_dimensions(256);

        let json = serde_json::to_string(&options).unwrap();

        // Check that None fields are skipped
        assert!(json.contains("\"dimensions\""));
        assert!(!json.contains("\"user\""));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{
            "dimensions": 768,
            "user": "user-abc"
        }"#;

        let options: OpenAICompatibleEmbeddingProviderOptions = serde_json::from_str(json).unwrap();

        assert_eq!(options.dimensions, Some(768));
        assert_eq!(options.user, Some("user-abc".to_string()));
    }

    #[test]
    fn test_deserialization_empty() {
        let json = "{}";

        let options: OpenAICompatibleEmbeddingProviderOptions = serde_json::from_str(json).unwrap();

        assert_eq!(options.dimensions, None);
        assert_eq!(options.user, None);
    }

    #[test]
    fn test_clone() {
        let options1 = OpenAICompatibleEmbeddingProviderOptions::new().with_dimensions(1536);

        let options2 = options1.clone();

        assert_eq!(options1, options2);
    }

    #[test]
    fn test_partial_eq() {
        let options1 = OpenAICompatibleEmbeddingProviderOptions::new()
            .with_dimensions(256)
            .with_user("user-1".to_string());

        let options2 = OpenAICompatibleEmbeddingProviderOptions::new()
            .with_dimensions(256)
            .with_user("user-1".to_string());

        let options3 = OpenAICompatibleEmbeddingProviderOptions::new().with_dimensions(512);

        assert_eq!(options1, options2);
        assert_ne!(options1, options3);
    }
}
