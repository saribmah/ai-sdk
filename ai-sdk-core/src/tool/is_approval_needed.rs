use ai_sdk_provider_utils::message::Message;
use ai_sdk_provider_utils::tool::{NeedsApproval, Tool, ToolExecuteOptions};
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
/// use ai_sdk_core::tool::{is_approval_needed, Tool};
/// use serde_json::json;
/// # use ai_sdk_core::prompt::message::Message;
/// # use tokio::runtime::Runtime;
/// # fn example(tool: Tool) {
/// # Runtime::new().unwrap().block_on(async {
/// # let input = json!({});
/// # let messages: Vec<Message> = vec![];
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
/// # });
/// # }
/// ```
pub async fn is_approval_needed(
    tool: &Tool,
    tool_call_id: impl Into<String>,
    input: Value,
    messages: Vec<Message>,
    experimental_context: Option<Value>,
) -> bool {
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
    use ai_sdk_provider_utils::tool::{NeedsApproval, Tool};
    use serde_json::json;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_no_approval_needed() {
        let tool = Tool::function(json!({
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
        let mut tool = Tool::function(json!({
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
        let mut tool = Tool::function(json!({
            "type": "object",
            "properties": {
                "action": {"type": "string"}
            }
        }));

        // Set up a function that requires approval for "delete" actions
        tool.needs_approval = NeedsApproval::Function(Arc::new(|input: Value, _options| {
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
        let mut tool = Tool::function(json!({
            "type": "object",
            "properties": {
                "action": {"type": "string"}
            }
        }));

        // Set up a function that requires approval for "delete" actions
        tool.needs_approval = NeedsApproval::Function(Arc::new(|input: Value, _options| {
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
        let mut tool = Tool::function(json!({
            "type": "object",
            "properties": {}
        }));

        // Set up a function that checks experimental context
        tool.needs_approval = NeedsApproval::Function(Arc::new(|_input: Value, options| {
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
        use ai_sdk_provider_utils::message::user::UserMessage;

        let mut tool = Tool::function(json!({
            "type": "object",
            "properties": {}
        }));

        // Set up a function that checks if messages are present
        tool.needs_approval = NeedsApproval::Function(Arc::new(|_input: Value, options| {
            Box::pin(async move { !options.messages.is_empty() })
        }));

        let messages = vec![Message::User(UserMessage::new("Hello"))];

        let result = is_approval_needed(&tool, "call_123", json!({}), messages, None).await;

        assert!(result);
    }
}
