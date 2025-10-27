use ai_sdk_provider::shared::provider_metadata::ProviderMetadata;
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;

/// Extracts provider-specific metadata from API responses.
/// Used to standardize metadata handling across different LLM providers while allowing
/// provider-specific metadata to be captured.
pub trait MetadataExtractor: Send + Sync {
    /// Extracts provider metadata from a complete, non-streaming response.
    ///
    /// # Arguments
    ///
    /// * `parsed_body` - The parsed response JSON body from the provider's API.
    ///
    /// # Returns
    ///
    /// Provider-specific metadata or None if no metadata is available.
    /// The metadata should be under a key indicating the provider id.
    fn extract_metadata(
        &self,
        parsed_body: Value,
    ) -> Pin<Box<dyn Future<Output = Option<ProviderMetadata>> + Send + '_>>;

    /// Creates an extractor for handling streaming responses. The returned object provides
    /// methods to process individual chunks and build the final metadata from the accumulated
    /// stream data.
    ///
    /// # Returns
    ///
    /// A stream extractor that can process chunks and build metadata from a stream
    fn create_stream_extractor(&self) -> Box<dyn StreamMetadataExtractor>;
}

/// A stream metadata extractor that processes chunks and builds final metadata
pub trait StreamMetadataExtractor: Send {
    /// Process an individual chunk from the stream. Called for each chunk in the response stream
    /// to accumulate metadata throughout the streaming process.
    ///
    /// # Arguments
    ///
    /// * `parsed_chunk` - The parsed JSON response chunk from the provider's API
    fn process_chunk(&mut self, parsed_chunk: Value);

    /// Builds the metadata object after all chunks have been processed.
    /// Called at the end of the stream to generate the complete provider metadata.
    ///
    /// # Returns
    ///
    /// Provider-specific metadata or None if no metadata is available.
    /// The metadata should be under a key indicating the provider id.
    fn build_metadata(&self) -> Option<ProviderMetadata>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::HashMap;

    // Test implementation of MetadataExtractor
    struct TestMetadataExtractor;

    impl MetadataExtractor for TestMetadataExtractor {
        fn extract_metadata(
            &self,
            parsed_body: Value,
        ) -> Pin<Box<dyn Future<Output = Option<ProviderMetadata>> + Send + '_>> {
            Box::pin(async move {
                if let Some(id) = parsed_body.get("id").and_then(|v| v.as_str()) {
                    let mut metadata = HashMap::new();
                    let mut provider_data = HashMap::new();
                    provider_data.insert("id".to_string(), json!(id));
                    metadata.insert("test-provider".to_string(), provider_data);
                    Some(metadata)
                } else {
                    None
                }
            })
        }

        fn create_stream_extractor(&self) -> Box<dyn StreamMetadataExtractor> {
            Box::new(TestStreamExtractor::new())
        }
    }

    // Test implementation of StreamMetadataExtractor
    struct TestStreamExtractor {
        ids: Vec<String>,
    }

    impl TestStreamExtractor {
        fn new() -> Self {
            Self { ids: Vec::new() }
        }
    }

    impl StreamMetadataExtractor for TestStreamExtractor {
        fn process_chunk(&mut self, parsed_chunk: Value) {
            if let Some(id) = parsed_chunk.get("id").and_then(|v| v.as_str()) {
                self.ids.push(id.to_string());
            }
        }

        fn build_metadata(&self) -> Option<ProviderMetadata> {
            if self.ids.is_empty() {
                None
            } else {
                let mut metadata = HashMap::new();
                let mut provider_data = HashMap::new();
                provider_data.insert("chunks".to_string(), json!(self.ids.len()));
                provider_data.insert("ids".to_string(), json!(self.ids));
                metadata.insert("test-provider".to_string(), provider_data);
                Some(metadata)
            }
        }
    }

    #[tokio::test]
    async fn test_extract_metadata_with_data() {
        let extractor = TestMetadataExtractor;
        let body = json!({
            "id": "test-123",
            "model": "gpt-4"
        });

        let result = extractor.extract_metadata(body).await;

        assert!(result.is_some());
        let metadata = result.unwrap();
        assert!(metadata.contains_key("test-provider"));

        let provider_data = &metadata["test-provider"];
        assert_eq!(provider_data.get("id"), Some(&json!("test-123")));
    }

    #[tokio::test]
    async fn test_extract_metadata_without_id() {
        let extractor = TestMetadataExtractor;
        let body = json!({
            "model": "gpt-4"
        });

        let result = extractor.extract_metadata(body).await;

        assert!(result.is_none());
    }

    #[test]
    fn test_stream_extractor_single_chunk() {
        let extractor = TestMetadataExtractor;
        let mut stream_extractor = extractor.create_stream_extractor();

        stream_extractor.process_chunk(json!({
            "id": "chunk-1"
        }));

        let metadata = stream_extractor.build_metadata();

        assert!(metadata.is_some());
        let metadata = metadata.unwrap();
        assert!(metadata.contains_key("test-provider"));

        let provider_data = &metadata["test-provider"];
        assert_eq!(provider_data.get("chunks"), Some(&json!(1)));
    }

    #[test]
    fn test_stream_extractor_multiple_chunks() {
        let extractor = TestMetadataExtractor;
        let mut stream_extractor = extractor.create_stream_extractor();

        stream_extractor.process_chunk(json!({"id": "chunk-1"}));
        stream_extractor.process_chunk(json!({"id": "chunk-2"}));
        stream_extractor.process_chunk(json!({"id": "chunk-3"}));

        let metadata = stream_extractor.build_metadata();

        assert!(metadata.is_some());
        let metadata = metadata.unwrap();
        let provider_data = &metadata["test-provider"];

        assert_eq!(provider_data.get("chunks"), Some(&json!(3)));
        assert_eq!(
            provider_data.get("ids"),
            Some(&json!(["chunk-1", "chunk-2", "chunk-3"]))
        );
    }

    #[test]
    fn test_stream_extractor_no_chunks() {
        let extractor = TestMetadataExtractor;
        let stream_extractor = extractor.create_stream_extractor();

        let metadata = stream_extractor.build_metadata();

        assert!(metadata.is_none());
    }

    #[test]
    fn test_stream_extractor_chunks_without_id() {
        let extractor = TestMetadataExtractor;
        let mut stream_extractor = extractor.create_stream_extractor();

        stream_extractor.process_chunk(json!({"model": "gpt-4"}));
        stream_extractor.process_chunk(json!({"other": "data"}));

        let metadata = stream_extractor.build_metadata();

        assert!(metadata.is_none());
    }

    #[test]
    fn test_stream_extractor_mixed_chunks() {
        let extractor = TestMetadataExtractor;
        let mut stream_extractor = extractor.create_stream_extractor();

        stream_extractor.process_chunk(json!({"id": "chunk-1"}));
        stream_extractor.process_chunk(json!({"model": "gpt-4"})); // no id
        stream_extractor.process_chunk(json!({"id": "chunk-2"}));

        let metadata = stream_extractor.build_metadata();

        assert!(metadata.is_some());
        let metadata = metadata.unwrap();
        let provider_data = &metadata["test-provider"];

        assert_eq!(provider_data.get("chunks"), Some(&json!(2)));
        assert_eq!(
            provider_data.get("ids"),
            Some(&json!(["chunk-1", "chunk-2"]))
        );
    }
}
