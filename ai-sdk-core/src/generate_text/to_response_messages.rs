use crate::ResponseMessage;
use crate::output::Output;
use crate::prompt::create_tool_model_output::{ErrorMode, create_tool_model_output};
use crate::prompt::message::assistant::AssistantContentPart;
use crate::prompt::message::content_parts::tool_result::ToolResultPart;
use crate::prompt::message::tool::ToolContentPart;
use crate::prompt::message::{AssistantMessage, ToolMessage};
use crate::tool::ToolSet;

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
/// use ai_sdk_core::generate_text::{to_response_messages, Output};
///
/// let content = vec![/* content parts */];
/// let messages = to_response_messages(content, None);
/// ```
pub fn to_response_messages(content: Vec<Output>, tools: Option<&ToolSet>) -> Vec<ResponseMessage> {
    let mut response_messages: Vec<ResponseMessage> = Vec::new();

    // Filter and map content for assistant message
    let assistant_content: Vec<AssistantContentPart> = content
        .iter()
        .filter(|part| {
            // Filter out source parts
            !matches!(part, Output::Source(_))
        })
        .filter(|part| {
            // Filter out tool-result and tool-error if NOT provider-executed
            match part {
                Output::ToolResult(result) => {
                    // Include only if provider executed
                    result.provider_executed == Some(true)
                }
                Output::ToolError(error) => {
                    // Include only if provider executed
                    error.provider_executed == Some(true)
                }
                _ => true,
            }
        })
        .filter(|part| {
            // Filter out empty text
            match part {
                Output::Text(text_output) => !text_output.text.is_empty(),
                _ => true,
            }
        })
        .filter_map(|part| {
            match part {
                Output::Text(text_output) => Some(AssistantContentPart::Text(
                    crate::prompt::message::content_parts::TextPart::new(text_output.text.clone()),
                )),
                Output::Reasoning(reasoning_output) => Some(AssistantContentPart::Reasoning(
                    crate::prompt::message::content_parts::ReasoningPart::new(
                        reasoning_output.text.clone(),
                    ),
                )),
                Output::ToolCall(tool_call) => Some(AssistantContentPart::ToolCall(
                    crate::prompt::message::content_parts::ToolCallPart::new(
                        tool_call.tool_call_id.clone(),
                        tool_call.tool_name.clone(),
                        tool_call.input.clone(),
                    )
                    .with_provider_executed(tool_call.provider_executed.unwrap_or(false)),
                )),
                Output::ToolResult(result) => {
                    // Get the tool from the toolset
                    let tool = tools.and_then(|ts| ts.get(&result.tool_name));

                    let tool_output =
                        create_tool_model_output(result.output.clone(), tool, ErrorMode::None);

                    Some(AssistantContentPart::ToolResult(ToolResultPart {
                        tool_call_id: result.tool_call_id.clone(),
                        tool_name: result.tool_name.clone(),
                        output: tool_output,
                        provider_options: None,
                    }))
                }
                Output::ToolError(error) => {
                    // Get the tool from the toolset
                    let tool = tools.and_then(|ts| ts.get(&error.tool_name));

                    let tool_output =
                        create_tool_model_output(error.error.clone(), tool, ErrorMode::Json);

                    Some(AssistantContentPart::ToolResult(ToolResultPart {
                        tool_call_id: error.tool_call_id.clone(),
                        tool_name: error.tool_name.clone(),
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
        response_messages.push(ResponseMessage::Assistant(AssistantMessage::with_parts(
            assistant_content,
        )));
    }

    // Filter and map content for tool message (client-executed tool results)
    let tool_content: Vec<ToolResultPart> = content
        .iter()
        .filter(|part| matches!(part, Output::ToolResult(_) | Output::ToolError(_)))
        .filter(|part| {
            // Include only if NOT provider-executed
            match part {
                Output::ToolResult(result) => result.provider_executed != Some(true),
                Output::ToolError(error) => error.provider_executed != Some(true),
                _ => false,
            }
        })
        .map(|part| match part {
            Output::ToolResult(result) => {
                let tool = tools.and_then(|ts| ts.get(&result.tool_name));
                let tool_output =
                    create_tool_model_output(result.output.clone(), tool, ErrorMode::None);

                ToolResultPart {
                    tool_call_id: result.tool_call_id.clone(),
                    tool_name: result.tool_name.clone(),
                    output: tool_output,
                    provider_options: None,
                }
            }
            Output::ToolError(error) => {
                let tool = tools.and_then(|ts| ts.get(&error.tool_name));
                let tool_output =
                    create_tool_model_output(error.error.clone(), tool, ErrorMode::Text);

                ToolResultPart {
                    tool_call_id: error.tool_call_id.clone(),
                    tool_name: error.tool_name.clone(),
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
            .map(ToolContentPart::ToolResult)
            .collect();

        response_messages.push(ResponseMessage::Tool(ToolMessage::new(tool_content_parts)));
    }

    response_messages
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate_text::{ReasoningOutput, TextOutput};
    use crate::tool::{ToolCall, ToolError, ToolResult};
    use serde_json::json;

    #[test]
    fn test_to_response_messages_empty() {
        let content = vec![];
        let messages = to_response_messages(content, None);
        assert!(messages.is_empty());
    }

    #[test]
    fn test_to_response_messages_text_only() {
        let content = vec![Output::Text(TextOutput::new("Hello, world!"))];
        let messages = to_response_messages(content, None);

        assert_eq!(messages.len(), 1);
        match &messages[0] {
            ResponseMessage::Assistant(msg) => {
                assert_eq!(msg.role, "assistant");
            }
            _ => panic!("Expected assistant message"),
        }
    }

    #[test]
    fn test_to_response_messages_filters_empty_text() {
        let content = vec![
            Output::Text(TextOutput::new("")),
            Output::Text(TextOutput::new("Hello")),
        ];
        let messages = to_response_messages(content, None);

        assert_eq!(messages.len(), 1);
    }

    #[test]
    fn test_to_response_messages_with_reasoning() {
        let content = vec![
            Output::Reasoning(ReasoningOutput::new("Thinking...")),
            Output::Text(TextOutput::new("Answer")),
        ];
        let messages = to_response_messages(content, None);

        assert_eq!(messages.len(), 1);
        match &messages[0] {
            ResponseMessage::Assistant(msg) => {
                assert_eq!(msg.role, "assistant");
            }
            _ => panic!("Expected assistant message"),
        }
    }

    #[test]
    fn test_to_response_messages_with_tool_call() {
        let tool_call = ToolCall::new("call_1", "test_tool", json!({"arg": "value"}));
        let content = vec![Output::ToolCall(tool_call)];
        let messages = to_response_messages(content, None);

        assert_eq!(messages.len(), 1);
        match &messages[0] {
            ResponseMessage::Assistant(msg) => {
                assert_eq!(msg.role, "assistant");
            }
            _ => panic!("Expected assistant message"),
        }
    }

    #[test]
    fn test_to_response_messages_provider_executed_tool_result() {
        let tool_result = ToolResult::new(
            "call_1",
            "test_tool",
            json!({"arg": "value"}),
            json!({"result": "success"}),
        )
        .with_provider_executed(true);

        let content = vec![Output::ToolResult(tool_result)];
        let messages = to_response_messages(content, None);

        assert_eq!(messages.len(), 1);
        match &messages[0] {
            ResponseMessage::Assistant(_) => {
                // Provider-executed results go in assistant message
            }
            _ => panic!("Expected assistant message"),
        }
    }

    #[test]
    fn test_to_response_messages_client_executed_tool_result() {
        let tool_result = ToolResult::new(
            "call_1",
            "test_tool",
            json!({"arg": "value"}),
            json!({"result": "success"}),
        );
        // provider_executed defaults to None/false

        let content = vec![Output::ToolResult(tool_result)];
        let messages = to_response_messages(content, None);

        assert_eq!(messages.len(), 1);
        match &messages[0] {
            ResponseMessage::Tool(_) => {
                // Client-executed results go in tool message
            }
            _ => panic!("Expected tool message"),
        }
    }

    #[test]
    fn test_to_response_messages_client_executed_tool_error() {
        let tool_error = ToolError::new(
            "call_1",
            "test_tool",
            json!({"arg": "value"}),
            json!({"error": "failed"}),
        );

        let content = vec![Output::ToolError(tool_error)];
        let messages = to_response_messages(content, None);

        assert_eq!(messages.len(), 1);
        match &messages[0] {
            ResponseMessage::Tool(_) => {
                // Client-executed errors go in tool message
            }
            _ => panic!("Expected tool message"),
        }
    }

    #[test]
    fn test_to_response_messages_mixed_content() {
        let tool_call = ToolCall::new("call_1", "test_tool", json!({"arg": "value"}));
        let tool_result = ToolResult::new(
            "call_1",
            "test_tool",
            json!({"arg": "value"}),
            json!({"result": "success"}),
        );

        let content = vec![
            Output::Text(TextOutput::new("Before call")),
            Output::ToolCall(tool_call),
            Output::ToolResult(tool_result),
        ];
        let messages = to_response_messages(content, None);

        assert_eq!(messages.len(), 2);
        match &messages[0] {
            ResponseMessage::Assistant(_) => {
                // Text and tool call in assistant message
            }
            _ => panic!("Expected assistant message first"),
        }
        match &messages[1] {
            ResponseMessage::Tool(_) => {
                // Client-executed result in tool message
            }
            _ => panic!("Expected tool message second"),
        }
    }
}
