use super::tool_set::ToolSet;
use crate::error::AISDKError;
use crate::prompt::message::Message;
use ai_sdk_provider::language_model::content::tool_call::LanguageModelToolCall;
use std::future::Future;
use std::pin::Pin;

/// Options passed to a tool call repair function.
///
/// This struct contains all the context needed to repair a tool call that failed to parse.
pub struct ToolCallRepairOptions {
    /// The system prompt (if any).
    pub system: Option<String>,

    /// The messages in the current generation step.
    pub messages: Vec<Message>,

    /// The tool call that failed to parse.
    pub tool_call: LanguageModelToolCall,

    /// The tools that are available.
    pub tools: ToolSet,

    /// The error that occurred while parsing the tool call.
    pub error: AISDKError,
}

impl ToolCallRepairOptions {
    /// Creates new tool call repair options.
    pub fn new(
        system: Option<String>,
        messages: Vec<Message>,
        tool_call: LanguageModelToolCall,
        tools: ToolSet,
        error: AISDKError,
    ) -> Self {
        Self {
            system,
            messages,
            tool_call,
            tools,
            error,
        }
    }
}

/// A function that attempts to repair a tool call that failed to parse.
///
/// It receives the repair options as an argument and returns the repaired
/// tool call, or `None` if the tool call cannot be repaired.
///
/// # Arguments
///
/// * `options` - The repair options containing:
///   - `system`: The system prompt (if any)
///   - `messages`: The messages in the current generation step
///   - `tool_call`: The tool call that failed to parse
///   - `tools`: The tools that are available (ToolSet)
///   - `error`: The error that occurred (InvalidToolInput or NoSuchTool)
///
/// # Returns
///
/// A `Future` that resolves to `Option<ToolCall>`:
/// - `Some(ToolCall)`: The repaired tool call
/// - `None`: The tool call cannot be repaired
///
/// # Example
///
/// ```rust
/// use ai_sdk_core::generate_text::tool_call_repair_function::{ToolCallRepairFunction, ToolCallRepairOptions};
/// use ai_sdk_provider::language_model::tool_call::ToolCall;
/// use std::future::Future;
/// use std::pin::Pin;
///
/// // Define a simple repair function that returns None (no repair)
/// fn no_repair_function(options: ToolCallRepairOptions) -> Pin<Box<dyn Future<Output = Option<ToolCall>> + Send>> {
///     Box::pin(async move {
///         // In a real implementation, you might use an LLM to repair the tool call
///         None
///     })
/// }
///
/// // The function signature matches ToolCallRepairFunction
/// let repair_fn: ToolCallRepairFunction = Box::new(no_repair_function);
/// ```
pub type ToolCallRepairFunction = Box<
    dyn Fn(
            ToolCallRepairOptions,
        ) -> Pin<Box<dyn Future<Output = Option<LanguageModelToolCall>> + Send>>
        + Send
        + Sync,
>;

/// Creates a no-op tool call repair function that always returns `None`.
///
/// This is useful as a default when you don't want to implement tool call repair.
///
/// # Example
///
/// ```rust
/// use ai_sdk_core::generate_text::tool_call_repair_function::no_repair;
///
/// let repair_fn = no_repair();
/// // This repair function will always return None, indicating no repair is possible
/// ```
pub fn no_repair() -> ToolCallRepairFunction {
    Box::new(|_options: ToolCallRepairOptions| Box::pin(async move { None }))
}

#[cfg(test)]
mod tests {
    use super::super::Tool;
    use super::*;
    use crate::prompt::message::UserMessage;
    use serde_json::json;

    fn create_test_toolset() -> ToolSet {
        let mut tools = ToolSet::new();
        tools.insert(
            "test_tool".to_string(),
            Tool::function(json!({
                "type": "object",
                "properties": {
                    "param": { "type": "string" }
                }
            })),
        );
        tools
    }

