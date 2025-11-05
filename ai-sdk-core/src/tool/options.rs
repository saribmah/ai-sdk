use crate::prompt::message::Message;
use tokio_util::sync::CancellationToken;

/// Additional options that are sent into each tool call.
#[derive(Debug, Clone)]
pub struct ToolExecuteOptions {
    /// The ID of the tool call. You can use it e.g. when sending tool-call related information with stream data.
    pub tool_call_id: String,

    /// Messages that were sent to the language model to initiate the response that contained the tool call.
    /// The messages **do not** include the system prompt nor the assistant response that contained the tool call.
    pub messages: Vec<Message>,

    /// An optional abort signal that indicates that the overall operation should be aborted.
    pub abort_signal: Option<CancellationToken>,

    /// Additional context.
    ///
    /// Experimental (can break in patch releases).
    pub experimental_context: Option<serde_json::Value>,
}

impl ToolExecuteOptions {
    /// Creates new tool call options.
    pub fn new(tool_call_id: impl Into<String>, messages: Vec<Message>) -> Self {
        Self {
            tool_call_id: tool_call_id.into(),
            messages,
            abort_signal: None,
            experimental_context: None,
        }
    }

    /// Sets the abort signal.
    pub fn with_abort_signal(mut self, signal: CancellationToken) -> Self {
        self.abort_signal = Some(signal);
        self
    }

    /// Sets the experimental context.
    pub fn with_experimental_context(mut self, context: serde_json::Value) -> Self {
        self.experimental_context = Some(context);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt::message::{SystemMessage, UserMessage};

    #[test]
    fn test_tool_call_options_new() {
        let messages = vec![
            Message::from(SystemMessage::new("You are helpful.")),
            Message::from(UserMessage::new("Hello!")),
        ];

        let options = ToolExecuteOptions::new("call_123", messages.clone());

        assert_eq!(options.tool_call_id, "call_123");
        assert_eq!(options.messages.len(), 2);
        assert!(options.abort_signal.is_none());
        assert!(options.experimental_context.is_none());
    }

    #[test]
    fn test_tool_call_options_with_abort_signal() {
        let messages = vec![Message::from(UserMessage::new("Hello!"))];

        let token = CancellationToken::new();
        let options = ToolExecuteOptions::new("call_123", messages).with_abort_signal(token.clone());

        assert!(options.abort_signal.is_some());
    }

    #[test]
    fn test_tool_call_options_with_experimental_context() {
        let messages = vec![Message::from(UserMessage::new("Hello!"))];

        let context = serde_json::json!({"key": "value"});
        let options =
            ToolExecuteOptions::new("call_123", messages).with_experimental_context(context.clone());

        assert_eq!(options.experimental_context, Some(context));
    }
}
