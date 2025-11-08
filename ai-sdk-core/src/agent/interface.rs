use crate::generate_text::GenerateText;
use crate::prompt::PromptContent;
use crate::prompt::message::Message;
use crate::stream_text::StreamText;
use crate::tool::ToolSet;

/// Parameters for calling an agent.
///
/// Contains just the prompt - tools are configured in `AgentSettings`.
///
/// # Examples
///
/// ```ignore
/// use ai_sdk_core::prompt::PromptContent;
///
/// // Using a text prompt
/// let params = AgentCallParameters::new(PromptContent::Text {
///     text: "What is the weather?".to_string()
/// });
///
/// // Or use the helper methods
/// let params = AgentCallParameters::from_text("What is the weather?");
/// let params = AgentCallParameters::from_messages(messages);
/// ```
pub struct AgentCallParameters {
    /// The prompt content - either text or messages.
    pub prompt: PromptContent,
}

impl AgentCallParameters {
    /// Creates new agent call parameters with the given prompt content.
    pub fn new(prompt: PromptContent) -> Self {
        Self { prompt }
    }

    /// Creates parameters with a text prompt.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let params = AgentCallParameters::from_text("What is the weather?");
    /// ```
    pub fn from_text(text: impl Into<String>) -> Self {
        Self {
            prompt: PromptContent::Text { text: text.into() },
        }
    }

    /// Creates parameters with messages.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let messages = vec![Message::User(UserMessage::new("Hello"))];
    /// let params = AgentCallParameters::from_messages(messages);
    /// ```
    pub fn from_messages(messages: Vec<Message>) -> Self {
        Self {
            prompt: PromptContent::Messages { messages },
        }
    }
}

/// An interface that defines how agents receive prompts and generate or stream outputs.
///
/// Agents receive a prompt (text or messages) and generate or stream an output
/// that consists of steps, tool calls, data parts, etc.
///
/// You can implement your own agent by implementing the `AgentInterface` trait,
/// or use the `Agent` struct which is the primary concrete implementation.
///
/// # Type Parameters
///
/// * `CallOptions` - Optional type for provider-specific call options (defaults to `()`)
/// * `Output` - The output type (defaults to `Output`)
///
/// # Examples
///
/// ```ignore
/// use ai_sdk_core::agent::{AgentInterface, AgentCallParameters};
///
/// struct MyAgent {
///     id: Option<String>,
///     tools: ToolSet,
/// }
///
/// impl AgentInterface for MyAgent {
///     type CallOptions = ();
///     type Output = Output;
///
///     fn version(&self) -> &'static str {
///         "agent-v1"
///     }
///
///     fn id(&self) -> Option<&str> {
///         self.id.as_deref()
///     }
///
///     fn tools(&self) -> Option<&ToolSet> {
///         Some(&self.tools)
///     }
///
///     fn generate(
///         &self,
///         params: AgentCallParameters,
///     ) -> Result<GenerateText, AISDKError> {
///         // Implementation here
///         todo!()
///     }
///
///     fn stream(
///         &self,
///         params: AgentCallParameters,
///     ) -> Result<StreamText, AISDKError> {
///         // Implementation here
///         todo!()
///     }
/// }
/// ```
pub trait AgentInterface: Send + Sync {
    /// The output type for this agent.
    type Output: Send + Sync;

    /// The specification version of the agent interface.
    /// This enables evolution of the agent interface with backwards compatibility.
    ///
    /// Current version: `"agent-v1"`
    fn version(&self) -> &'static str {
        "agent-v1"
    }

    /// The id of the agent (optional).
    fn id(&self) -> Option<&str>;

    /// The tools that the agent can use.
    /// Returns the tools available to this agent, if any.
    fn tools(&self) -> Option<&ToolSet>;

    /// Creates a GenerateText builder from the agent (non-streaming).
    ///
    /// Returns a configured `GenerateText` builder that can be further customized
    /// before calling `.execute()`.
    ///
    /// # Arguments
    ///
    /// * `params` - The call parameters containing either a prompt or messages
    ///
    /// # Returns
    ///
    /// A `GenerateText` builder configured with agent settings.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let params = AgentCallParameters::from_text("What is the weather?");
    ///
    /// // Get builder and execute
    /// let result = agent.generate(params)?.execute().await?;
    /// println!("Generated text: {}", result.text);
    ///
    /// // Or customize before executing
    /// let result = agent.generate(params)?
    ///     .temperature(0.9)
    ///     .max_output_tokens(200)
    ///     .execute()
    ///     .await?;
    /// ```
    fn generate(
        &self,
        params: AgentCallParameters,
    ) -> Result<GenerateText, crate::error::AISDKError>;

    /// Creates a StreamText builder from the agent (streaming).
    ///
    /// Returns a configured `StreamText` builder that can be further customized
    /// before calling `.execute()`.
    ///
    /// # Arguments
    ///
    /// * `params` - The call parameters containing either a prompt or messages
    ///
    /// # Returns
    ///
    /// A `StreamText` builder configured with agent settings.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let params = AgentCallParameters::from_text("Tell me a story");
    ///
    /// // Get builder and execute
    /// let result = agent.stream(params)?.execute().await?;
    ///
    /// // Stream text deltas
    /// let mut stream = result.text_stream();
    /// while let Some(delta) = stream.next().await {
    ///     print!("{}", delta);
    /// }
    ///
    /// // Or customize before executing
    /// let result = agent.stream(params)?
    ///     .temperature(0.8)
    ///     .on_chunk(|chunk| { /* ... */ })
    ///     .execute()
    ///     .await?;
    /// ```
    fn stream(&self, params: AgentCallParameters) -> Result<StreamText, crate::error::AISDKError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_call_parameters_from_text() {
        let params = AgentCallParameters::from_text("What is the weather?");
        match &params.prompt {
            PromptContent::Text { text } => assert_eq!(text, "What is the weather?"),
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_agent_call_parameters_from_messages() {
        let messages = vec![];
        let params = AgentCallParameters::from_messages(messages);
        match &params.prompt {
            PromptContent::Messages { messages } => assert_eq!(messages.len(), 0),
            _ => panic!("Expected Messages variant"),
        }
    }

    #[test]
    fn test_agent_call_parameters_new() {
        let prompt_content = PromptContent::Text {
            text: "Hello".to_string(),
        };
        let params = AgentCallParameters::new(prompt_content);
        match &params.prompt {
            PromptContent::Text { text } => assert_eq!(text, "Hello"),
            _ => panic!("Expected Text variant"),
        }
    }
}
