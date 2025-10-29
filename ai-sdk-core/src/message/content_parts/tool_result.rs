use ai_sdk_provider::shared::provider_options::ProviderOptions;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Tool result content part of a prompt.
///
/// It contains the result of the tool call with the matching ID.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolResultPart {
    /// ID of the tool call that this result is associated with.
    #[serde(rename = "toolCallId")]
    pub tool_call_id: String,

    /// Name of the tool that generated this result.
    #[serde(rename = "toolName")]
    pub tool_name: String,

    /// Result of the tool call.
    pub output: ToolResultOutput,

    /// Additional provider-specific metadata.
    #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<ProviderOptions>,
}

/// Output of a tool result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ToolResultOutput {
    /// Text tool output that should be directly sent to the API.
    Text {
        value: String,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// JSON tool output.
    Json {
        value: Value,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// Type when the user has denied the execution of the tool call.
    ExecutionDenied {
        #[serde(skip_serializing_if = "Option::is_none")]
        reason: Option<String>,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// Error as text.
    ErrorText {
        value: String,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// Error as JSON.
    ErrorJson {
        value: Value,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// Content array with various content types.
    Content { value: Vec<ToolResultContentPart> },
}

/// Content part within a tool result output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ToolResultContentPart {
    /// Text content.
    Text {
        text: String,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// Media content (deprecated - use FileData or ImageData instead).
    #[deprecated(note = "Use FileData or ImageData instead")]
    Media {
        data: String,
        #[serde(rename = "mediaType")]
        media_type: String,
    },

    /// File data.
    FileData {
        /// Base64-encoded media data.
        data: String,
        /// IANA media type.
        #[serde(rename = "mediaType")]
        media_type: String,
        /// Optional filename of the file.
        #[serde(skip_serializing_if = "Option::is_none")]
        filename: Option<String>,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// File URL.
    FileUrl {
        url: String,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// File ID.
    FileId {
        /// ID of the file.
        ///
        /// If you use multiple providers, you can specify provider-specific IDs
        /// using the HashMap variant. The key is the provider name (e.g., "openai", "anthropic").
        #[serde(rename = "fileId")]
        file_id: FileId,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// Image data (base64-encoded).
    ImageData {
        /// Base64-encoded image data.
        data: String,
        /// IANA media type.
        #[serde(rename = "mediaType")]
        media_type: String,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// Image URL.
    ImageUrl {
        url: String,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// Image file ID.
    ImageFileId {
        /// Image file ID.
        ///
        /// If you use multiple providers, you can specify provider-specific IDs
        /// using the HashMap variant.
        #[serde(rename = "fileId")]
        file_id: FileId,
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// Custom content part.
    ///
    /// This can be used to implement provider-specific content parts.
    Custom {
        #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },
}

/// File ID - either a single string or provider-specific IDs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FileId {
    /// Single file ID (works with a single provider).
    Single(String),
    /// Provider-specific file IDs.
    ///
    /// The key is the provider name (e.g., "openai", "anthropic").
    Multiple(HashMap<String, String>),
}

impl ToolResultPart {
    /// Creates a new tool result part.
    pub fn new(
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        output: ToolResultOutput,
    ) -> Self {
        Self {
            tool_call_id: tool_call_id.into(),
            tool_name: tool_name.into(),
            output,
            provider_options: None,
        }
    }

    /// Sets the provider options.
    pub fn with_provider_options(mut self, options: ProviderOptions) -> Self {
        self.provider_options = Some(options);
        self
    }
}

impl ToolResultOutput {
    /// Creates a text output.
    pub fn text(value: impl Into<String>) -> Self {
        Self::Text {
            value: value.into(),
            provider_options: None,
        }
    }

    /// Creates a JSON output.
    pub fn json(value: Value) -> Self {
        Self::Json {
            value,
            provider_options: None,
        }
    }

    /// Creates an execution denied output.
    pub fn execution_denied(reason: Option<String>) -> Self {
        Self::ExecutionDenied {
            reason,
            provider_options: None,
        }
    }

    /// Creates an error text output.
    pub fn error_text(value: impl Into<String>) -> Self {
        Self::ErrorText {
            value: value.into(),
            provider_options: None,
        }
    }

    /// Creates an error JSON output.
    pub fn error_json(value: Value) -> Self {
        Self::ErrorJson {
            value,
            provider_options: None,
        }
    }

    /// Creates a content output.
    pub fn content(parts: Vec<ToolResultContentPart>) -> Self {
        Self::Content { value: parts }
    }
}

impl From<String> for FileId {
    fn from(s: String) -> Self {
        Self::Single(s)
    }
}

impl From<&str> for FileId {
    fn from(s: &str) -> Self {
        Self::Single(s.to_string())
    }
}

impl From<HashMap<String, String>> for FileId {
    fn from(map: HashMap<String, String>) -> Self {
        Self::Multiple(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_tool_result_part_text() {
        let output = ToolResultOutput::text("Success!");
        let part = ToolResultPart::new("call_123", "get_weather", output);

        assert_eq!(part.tool_call_id, "call_123");
        assert_eq!(part.tool_name, "get_weather");
        match part.output {
            ToolResultOutput::Text { value, .. } => assert_eq!(value, "Success!"),
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_tool_result_part_json() {
        let json_value = json!({"temperature": 72});
        let output = ToolResultOutput::json(json_value.clone());
        let part = ToolResultPart::new("call_456", "get_weather", output);

        match part.output {
            ToolResultOutput::Json { value, .. } => assert_eq!(value, json_value),
            _ => panic!("Expected Json variant"),
        }
    }

    #[test]
    fn test_tool_result_output_execution_denied() {
        let output = ToolResultOutput::execution_denied(Some("Permission denied".to_string()));

        match output {
            ToolResultOutput::ExecutionDenied { reason, .. } => {
                assert_eq!(reason, Some("Permission denied".to_string()));
            }
            _ => panic!("Expected ExecutionDenied variant"),
        }
    }

    #[test]
    fn test_tool_result_output_error_text() {
        let output = ToolResultOutput::error_text("Connection failed");

        match output {
            ToolResultOutput::ErrorText { value, .. } => {
                assert_eq!(value, "Connection failed");
            }
            _ => panic!("Expected ErrorText variant"),
        }
    }

    #[test]
    fn test_file_id_single() {
        let file_id = FileId::from("file_123");
        match file_id {
            FileId::Single(s) => assert_eq!(s, "file_123"),
            _ => panic!("Expected Single variant"),
        }
    }

    #[test]
    fn test_file_id_multiple() {
        let mut map = HashMap::new();
        map.insert("openai".to_string(), "file_openai_123".to_string());
        map.insert("anthropic".to_string(), "file_anthropic_456".to_string());

        let file_id = FileId::from(map.clone());
        match file_id {
            FileId::Multiple(m) => assert_eq!(m, map),
            _ => panic!("Expected Multiple variant"),
        }
    }
}
