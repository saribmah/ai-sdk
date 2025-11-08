use crate::generate_text::GenerateTextResult;
use crate::prompt::message::Message;
use crate::stream_text::StreamTextResult;
use crate::tool::ToolSet;
use std::fmt::Debug;
use std::future::Future;

/// Parameters for calling an agent with either a text prompt or messages.
///
/// This struct ensures that you provide either `prompt` or `messages`, but not both.
///
/// # Type Parameters
///
/// * `CallOptions` - Optional type for provider-specific call options
///
/// # Examples
///
/// ```ignore
/// // Using a text prompt
/// let params = AgentCallParameters {
///     prompt: Some(AgentPrompt::Text("What is the weather?".to_string())),
///     messages: None,
///     options: None,
/// };
///
/// // Using messages
/// let params = AgentCallParameters {
///     prompt: None,
///     messages: Some(vec![Message::User(UserMessage::new("Hello"))]),
///     options: None,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct AgentCallParameters<CallOptions = ()> {
    /// A prompt - either text or messages. Mutually exclusive with `messages`.
    pub prompt: Option<AgentPrompt>,

    /// A list of messages. Mutually exclusive with `prompt`.
    pub messages: Option<Vec<Message>>,

    /// Optional provider-specific call options.
    pub options: Option<CallOptions>,
}

/// The prompt content for an agent call.
#[derive(Debug, Clone)]
pub enum AgentPrompt {
    /// A simple text prompt
    Text(String),
    /// A list of messages
    Messages(Vec<Message>),
}

impl<CallOptions> AgentCallParameters<CallOptions> {
    /// Creates parameters with a text prompt.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let params = AgentCallParameters::with_prompt("What is the weather?");
    /// ```
    pub fn with_prompt(prompt: impl Into<String>) -> Self {
        Self {
            prompt: Some(AgentPrompt::Text(prompt.into())),
            messages: None,
            options: None,
        }
    }

    /// Creates parameters with messages.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let messages = vec![Message::User(UserMessage::new("Hello"))];
    /// let params = AgentCallParameters::with_messages(messages);
    /// ```
    pub fn with_messages(messages: Vec<Message>) -> Self {
        Self {
            prompt: None,
            messages: Some(messages),
            options: None,
        }
    }

    /// Sets the call options.
    pub fn with_options(mut self, options: CallOptions) -> Self {
        self.options = Some(options);
        self
    }

    /// Validates that either prompt or messages is set, but not both.
    pub fn validate(&self) -> Result<(), String> {
        match (&self.prompt, &self.messages) {
            (None, None) => Err("Either prompt or messages must be provided".to_string()),
            (Some(_), Some(_)) => {
                Err("Cannot provide both prompt and messages, use only one".to_string())
            }
            _ => Ok(()),
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
///     fn tools(&self) -> &ToolSet {
///         &self.tools
///     }
///
///     async fn generate(
///         &self,
///         params: AgentCallParameters<Self::CallOptions>,
///     ) -> Result<GenerateTextResult, AISDKError> {
///         // Implementation here
///         todo!()
///     }
///
///     async fn stream(
///         &self,
///         params: AgentCallParameters<Self::CallOptions>,
///     ) -> Result<StreamTextResult, AISDKError> {
///         // Implementation here
///         todo!()
///     }
/// }
/// ```
pub trait AgentInterface: Send + Sync {
    /// The type for call options (provider-specific settings).
    /// Use `()` if no options are needed.
    type CallOptions: Send + Sync;

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
    fn tools(&self) -> &ToolSet;

    /// Generates an output from the agent (non-streaming).
    ///
    /// # Arguments
    ///
    /// * `params` - The call parameters containing either a prompt or messages
    ///
    /// # Returns
    ///
    /// A future that resolves to a `GenerateTextResult` containing the generated output.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let params = AgentCallParameters::with_prompt("What is the weather?");
    /// let result = agent.generate(params).await?;
    /// println!("Generated text: {}", result.text);
    /// ```
    fn generate(
        &self,
        params: AgentCallParameters<Self::CallOptions>,
    ) -> impl Future<Output = Result<GenerateTextResult, crate::error::AISDKError>> + Send;

    /// Streams an output from the agent (streaming).
    ///
    /// # Arguments
    ///
    /// * `params` - The call parameters containing either a prompt or messages
    ///
    /// # Returns
    ///
    /// A future that resolves to a `StreamTextResult` containing the streamed output.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let params = AgentCallParameters::with_prompt("Tell me a story");
    /// let result = agent.stream(params).await?;
    ///
    /// // Stream text deltas
    /// let mut stream = result.text_stream();
    /// while let Some(delta) = stream.next().await {
    ///     print!("{}", delta);
    /// }
    /// ```
    fn stream(
        &self,
        params: AgentCallParameters<Self::CallOptions>,
    ) -> impl Future<Output = Result<StreamTextResult, crate::error::AISDKError>> + Send;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_call_parameters_with_prompt() {
        let params = AgentCallParameters::<()>::with_prompt("What is the weather?");
        assert!(params.prompt.is_some());
        assert!(params.messages.is_none());
        assert!(params.options.is_none());
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_agent_call_parameters_with_messages() {
        let messages = vec![];
        let params = AgentCallParameters::<()>::with_messages(messages);
        assert!(params.prompt.is_none());
        assert!(params.messages.is_some());
        assert!(params.options.is_none());
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_agent_call_parameters_with_options() {
        let params = AgentCallParameters::with_prompt("Hello").with_options("custom_option");
        assert!(params.options.is_some());
        assert_eq!(params.options.unwrap(), "custom_option");
    }

    #[test]
    fn test_agent_call_parameters_validation_empty() {
        let params = AgentCallParameters::<()> {
            prompt: None,
            messages: None,
            options: None,
        };
        assert!(params.validate().is_err());
        assert_eq!(
            params.validate().unwrap_err(),
            "Either prompt or messages must be provided"
        );
    }

    #[test]
    fn test_agent_call_parameters_validation_both() {
        let params = AgentCallParameters::<()> {
            prompt: Some(AgentPrompt::Text("Hello".to_string())),
            messages: Some(vec![]),
            options: None,
        };
        assert!(params.validate().is_err());
        assert_eq!(
            params.validate().unwrap_err(),
            "Cannot provide both prompt and messages, use only one"
        );
    }

    #[test]
    fn test_agent_prompt_variants() {
        let text_prompt = AgentPrompt::Text("Hello".to_string());
        match text_prompt {
            AgentPrompt::Text(s) => assert_eq!(s, "Hello"),
            _ => panic!("Expected Text variant"),
        }

        let messages_prompt = AgentPrompt::Messages(vec![]);
        match messages_prompt {
            AgentPrompt::Messages(msgs) => assert_eq!(msgs.len(), 0),
            _ => panic!("Expected Messages variant"),
        }
    }
}
