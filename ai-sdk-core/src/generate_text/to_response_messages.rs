use crate::message::ModelMessage;
use crate::message::content_parts::tool_result::ToolResultPart;
use crate::message::model::assistant::AssistantContentPart;
use crate::message::model::tool::ToolContentPart;
use crate::message::model::{AssistantModelMessage, ToolModelMessage};
use crate::prompt::create_tool_model_output::{ErrorMode, create_tool_model_output};
use serde_json::Value;

use super::content_part::ContentPart;
use super::tool_set::ToolSet;

/// Converts the result of a `generate_text` call to a list of response messages.
///
/// This function takes the content parts from a generation result and converts them
/// into properly formatted assistant and tool messages that can be used in subsequent
/// conversations.
///
/// # Arguments
///
/// * `content` - Array of content parts from the generation result
/// * `tools` - Optional tool set for formatting tool outputs
///
/// # Returns
///
/// A vector of model messages (assistant and/or tool messages)
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::generate_text::{to_response_messages, ContentPart};
///
/// let content = vec![/* content parts */];
/// let messages = to_response_messages(content, None);
/// ```
pub fn to_response_messages(
    content: Vec<ContentPart<Value, Value>>,
    tools: Option<&ToolSet>,
) -> Vec<ModelMessage> {
    let mut response_messages: Vec<ModelMessage> = Vec::new();

    // Filter and map content for assistant message
    let assistant_content: Vec<AssistantContentPart> = content
        .iter()
        .filter(|part| {
            // Filter out source parts
            !matches!(part, ContentPart::Source(_))
        })
        .filter(|part| {
            // Filter out tool-result and tool-error if NOT provider-executed
            match part {
                ContentPart::ToolResult(result) => {
                    // Include only if provider executed
                    match result {
                        super::tool_result::TypedToolResult::Static(r) => {
                            r.provider_executed == Some(true)
                        }
                        super::tool_result::TypedToolResult::Dynamic(r) => {
                            r.provider_executed == Some(true)
                        }
                    }
                }
                ContentPart::ToolError(error) => {
                    // Include only if provider executed
                    match error {
                        super::tool_error::TypedToolError::Static(e) => {
                            e.provider_executed == Some(true)
                        }
                        super::tool_error::TypedToolError::Dynamic(e) => {
                            e.provider_executed == Some(true)
                        }
                    }
                }
                _ => true,
            }
        })
        .filter(|part| {
            // Filter out empty text
            match part {
                ContentPart::Text(text_output) => !text_output.text.is_empty(),
                _ => true,
            }
        })
        .filter_map(|part| {
            match part {
                ContentPart::Text(text_output) => Some(AssistantContentPart::Text(
                    crate::message::content_parts::TextPart::new(text_output.text.clone()),
                )),
                ContentPart::Reasoning(reasoning_output) => Some(AssistantContentPart::Reasoning(
                    crate::message::content_parts::ReasoningPart::new(
                        reasoning_output.text.clone(),
                    ),
                )),
                ContentPart::ToolCall(tool_call) => {
                    let (tool_call_id, tool_name, input, provider_executed) = match tool_call {
                        super::tool_call::TypedToolCall::Static(call) => (
                            call.tool_call_id.clone(),
                            call.tool_name.clone(),
                            call.input.clone(),
                            call.provider_executed,
                        ),
                        super::tool_call::TypedToolCall::Dynamic(call) => (
                            call.tool_call_id.clone(),
                            call.tool_name.clone(),
                            call.input.clone(),
                            call.provider_executed,
                        ),
                    };

                    Some(AssistantContentPart::ToolCall(
                        crate::message::content_parts::ToolCallPart::new(
                            tool_call_id,
                            tool_name,
                            input,
                        )
                        .with_provider_executed(provider_executed.unwrap_or(false)),
                    ))
                }
                ContentPart::ToolResult(result) => {
                    let (tool_call_id, tool_name, output) = match result {
                        super::tool_result::TypedToolResult::Static(r) => (
                            r.tool_call_id.clone(),
                            r.tool_name.clone(),
                            r.output.clone(),
                        ),
                        super::tool_result::TypedToolResult::Dynamic(r) => (
                            r.tool_call_id.clone(),
                            r.tool_name.clone(),
                            r.output.clone(),
                        ),
                    };

                    // Get the tool from the toolset
                    let tool = tools.and_then(|ts| ts.get(&tool_name));

                    let tool_output = create_tool_model_output(output, tool, ErrorMode::None);

                    Some(AssistantContentPart::ToolResult(ToolResultPart {
                        tool_call_id,
                        tool_name,
                        output: tool_output,
                        provider_options: None,
                    }))
                }
                ContentPart::ToolError(error) => {
                    let (tool_call_id, tool_name, error_value) = match error {
                        super::tool_error::TypedToolError::Static(e) => {
                            (e.tool_call_id.clone(), e.tool_name.clone(), e.error.clone())
                        }
                        super::tool_error::TypedToolError::Dynamic(e) => {
                            (e.tool_call_id.clone(), e.tool_name.clone(), e.error.clone())
                        }
                    };

                    // Get the tool from the toolset
                    let tool = tools.and_then(|ts| ts.get(&tool_name));

                    let tool_output = create_tool_model_output(error_value, tool, ErrorMode::Json);

                    Some(AssistantContentPart::ToolResult(ToolResultPart {
                        tool_call_id,
                        tool_name,
                        output: tool_output,
                        provider_options: None,
                    }))
                }
                _ => None,
            }
        })
        .collect();

    // Add assistant message if there's content
    if !assistant_content.is_empty() {
        response_messages.push(ModelMessage::Assistant(AssistantModelMessage::with_parts(
            assistant_content,
        )));
    }

    // Filter and map content for tool message (client-executed tool results)
    let tool_content: Vec<ToolResultPart> = content
        .iter()
        .filter(|part| matches!(part, ContentPart::ToolResult(_) | ContentPart::ToolError(_)))
        .filter(|part| {
            // Include only if NOT provider-executed
            match part {
                ContentPart::ToolResult(result) => match result {
                    super::tool_result::TypedToolResult::Static(r) => {
                        r.provider_executed != Some(true)
                    }
                    super::tool_result::TypedToolResult::Dynamic(r) => {
                        r.provider_executed != Some(true)
                    }
                },
                ContentPart::ToolError(error) => match error {
                    super::tool_error::TypedToolError::Static(e) => {
                        e.provider_executed != Some(true)
                    }
                    super::tool_error::TypedToolError::Dynamic(e) => {
                        e.provider_executed != Some(true)
                    }
                },
                _ => false,
            }
        })
        .map(|part| match part {
            ContentPart::ToolResult(result) => {
                let (tool_call_id, tool_name, output) = match result {
                    super::tool_result::TypedToolResult::Static(r) => (
                        r.tool_call_id.clone(),
                        r.tool_name.clone(),
                        r.output.clone(),
                    ),
                    super::tool_result::TypedToolResult::Dynamic(r) => (
                        r.tool_call_id.clone(),
                        r.tool_name.clone(),
                        r.output.clone(),
                    ),
                };

                let tool = tools.and_then(|ts| ts.get(&tool_name));
                let tool_output = create_tool_model_output(output, tool, ErrorMode::None);

                ToolResultPart {
                    tool_call_id,
                    tool_name,
                    output: tool_output,
                    provider_options: None,
                }
            }
            ContentPart::ToolError(error) => {
                let (tool_call_id, tool_name, error_value) = match error {
                    super::tool_error::TypedToolError::Static(e) => {
                        (e.tool_call_id.clone(), e.tool_name.clone(), e.error.clone())
                    }
                    super::tool_error::TypedToolError::Dynamic(e) => {
                        (e.tool_call_id.clone(), e.tool_name.clone(), e.error.clone())
                    }
                };

                let tool = tools.and_then(|ts| ts.get(&tool_name));
                let tool_output = create_tool_model_output(error_value, tool, ErrorMode::Text);

                ToolResultPart {
                    tool_call_id,
                    tool_name,
                    output: tool_output,
                    provider_options: None,
                }
            }
            _ => unreachable!(),
        })
        .collect();

    // Add tool message if there's content
    if !tool_content.is_empty() {
        // Convert Vec<ToolResultPart> to Vec<ToolContentPart>
        let tool_content_parts: Vec<ToolContentPart> = tool_content
            .into_iter()
            .map(|part| ToolContentPart::ToolResult(part))
            .collect();

        response_messages.push(ModelMessage::Tool(ToolModelMessage::new(
            tool_content_parts,
        )));
    }

    response_messages
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate_text::{
        ReasoningOutput, StaticToolCall, StaticToolError, StaticToolResult, TextOutput,
        TypedToolCall, TypedToolError, TypedToolResult,
    };
    use serde_json::json;

    #[test]
    fn test_to_response_messages_empty() {
        let content = vec![];
        let messages = to_response_messages(content, None);
        assert!(messages.is_empty());
    }

    #[test]
    fn test_to_response_messages_text_only() {
        let content = vec![ContentPart::Text(TextOutput::new("Hello, world!"))];
        let messages = to_response_messages(content, None);

        assert_eq!(messages.len(), 1);
        match &messages[0] {
            ModelMessage::Assistant(msg) => {
                assert_eq!(msg.role, "assistant");
            }
            _ => panic!("Expected assistant message"),
        }
    }

    #[test]
    fn test_to_response_messages_filters_empty_text() {
        let content = vec![
            ContentPart::Text(TextOutput::new("")),
            ContentPart::Text(TextOutput::new("Hello")),
        ];
        let messages = to_response_messages(content, None);

        assert_eq!(messages.len(), 1);
    }

    #[test]
    fn test_to_response_messages_with_reasoning() {
        let content = vec![
            ContentPart::Reasoning(ReasoningOutput::new("Thinking...")),
            ContentPart::Text(TextOutput::new("Answer")),
        ];
        let messages = to_response_messages(content, None);

        assert_eq!(messages.len(), 1);
        match &messages[0] {
            ModelMessage::Assistant(msg) => {
                assert_eq!(msg.role, "assistant");
            }
            _ => panic!("Expected assistant message"),
        }
    }

    #[test]
    fn test_to_response_messages_with_tool_call() {
        let tool_call = StaticToolCall::new("call_1", "test_tool", json!({"arg": "value"}));
        let content = vec![ContentPart::ToolCall(TypedToolCall::Static(tool_call))];
        let messages = to_response_messages(content, None);

        assert_eq!(messages.len(), 1);
        match &messages[0] {
            ModelMessage::Assistant(msg) => {
                assert_eq!(msg.role, "assistant");
            }
            _ => panic!("Expected assistant message"),
        }
    }

    #[test]
    fn test_to_response_messages_provider_executed_tool_result() {
        let tool_result = StaticToolResult::new(
            "call_1",
            "test_tool",
            json!({"arg": "value"}),
            json!({"result": "success"}),
        )
        .with_provider_executed(true);

        let content = vec![ContentPart::ToolResult(TypedToolResult::Static(
            tool_result,
        ))];
        let messages = to_response_messages(content, None);

        assert_eq!(messages.len(), 1);
        match &messages[0] {
            ModelMessage::Assistant(_) => {
                // Provider-executed results go in assistant message
            }
            _ => panic!("Expected assistant message"),
        }
    }

    #[test]
    fn test_to_response_messages_client_executed_tool_result() {
        let tool_result = StaticToolResult::new(
            "call_1",
            "test_tool",
            json!({"arg": "value"}),
            json!({"result": "success"}),
        );
        // provider_executed defaults to None/false

        let content = vec![ContentPart::ToolResult(TypedToolResult::Static(
            tool_result,
        ))];
        let messages = to_response_messages(content, None);

        assert_eq!(messages.len(), 1);
        match &messages[0] {
            ModelMessage::Tool(_) => {
                // Client-executed results go in tool message
            }
            _ => panic!("Expected tool message"),
        }
    }

    #[test]
    fn test_to_response_messages_client_executed_tool_error() {
        let tool_error = StaticToolError::new(
            "call_1",
            "test_tool",
            json!({"arg": "value"}),
            json!({"error": "failed"}),
        );

        let content = vec![ContentPart::ToolError(TypedToolError::Static(tool_error))];
        let messages = to_response_messages(content, None);

        assert_eq!(messages.len(), 1);
        match &messages[0] {
            ModelMessage::Tool(_) => {
                // Client-executed errors go in tool message
            }
            _ => panic!("Expected tool message"),
        }
    }

    #[test]
    fn test_to_response_messages_mixed_content() {
        let tool_call = StaticToolCall::new("call_1", "test_tool", json!({"arg": "value"}));
        let tool_result = StaticToolResult::new(
            "call_1",
            "test_tool",
            json!({"arg": "value"}),
            json!({"result": "success"}),
        );

        let content = vec![
            ContentPart::Text(TextOutput::new("Before call")),
            ContentPart::ToolCall(TypedToolCall::Static(tool_call)),
            ContentPart::ToolResult(TypedToolResult::Static(tool_result)),
        ];
        let messages = to_response_messages(content, None);

        assert_eq!(messages.len(), 2);
        match &messages[0] {
            ModelMessage::Assistant(_) => {
                // Text and tool call in assistant message
            }
            _ => panic!("Expected assistant message first"),
        }
        match &messages[1] {
            ModelMessage::Tool(_) => {
                // Client-executed result in tool message
            }
            _ => panic!("Expected tool message second"),
        }
    }
}
