use ai_sdk_provider::language_model::response_metadata::LanguageModelResponseMetadata;

/// Creates response metadata from OpenAI-compatible API response fields.
///
/// # Arguments
///
/// * `id` - The unique identifier for the response
/// * `model` - The model ID that generated the response
/// * `created` - Unix timestamp in seconds when the response was created
///
/// # Returns
///
/// A `ResponseMetadata` struct with the provided values, converting the timestamp
/// from seconds to milliseconds.
///
/// # Example
///
/// ```
/// use ai_sdk_provider::language_model::response_metadata::LanguageModelResponseMetadata;
///
/// // This is an internal utility function
/// fn get_response_metadata(
///     id: Option<String>,
///     model: Option<String>,
///     created: Option<i64>,
/// ) -> LanguageModelResponseMetadata {
///     LanguageModelResponseMetadata {
///         id,
///         model_id: model,
///         timestamp: created.map(|ts| ts * 1000),
///     }
/// }
///
/// let metadata = get_response_metadata(
///     Some("chatcmpl-123".to_string()),
///     Some("gpt-4".to_string()),
///     Some(1677649420)
/// );
/// assert_eq!(metadata.id, Some("chatcmpl-123".to_string()));
/// ```
pub fn get_response_metadata(
    id: Option<String>,
    model: Option<String>,
    created: Option<i64>,
) -> LanguageModelResponseMetadata {
    LanguageModelResponseMetadata {
        id,
        model_id: model,
        // Convert Unix timestamp from seconds to milliseconds
        timestamp: created.map(|ts| ts * 1000),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_response_metadata_all_fields() {
        let metadata = get_response_metadata(
            Some("chatcmpl-123".to_string()),
            Some("gpt-4".to_string()),
            Some(1677649420),
        );

        assert_eq!(metadata.id, Some("chatcmpl-123".to_string()));
        assert_eq!(metadata.model_id, Some("gpt-4".to_string()));
        assert_eq!(metadata.timestamp, Some(1677649420000)); // seconds * 1000
    }

    #[test]
    fn test_get_response_metadata_all_none() {
        let metadata = get_response_metadata(None, None, None);

        assert_eq!(metadata.id, None);
        assert_eq!(metadata.model_id, None);
        assert_eq!(metadata.timestamp, None);
    }

    #[test]
    fn test_get_response_metadata_partial_fields() {
        let metadata =
            get_response_metadata(Some("chatcmpl-456".to_string()), None, Some(1677649420));

        assert_eq!(metadata.id, Some("chatcmpl-456".to_string()));
        assert_eq!(metadata.model_id, None);
        assert_eq!(metadata.timestamp, Some(1677649420000));
    }

    #[test]
    fn test_get_response_metadata_timestamp_conversion() {
        let metadata = get_response_metadata(None, None, Some(1));

        // 1 second should convert to 1000 milliseconds
        assert_eq!(metadata.timestamp, Some(1000));
    }

    #[test]
    fn test_get_response_metadata_zero_timestamp() {
        let metadata = get_response_metadata(None, None, Some(0));

        assert_eq!(metadata.timestamp, Some(0));
    }

    #[test]
    fn test_get_response_metadata_negative_timestamp() {
        let metadata = get_response_metadata(None, None, Some(-1));

        // Negative timestamps should still be converted
        assert_eq!(metadata.timestamp, Some(-1000));
    }

    #[test]
    fn test_get_response_metadata_large_timestamp() {
        let metadata = get_response_metadata(None, None, Some(9999999999));

        assert_eq!(metadata.timestamp, Some(9999999999000));
    }
}
