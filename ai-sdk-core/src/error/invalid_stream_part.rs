use crate::error::AISDKError;
use serde_json::Value;

/// Builder for [`AISDKError::InvalidStreamPart`].
///
/// # Examples
///
/// ```
/// use ai_sdk_core::error::{AISDKError, InvalidStreamPartErrorBuilder};
/// use serde_json::json;
///
/// let chunk = json!({"type": "unknown", "data": "test"});
/// let error = InvalidStreamPartErrorBuilder::new(chunk.clone())
///     .message("Unknown stream part type")
///     .build();
///
/// match error {
///     AISDKError::InvalidStreamPart { chunk: _, message } => {
///         assert_eq!(message, "Unknown stream part type");
///     }
///     _ => panic!("Expected InvalidStreamPart"),
/// }
/// ```
#[derive(Debug, Clone)]
pub struct InvalidStreamPartErrorBuilder {
    chunk: Value,
    message: Option<String>,
}

impl InvalidStreamPartErrorBuilder {
    /// Creates a new builder for an invalid stream part error.
    ///
    /// # Arguments
    ///
    /// * `chunk` - The stream chunk that was invalid (as a JSON value)
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::InvalidStreamPartErrorBuilder;
    /// use serde_json::json;
    ///
    /// let chunk = json!({"type": "invalid-type"});
    /// let builder = InvalidStreamPartErrorBuilder::new(chunk);
    /// ```
    pub fn new(chunk: Value) -> Self {
        Self {
            chunk,
            message: None,
        }
    }

    /// Sets the error message explaining why the stream part is invalid.
    ///
    /// # Arguments
    ///
    /// * `message` - Description of why the stream part is invalid
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::InvalidStreamPartErrorBuilder;
    /// use serde_json::json;
    ///
    /// let chunk = json!({"type": "unknown"});
    /// let builder = InvalidStreamPartErrorBuilder::new(chunk)
    ///     .message("Unsupported stream part type");
    /// ```
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Builds the [`AISDKError::InvalidStreamPart`] error.
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::InvalidStreamPartErrorBuilder;
    /// use serde_json::json;
    ///
    /// let chunk = json!({"type": "invalid"});
    /// let error = InvalidStreamPartErrorBuilder::new(chunk)
    ///     .message("Invalid stream part")
    ///     .build();
    /// ```
    pub fn build(self) -> AISDKError {
        AISDKError::InvalidStreamPart {
            chunk: self.chunk.to_string(),
            message: self
                .message
                .unwrap_or_else(|| "Invalid stream part".to_string()),
        }
    }
}

impl AISDKError {
    /// Creates a new invalid stream part error.
    ///
    /// This is a convenience method that creates an error with a chunk and message in one call.
    ///
    /// # Arguments
    ///
    /// * `chunk` - The stream chunk that was invalid (as a JSON value)
    /// * `message` - Description of why the stream part is invalid
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::AISDKError;
    /// use serde_json::json;
    ///
    /// let chunk = json!({"type": "unknown", "data": "test"});
    /// let error = AISDKError::invalid_stream_part(chunk.clone(), "Unknown stream part type");
    ///
    /// match error {
    ///     AISDKError::InvalidStreamPart { chunk, message } => {
    ///         assert!(chunk.contains("unknown"));
    ///         assert_eq!(message, "Unknown stream part type");
    ///     }
    ///     _ => panic!("Expected InvalidStreamPart"),
    /// }
    /// ```
    pub fn invalid_stream_part(chunk: Value, message: impl Into<String>) -> Self {
        Self::InvalidStreamPart {
            chunk: chunk.to_string(),
            message: message.into(),
        }
    }

