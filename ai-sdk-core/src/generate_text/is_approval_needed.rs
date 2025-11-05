use crate::prompt::message::Message;
use crate::prompt::message::tool::definition::{NeedsApproval, Tool};
use crate::prompt::message::tool::options::ToolExecuteOptions;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

/// Checks if a tool call needs approval before execution.
///
/// This function evaluates the tool's `needs_approval` configuration to determine
/// if user approval is required before executing the tool call.
///
/// # Arguments
///
/// * `tool` - The tool definition
/// * `tool_call_id` - The ID of the tool call
/// * `input` - The input to the tool call
/// * `messages` - The conversation messages for context
/// * `experimental_context` - Optional experimental context data
///
/// # Returns
///
/// Returns `true` if approval is needed, `false` otherwise.
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::generate_text::is_approval_needed;
///
/// let needs_approval = is_approval_needed(
///     &tool,
///     "call_123",
///     input,
///     messages,
///     None,
/// ).await;
///
/// if needs_approval {
///     println!("This tool call requires user approval");
/// }
/// ```
pub async fn is_approval_needed<INPUT, OUTPUT>(
    tool: &Tool<INPUT, OUTPUT>,
    tool_call_id: impl Into<String>,
    input: INPUT,
    messages: Vec<Message>,
    experimental_context: Option<Value>,
) -> bool
where
    INPUT: Serialize + DeserializeOwned + Send + Clone + 'static,
    OUTPUT: Serialize + DeserializeOwned + Send + 'static,
{
    match &tool.needs_approval {
        // Tool does not need approval
        NeedsApproval::No => false,

        // Tool always needs approval
        NeedsApproval::Yes => true,

        // Tool needs approval based on a function
        NeedsApproval::Function(needs_approval_fn) => {
            // Create tool call options
            let mut options = ToolExecuteOptions::new(tool_call_id, messages);

            if let Some(context) = experimental_context {
                options = options.with_experimental_context(context);
            }

            // Call the approval function
            needs_approval_fn(input, options).await
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt::message::tool::definition::{NeedsApproval, Tool};
    use serde_json::json;

    #[tokio::test]
    async fn test_no_approval_needed() {
        let tool: Tool<Value, Value> = Tool::function(json!({
            "type": "object",
            "properties": {
                "city": {"type": "string"}
            }
        }));

        let result =
            is_approval_needed(&tool, "call_123", json!({"city": "SF"}), vec![], None).await;

        assert!(!result);
    }

    #[tokio::test]
    async fn test_always_needs_approval() {
        let mut tool: Tool<Value, Value> = Tool::function(json!({
            "type": "object",
            "properties": {
                "city": {"type": "string"}
            }
        }));
        tool.needs_approval = NeedsApproval::Yes;

        let result =
            is_approval_needed(&tool, "call_123", json!({"city": "SF"}), vec![], None).await;

        assert!(result);
    }

    #[tokio::test]
    async fn test_conditional_approval_returns_true() {
        let mut tool: Tool<Value, Value> = Tool::function(json!({
            "type": "object",
            "properties": {
                "action": {"type": "string"}
            }
        }));

        // Set up a function that requires approval for "delete" actions
        tool.needs_approval = NeedsApproval::Function(Box::new(|input: Value, _options| {
            Box::pin(async move {
                if let Some(action) = input.get("action").and_then(|v| v.as_str()) {
                    action == "delete"
                } else {
                    false
                }
            })
        }));

        let result =
            is_approval_needed(&tool, "call_123", json!({"action": "delete"}), vec![], None).await;

        assert!(result);
    }

    #[tokio::test]
    async fn test_conditional_approval_returns_false() {
        let mut tool: Tool<Value, Value> = Tool::function(json!({
            "type": "object",
            "properties": {
                "action": {"type": "string"}
            }
        }));

        // Set up a function that requires approval for "delete" actions
        tool.needs_approval = NeedsApproval::Function(Box::new(|input: Value, _options| {
            Box::pin(async move {
                if let Some(action) = input.get("action").and_then(|v| v.as_str()) {
                    action == "delete"
                } else {
                    false
                }
            })
        }));

        let result =
            is_approval_needed(&tool, "call_123", json!({"action": "read"}), vec![], None).await;

        assert!(!result);
    }

    #[tokio::test]
    async fn test_with_experimental_context() {
        let mut tool: Tool<Value, Value> = Tool::function(json!({
            "type": "object",
            "properties": {}
        }));

        // Set up a function that checks experimental context
        tool.needs_approval = NeedsApproval::Function(Box::new(|_input: Value, options| {
            Box::pin(async move {
                options
                    .experimental_context
                    .and_then(|ctx| ctx.get("requireApproval").and_then(|v| v.as_bool()))
                    .unwrap_or(false)
            })
        }));

        let result = is_approval_needed(
            &tool,
            "call_123",
            json!({}),
            vec![],
            Some(json!({"requireApproval": true})),
        )
        .await;

        assert!(result);
    }

    #[tokio::test]
    async fn test_with_messages() {
        use crate::prompt::message::user::UserMessage;

        let mut tool: Tool<Value, Value> = Tool::function(json!({
            "type": "object",
            "properties": {}
        }));

        // Set up a function that checks if messages are present
        tool.needs_approval = NeedsApproval::Function(Box::new(|_input: Value, options| {
            Box::pin(async move { !options.messages.is_empty() })
        }));

        let messages = vec![Message::User(UserMessage::new("Hello"))];

        let result = is_approval_needed(&tool, "call_123", json!({}), messages, None).await;

        assert!(result);
    }
}