    #[tokio::test]
    async fn test_no_repair_returns_none() {
        let repair_fn = no_repair();

        let options = ToolCallRepairOptions::new(
            Some("You are a helpful assistant".to_string()),
            vec![Message::User(UserMessage::new("Hello"))],
            LanguageModelToolCall::new("call_123", "test_tool", "{}"),
            create_test_toolset(),
            AISDKError::invalid_tool_input("test_tool", "{}", "Invalid input"),
        );

        let result = repair_fn(options).await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_custom_repair_function() {
        // Create a custom repair function that returns a repaired tool call
        let repair_fn: ToolCallRepairFunction = Box::new(|options: ToolCallRepairOptions| {
            Box::pin(async move {
                // Return a repaired version of the tool call
                Some(LanguageModelToolCall::new(
                    options.tool_call.tool_call_id,
                    options.tool_call.tool_name,
                    r#"{"repaired": true}"#,
                ))
            })
        });

        let mut tools = ToolSet::new();
        tools.insert(
            "broken_tool".to_string(),
            Tool::function(json!({
                "type": "object",
                "properties": {}
            })),
        );

        let options = ToolCallRepairOptions::new(
            None,
            vec![],
            LanguageModelToolCall::new("call_456", "broken_tool", "invalid json"),
            tools,
            AISDKError::invalid_tool_input("broken_tool", "invalid json", "Parse error"),
        );

        let result = repair_fn(options).await;
        assert!(result.is_some());

        let repaired = result.unwrap();
        assert_eq!(repaired.tool_call_id, "call_456");
        assert_eq!(repaired.tool_name, "broken_tool");
        assert_eq!(repaired.input, r#"{"repaired": true}"#);
    }

    #[tokio::test]
    async fn test_repair_options_creation() {
        let system = Some("System prompt".to_string());
        let messages = vec![Message::User(UserMessage::new("Test"))];
        let tool_call = LanguageModelToolCall::new("call_789", "my_tool", "{}");

        let mut tools = ToolSet::new();
        tools.insert(
            "my_tool".to_string(),
            Tool::function(json!({"type": "object"})),
        );
        tools.insert(
            "other_tool".to_string(),
            Tool::function(json!({"type": "object"})),
        );

        let tool_names = vec!["my_tool".to_string(), "other_tool".to_string()];
        let error = AISDKError::no_such_tool("unknown_tool", tool_names);

        let options = ToolCallRepairOptions::new(
            system.clone(),
            messages.clone(),
            tool_call.clone(),
            tools,
            error,
        );

        assert_eq!(options.system, system);
        assert_eq!(options.messages.len(), 1);
        assert_eq!(options.tool_call.tool_call_id, "call_789");
        assert_eq!(options.tools.len(), 2);
        assert!(options.tools.contains_key("my_tool"));
        assert!(options.tools.contains_key("other_tool"));
    }

    #[tokio::test]
    async fn test_repair_with_no_such_tool_error() {
        let repair_fn: ToolCallRepairFunction = Box::new(|options: ToolCallRepairOptions| {
            Box::pin(async move {
                // Check if it's a NoSuchTool error
                match &options.error {
                    AISDKError::NoSuchTool { tool_name, .. } => {
                        // Try to find a similar tool name and repair
                        if tool_name == "get_wheather" && options.tools.contains_key("get_weather")
                        {
                            return Some(LanguageModelToolCall::new(
                                options.tool_call.tool_call_id,
                                "get_weather", // Corrected tool name
                                options.tool_call.input,
                            ));
                        }
                    }
                    _ => {}
                }
                None
            })
        });

        let mut tools = ToolSet::new();
        tools.insert(
            "get_weather".to_string(),
            Tool::function(json!({
                "type": "object",
                "properties": {
                    "city": { "type": "string" }
                }
            })),
        );

        let options = ToolCallRepairOptions::new(
            None,
            vec![],
            LanguageModelToolCall::new("call_123", "get_wheather", r#"{"city": "SF"}"#),
            tools,
            AISDKError::no_such_tool("get_wheather", vec!["get_weather".to_string()]),
        );

        let result = repair_fn(options).await;
        assert!(result.is_some());

        let repaired = result.unwrap();
        assert_eq!(repaired.tool_name, "get_weather"); // Corrected typo
        assert_eq!(repaired.input, r#"{"city": "SF"}"#); // Input unchanged
    }

    #[tokio::test]
    async fn test_repair_with_invalid_tool_input_error() {
        let repair_fn: ToolCallRepairFunction = Box::new(|options: ToolCallRepairOptions| {
            Box::pin(async move {
                // Check if it's an InvalidToolInput error
                match &options.error {
                    AISDKError::InvalidToolInput {
                        tool_name,
                        tool_input,
                        ..
                    } => {
                        // Try to repair invalid JSON
                        if tool_input.contains("city") && !tool_input.starts_with("{") {
                            // Add missing braces
                            let repaired_input = format!("{{{}}}", tool_input);
                            return Some(LanguageModelToolCall::new(
                                options.tool_call.tool_call_id,
                                tool_name.clone(),
                                repaired_input,
                            ));
                        }
                    }
                    _ => {}
                }
                None
            })
        });

        let mut tools = ToolSet::new();
        tools.insert(
            "get_weather".to_string(),
            Tool::function(json!({
                "type": "object",
                "properties": {
                    "city": { "type": "string" }
                }
            })),
        );

        let options = ToolCallRepairOptions::new(
            None,
            vec![],
            LanguageModelToolCall::new("call_999", "get_weather", r#""city": "SF""#),
            tools,
            AISDKError::invalid_tool_input("get_weather", r#""city": "SF""#, "Missing braces"),
        );

        let result = repair_fn(options).await;
        assert!(result.is_some());

        let repaired = result.unwrap();
        assert_eq!(repaired.tool_name, "get_weather");
        assert_eq!(repaired.input, r#"{"city": "SF"}"#); // Braces added
    }
}