    /// Creates a builder for an invalid stream part error.
    ///
    /// # Arguments
    ///
    /// * `chunk` - The stream chunk that was invalid (as a JSON value)
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::AISDKError;
    /// use serde_json::json;
    ///
    /// let chunk = json!({"type": "invalid"});
    /// let error = AISDKError::invalid_stream_part_builder(chunk)
    ///     .message("Invalid stream part type")
    ///     .build();
    /// ```
    pub fn invalid_stream_part_builder(chunk: Value) -> InvalidStreamPartErrorBuilder {
        InvalidStreamPartErrorBuilder::new(chunk)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_invalid_stream_part_simple() {
        let chunk = json!({"type": "unknown", "data": "test"});
        let error = AISDKError::invalid_stream_part(chunk.clone(), "Unknown stream part type");

        match error {
            AISDKError::InvalidStreamPart { chunk, message } => {
                assert!(chunk.contains("unknown"));
                assert!(chunk.contains("test"));
                assert_eq!(message, "Unknown stream part type");
            }
            _ => panic!("Expected InvalidStreamPart"),
        }
    }

    #[test]
    fn test_invalid_stream_part_builder() {
        let chunk = json!({"type": "malformed"});
        let error = InvalidStreamPartErrorBuilder::new(chunk)
            .message("Malformed stream chunk")
            .build();

        match error {
            AISDKError::InvalidStreamPart { chunk, message } => {
                assert!(chunk.contains("malformed"));
                assert_eq!(message, "Malformed stream chunk");
            }
            _ => panic!("Expected InvalidStreamPart"),
        }
    }

    #[test]
    fn test_invalid_stream_part_builder_via_error() {
        let chunk = json!({"type": "unexpected", "id": 123});
        let error = AISDKError::invalid_stream_part_builder(chunk)
            .message("Unexpected stream part received")
            .build();

        match error {
            AISDKError::InvalidStreamPart { chunk, message } => {
                assert!(chunk.contains("unexpected"));
                assert!(chunk.contains("123"));
                assert_eq!(message, "Unexpected stream part received");
            }
            _ => panic!("Expected InvalidStreamPart"),
        }
    }

    #[test]
    fn test_invalid_stream_part_builder_minimal() {
        let chunk = json!({"type": "error"});
        let error = InvalidStreamPartErrorBuilder::new(chunk).build();

        match error {
            AISDKError::InvalidStreamPart { chunk, message } => {
                assert!(chunk.contains("error"));
                assert_eq!(message, "Invalid stream part");
            }
            _ => panic!("Expected InvalidStreamPart"),
        }
    }

    #[test]
    fn test_invalid_stream_part_display() {
        let chunk = json!({"type": "bad-type", "content": "test"});
        let error = AISDKError::invalid_stream_part(chunk, "Unsupported stream part type");
        let display = format!("{}", error);
        assert!(display.contains("Invalid stream part"));
        assert!(display.contains("Unsupported stream part type"));
    }

    #[test]
    fn test_invalid_stream_part_with_complex_chunk() {
        let chunk = json!({
            "type": "complex",
            "nested": {
                "field1": "value1",
                "field2": [1, 2, 3]
            }
        });
        let error =
            AISDKError::invalid_stream_part(chunk.clone(), "Complex chunk validation failed");

        match error {
            AISDKError::InvalidStreamPart { chunk, message } => {
                assert!(chunk.contains("complex"));
                assert!(chunk.contains("nested"));
                assert!(chunk.contains("value1"));
                assert_eq!(message, "Complex chunk validation failed");
            }
            _ => panic!("Expected InvalidStreamPart"),
        }
    }

    #[test]
    fn test_invalid_stream_part_with_array_chunk() {
        let chunk = json!(["item1", "item2", "item3"]);
        let error = AISDKError::invalid_stream_part(chunk, "Array chunks are not supported");

        match error {
            AISDKError::InvalidStreamPart { chunk, message } => {
                assert!(chunk.contains("item1"));
                assert!(chunk.contains("item2"));
                assert_eq!(message, "Array chunks are not supported");
            }
            _ => panic!("Expected InvalidStreamPart"),
        }
    }
}
