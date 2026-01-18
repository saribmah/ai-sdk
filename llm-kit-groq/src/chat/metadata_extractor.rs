use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Groq-specific usage metadata.
///
/// This includes prompt cache token details specific to Groq's API,
/// which appear in the `prompt_tokens_details` field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroqUsage {
    /// Number of tokens that were cached (from `prompt_tokens_details.cached_tokens`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_tokens: Option<u64>,
}

/// Extracts Groq-specific metadata from API responses.
pub struct GroqMetadataExtractor;

impl GroqMetadataExtractor {
    /// Extracts response metadata (id, model, timestamp) from a response.
    pub fn extract_response_metadata(
        id: Option<&str>,
        model: Option<&str>,
        created: Option<i64>,
    ) -> HashMap<String, Value> {
        let mut metadata = HashMap::new();

        if let Some(id_val) = id {
            metadata.insert("id".to_string(), Value::String(id_val.to_string()));
        }

        if let Some(model_val) = model {
            metadata.insert("modelId".to_string(), Value::String(model_val.to_string()));
        }

        if let Some(created_val) = created {
            // Convert Unix timestamp (seconds) to ISO 8601 string
            if let Some(datetime) = chrono::DateTime::from_timestamp(created_val, 0) {
                metadata.insert(
                    "timestamp".to_string(),
                    Value::String(datetime.to_rfc3339()),
                );
            }
        }

        metadata
    }

    /// Extracts Groq-specific metadata from a non-streaming response.
    ///
    /// Extracts cached tokens from the `usage.prompt_tokens_details.cached_tokens` field.
    pub fn extract_metadata(response_body: &Value) -> Option<HashMap<String, Value>> {
        let usage = Self::extract_usage(response_body)?;
        Self::build_metadata(&usage)
    }

    /// Extracts Groq-specific metadata from a streaming chunk.
    ///
    /// In Groq's streaming API, usage information is in the `x_groq.usage` field.
    pub fn extract_stream_metadata(chunk: &Value) -> Option<HashMap<String, Value>> {
        // Extract from x_groq.usage for streaming
        if let Some(x_groq) = chunk.get("x_groq")
            && let Some(usage) = Self::extract_usage_from_x_groq(x_groq)
        {
            return Self::build_metadata(&usage);
        }
        None
    }

    /// Extracts usage information from a standard response body.
    fn extract_usage(body: &Value) -> Option<GroqUsage> {
        let usage = body.get("usage")?;

        let cached_tokens = usage
            .get("prompt_tokens_details")
            .and_then(|d| d.get("cached_tokens"))
            .and_then(|v| v.as_u64());

        // Only return if cached_tokens is present
        cached_tokens.map(|tokens| GroqUsage {
            cached_tokens: Some(tokens),
        })
    }

    /// Extracts usage information from the `x_groq` field (streaming).
    fn extract_usage_from_x_groq(x_groq: &Value) -> Option<GroqUsage> {
        let usage = x_groq.get("usage")?;

        let cached_tokens = usage
            .get("prompt_tokens_details")
            .and_then(|d| d.get("cached_tokens"))
            .and_then(|v| v.as_u64());

        cached_tokens.map(|tokens| GroqUsage {
            cached_tokens: Some(tokens),
        })
    }

    /// Builds metadata map from usage information.
    fn build_metadata(usage: &GroqUsage) -> Option<HashMap<String, Value>> {
        if let Some(cached_tokens) = usage.cached_tokens {
            let mut metadata = HashMap::new();
            let mut groq_metadata = serde_json::Map::new();

            groq_metadata.insert(
                "cachedTokens".to_string(),
                Value::Number(cached_tokens.into()),
            );

            metadata.insert("groq".to_string(), Value::Object(groq_metadata));
            Some(metadata)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_extract_response_metadata() {
        let metadata = GroqMetadataExtractor::extract_response_metadata(
            Some("chatcmpl-123"),
            Some("llama-3.1-8b-instant"),
            Some(1234567890),
        );

        assert_eq!(
            metadata.get("id").and_then(|v| v.as_str()),
            Some("chatcmpl-123")
        );
        assert_eq!(
            metadata.get("modelId").and_then(|v| v.as_str()),
            Some("llama-3.1-8b-instant")
        );
        assert!(metadata.contains_key("timestamp"));
    }

    #[test]
    fn test_extract_metadata_with_cached_tokens() {
        let response = json!({
            "usage": {
                "prompt_tokens_details": {
                    "cached_tokens": 100
                }
            }
        });

        let metadata = GroqMetadataExtractor::extract_metadata(&response).unwrap();
        let groq = metadata.get("groq").unwrap();

        assert_eq!(groq.get("cachedTokens").unwrap().as_u64(), Some(100));
    }

    #[test]
    fn test_extract_metadata_without_usage() {
        let response = json!({
            "choices": []
        });

        let metadata = GroqMetadataExtractor::extract_metadata(&response);
        assert!(metadata.is_none());
    }

    #[test]
    fn test_extract_metadata_without_cached_tokens() {
        let response = json!({
            "usage": {
                "prompt_tokens": 50,
                "completion_tokens": 25
            }
        });

        let metadata = GroqMetadataExtractor::extract_metadata(&response);
        assert!(metadata.is_none());
    }

    #[test]
    fn test_extract_stream_metadata_from_x_groq() {
        let chunk = json!({
            "x_groq": {
                "usage": {
                    "prompt_tokens_details": {
                        "cached_tokens": 75
                    }
                }
            }
        });

        let metadata = GroqMetadataExtractor::extract_stream_metadata(&chunk).unwrap();
        let groq = metadata.get("groq").unwrap();

        assert_eq!(groq.get("cachedTokens").unwrap().as_u64(), Some(75));
    }

    #[test]
    fn test_extract_stream_metadata_without_x_groq() {
        let chunk = json!({
            "choices": [{
                "delta": { "content": "text" }
            }]
        });

        let metadata = GroqMetadataExtractor::extract_stream_metadata(&chunk);
        assert!(metadata.is_none());
    }
}
