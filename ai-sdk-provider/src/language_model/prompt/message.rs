use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::shared::provider_options::ProviderOptions;

/// A message in a prompt with role-specific content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum Message {
    /// System message with text content
    #[serde(rename_all = "camelCase")]
    System {
        /// The system message content
        content: String,

        /// Additional provider-specific options
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// User message with text and/or file parts
    #[serde(rename_all = "camelCase")]
    User {
        /// Array of text or file content parts
        content: Vec<UserMessagePart>,

        /// Additional provider-specific options
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// Assistant message with various content types
    #[serde(rename_all = "camelCase")]
    Assistant {
        /// Array of content parts (text, files, reasoning, tool calls, tool results)
        content: Vec<AssistantMessagePart>,

        /// Additional provider-specific options
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// Tool message with tool results
    #[serde(rename_all = "camelCase")]
    Tool {
        /// Array of tool result parts
        content: Vec<ToolResultPart>,

        /// Additional provider-specific options
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },
}

/// Content parts that can appear in a user message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum UserMessagePart {
    /// Text content
    #[serde(rename_all = "camelCase")]
    Text {
        /// The text content
        text: String,

        /// Additional provider-specific options
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// File content
    #[serde(rename_all = "camelCase")]
    File {
        /// Optional filename of the file
        #[serde(skip_serializing_if = "Option::is_none")]
        filename: Option<String>,

        /// File data. Can be binary, base64, or URL
        data: DataContent,

        /// IANA media type of the file
        media_type: String,

        /// Additional provider-specific options
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },
}

/// Content parts that can appear in an assistant message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum AssistantMessagePart {
    /// Text content
    #[serde(rename_all = "camelCase")]
    Text {
        /// The text content
        text: String,

        /// Additional provider-specific options
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// File content
    #[serde(rename_all = "camelCase")]
    File {
        /// Optional filename of the file
        #[serde(skip_serializing_if = "Option::is_none")]
        filename: Option<String>,

        /// File data
        data: DataContent,

        /// IANA media type of the file
        media_type: String,

        /// Additional provider-specific options
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// Reasoning content
    #[serde(rename_all = "camelCase")]
    Reasoning {
        /// The reasoning text
        text: String,

        /// Additional provider-specific options
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// Tool call
    #[serde(rename = "tool-call", rename_all = "camelCase")]
    ToolCall {
        /// ID of the tool call
        tool_call_id: String,

        /// Name of the tool being called
        tool_name: String,

        /// Arguments of the tool call (JSON-serializable object)
        input: Value,

        /// Whether the tool call will be executed by the provider
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_executed: Option<bool>,

        /// Additional provider-specific options
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// Tool result
    #[serde(rename = "tool-result", rename_all = "camelCase")]
    ToolResult {
        /// ID of the tool call this result is associated with
        tool_call_id: String,

        /// Name of the tool that generated this result
        tool_name: String,

        /// Result of the tool call
        output: ToolResultOutput,

        /// Additional provider-specific options
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },
}

/// Tool result part (used in tool messages and assistant messages)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolResultPart {
    /// Type discriminator (always "tool-result")
    #[serde(rename = "type")]
    part_type: ToolResultPartType,

    /// ID of the tool call this result is associated with
    pub tool_call_id: String,

    /// Name of the tool that generated this result
    pub tool_name: String,

    /// Result of the tool call
    pub output: ToolResultOutput,

    /// Additional provider-specific options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<ProviderOptions>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "tool-result")]
struct ToolResultPartType;

/// Tool result output types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ToolResultOutput {
    /// Plain text output
    Text {
        /// The text value
        value: String,
    },

    /// JSON output
    Json {
        /// The JSON value
        value: Value,
    },

    /// Error as text
    #[serde(rename = "error-text")]
    ErrorText {
        /// The error message
        value: String,
    },

    /// Error as JSON
    #[serde(rename = "error-json")]
    ErrorJson {
        /// The error value as JSON
        value: Value,
    },

    /// Content with multiple parts (text and/or media)
    Content {
        /// Array of content items
        value: Vec<ToolResultContentItem>,
    },
}

/// Content items within a tool result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ToolResultContentItem {
    /// Text content
    Text {
        /// Text content
        text: String,
    },

    /// Media content (image, audio, etc.)
    Media {
        /// Base64-encoded media data
        data: String,

        /// IANA media type
        #[serde(rename = "mediaType")]
        media_type: String,
    },
}

/// Data content - can be bytes, base64 string, or URL
#[derive(Debug, Clone, PartialEq)]
pub enum DataContent {
    /// Binary data
    Bytes(Vec<u8>),

    /// Base64 encoded string
    Base64(String),

    /// URL
    Url(url::Url),
}

// Custom serialization for DataContent
impl Serialize for DataContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Bytes(bytes) => {
                // Serialize as base64
                use base64::{Engine as _, engine::general_purpose};
                let encoded = general_purpose::STANDARD.encode(bytes);
                serializer.serialize_str(&encoded)
            }
            Self::Base64(s) => serializer.serialize_str(s),
            Self::Url(url) => serializer.serialize_str(url.as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for DataContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        // Try to parse as URL first
        if let Ok(url) = url::Url::parse(&s) {
            if url.scheme() == "http" || url.scheme() == "https" || url.scheme() == "data" {
                return Ok(Self::Url(url));
            }
        }

        // Otherwise treat as base64
        Ok(Self::Base64(s))
    }
}

// Helper implementations for Message
impl Message {
    /// Create a system message
    pub fn system(content: impl Into<String>) -> Self {
        Self::System {
            content: content.into(),
            provider_options: None,
        }
    }

    /// Create a user message with text
    pub fn user_text(text: impl Into<String>) -> Self {
        Self::User {
            content: vec![UserMessagePart::Text {
                text: text.into(),
                provider_options: None,
            }],
            provider_options: None,
        }
    }

    /// Create an assistant message with text
    pub fn assistant_text(text: impl Into<String>) -> Self {
        Self::Assistant {
            content: vec![AssistantMessagePart::Text {
                text: text.into(),
                provider_options: None,
            }],
            provider_options: None,
        }
    }

    /// Get the role of this message
    pub fn role(&self) -> &str {
        match self {
            Self::System { .. } => "system",
            Self::User { .. } => "user",
            Self::Assistant { .. } => "assistant",
            Self::Tool { .. } => "tool",
        }
    }
}

impl ToolResultPart {
    /// Create a new tool result part
    pub fn new(
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        output: ToolResultOutput,
    ) -> Self {
        Self {
            part_type: ToolResultPartType,
            tool_call_id: tool_call_id.into(),
            tool_name: tool_name.into(),
            output,
            provider_options: None,
        }
    }
}

impl ToolResultOutput {
    /// Create a text output
    pub fn text(value: impl Into<String>) -> Self {
        Self::Text {
            value: value.into(),
        }
    }

    /// Create a JSON output
    pub fn json(value: Value) -> Self {
        Self::Json { value }
    }

    /// Create an error text output
    pub fn error_text(value: impl Into<String>) -> Self {
        Self::ErrorText {
            value: value.into(),
        }
    }

    /// Create an error JSON output
    pub fn error_json(value: Value) -> Self {
        Self::ErrorJson { value }
    }

    /// Create a content output with items
    pub fn content(value: Vec<ToolResultContentItem>) -> Self {
        Self::Content { value }
    }
}
