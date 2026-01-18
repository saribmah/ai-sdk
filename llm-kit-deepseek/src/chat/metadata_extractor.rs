use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;

/// DeepSeek-specific usage metadata.
///
/// This includes prompt cache hit/miss tokens which are specific to DeepSeek's API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepSeekUsage {
    /// Number of tokens that hit the prompt cache
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_hit_tokens: Option<u64>,

    /// Number of tokens that missed the prompt cache
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_miss_tokens: Option<u64>,
}

/// Extracts DeepSeek-specific metadata from API responses.
pub struct DeepSeekMetadataExtractor;

impl DeepSeekMetadataExtractor {
    /// Extracts metadata from a non-streaming response.
    pub fn extract_metadata(response_body: &Value) -> Option<HashMap<String, Value>> {
        let usage = Self::extract_usage(response_body)?;
        Self::build_metadata(&usage)
    }

    /// Extracts metadata from a streaming chunk.
    ///
    /// Returns metadata only when the stream is complete (finish_reason is present).
    pub fn extract_stream_metadata(chunk: &Value) -> Option<HashMap<String, Value>> {
        // Only extract usage when the stream is complete
        if let Some(choices) = chunk.get("choices").and_then(|c| c.as_array())
            && let Some(choice) = choices.first()
            && choice
                .get("finish_reason")
                .and_then(|f| f.as_str())
                .is_some()
            && let Some(usage) = Self::extract_usage(chunk)
        {
            return Self::build_metadata(&usage);
        }
        None
    }

    /// Extracts usage information from a response body.
    fn extract_usage(body: &Value) -> Option<DeepSeekUsage> {
        let usage = body.get("usage")?;

        let prompt_cache_hit_tokens = usage
            .get("prompt_cache_hit_tokens")
            .and_then(|v| v.as_u64());

        let prompt_cache_miss_tokens = usage
            .get("prompt_cache_miss_tokens")
            .and_then(|v| v.as_u64());

        // Only return if at least one field is present
        if prompt_cache_hit_tokens.is_some() || prompt_cache_miss_tokens.is_some() {
            Some(DeepSeekUsage {
                prompt_cache_hit_tokens,
                prompt_cache_miss_tokens,
            })
        } else {
            None
        }
    }

    /// Builds metadata map from usage information.
    fn build_metadata(usage: &DeepSeekUsage) -> Option<HashMap<String, Value>> {
        let mut metadata = HashMap::new();
        let mut deepseek_metadata = Map::new();

        if let Some(hit_tokens) = usage.prompt_cache_hit_tokens {
            deepseek_metadata.insert(
                "promptCacheHitTokens".to_string(),
                Value::Number(hit_tokens.into()),
            );
        }

        if let Some(miss_tokens) = usage.prompt_cache_miss_tokens {
            deepseek_metadata.insert(
                "promptCacheMissTokens".to_string(),
                Value::Number(miss_tokens.into()),
            );
        }

        if !deepseek_metadata.is_empty() {
            metadata.insert("deepseek".to_string(), Value::Object(deepseek_metadata));
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
    fn test_extract_metadata_with_cache_tokens() {
        let response = json!({
            "usage": {
                "prompt_cache_hit_tokens": 100,
                "prompt_cache_miss_tokens": 50
            }
        });

        let metadata = DeepSeekMetadataExtractor::extract_metadata(&response).unwrap();
        let deepseek = metadata.get("deepseek").unwrap();

        assert_eq!(
            deepseek.get("promptCacheHitTokens").unwrap().as_u64(),
            Some(100)
        );
        assert_eq!(
            deepseek.get("promptCacheMissTokens").unwrap().as_u64(),
            Some(50)
        );
    }

    #[test]
    fn test_extract_metadata_without_usage() {
        let response = json!({
            "choices": []
        });

        let metadata = DeepSeekMetadataExtractor::extract_metadata(&response);
        assert!(metadata.is_none());
    }

    #[test]
    fn test_extract_metadata_partial_usage() {
        let response = json!({
            "usage": {
                "prompt_cache_hit_tokens": 100
            }
        });

        let metadata = DeepSeekMetadataExtractor::extract_metadata(&response).unwrap();
        let deepseek = metadata.get("deepseek").unwrap();

        assert_eq!(
            deepseek.get("promptCacheHitTokens").unwrap().as_u64(),
            Some(100)
        );
        assert!(deepseek.get("promptCacheMissTokens").is_none());
    }

    #[test]
    fn test_extract_stream_metadata_with_finish_reason() {
        let chunk = json!({
            "choices": [{
                "finish_reason": "stop"
            }],
            "usage": {
                "prompt_cache_hit_tokens": 75,
                "prompt_cache_miss_tokens": 25
            }
        });

        let metadata = DeepSeekMetadataExtractor::extract_stream_metadata(&chunk).unwrap();
        let deepseek = metadata.get("deepseek").unwrap();

        assert_eq!(
            deepseek.get("promptCacheHitTokens").unwrap().as_u64(),
            Some(75)
        );
        assert_eq!(
            deepseek.get("promptCacheMissTokens").unwrap().as_u64(),
            Some(25)
        );
    }

    #[test]
    fn test_extract_stream_metadata_without_finish_reason() {
        let chunk = json!({
            "choices": [{
                "delta": { "content": "text" }
            }],
            "usage": {
                "prompt_cache_hit_tokens": 75
            }
        });

        let metadata = DeepSeekMetadataExtractor::extract_stream_metadata(&chunk);
        assert!(metadata.is_none());
    }

    #[test]
    fn test_extract_stream_metadata_empty_choices() {
        let chunk = json!({
            "choices": [],
            "usage": {
                "prompt_cache_hit_tokens": 75
            }
        });

        let metadata = DeepSeekMetadataExtractor::extract_stream_metadata(&chunk);
        assert!(metadata.is_none());
    }
}
