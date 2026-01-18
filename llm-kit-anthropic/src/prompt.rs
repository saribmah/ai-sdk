pub mod message;
pub mod tool;

use serde::{Deserialize, Serialize};

use self::message::AnthropicMessage;
use self::message::content::text::AnthropicTextContent;

/// Anthropic Messages API prompt.
///
/// Represents a complete prompt for the Anthropic Messages API, including
/// optional system messages and a list of conversation messages.
///
/// # Example
///
/// ```
/// use llm_kit_anthropic::prompt::AnthropicMessagesPrompt;
/// use llm_kit_anthropic::prompt::message::AnthropicMessage;
/// use llm_kit_anthropic::prompt::message::user::AnthropicUserMessage;
/// use llm_kit_anthropic::prompt::message::content::text::AnthropicTextContent;
///
/// // Create a prompt with a system message and user message
/// let prompt = AnthropicMessagesPrompt::new(vec![
///     AnthropicMessage::User(AnthropicUserMessage::from_text("Hello, Claude!"))
/// ])
/// .with_system(vec![AnthropicTextContent::new("You are a helpful assistant.")]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicMessagesPrompt {
    /// Optional system messages
    ///
    /// System messages provide context and instructions to the model.
    /// They are processed before the conversation messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<Vec<AnthropicTextContent>>,

    /// Conversation messages
    ///
    /// An array of messages representing the conversation between the user and assistant.
    /// Must contain at least one message.
    pub messages: Vec<AnthropicMessage>,
}

impl AnthropicMessagesPrompt {
    /// Creates a new messages prompt.
    ///
    /// # Arguments
    ///
    /// * `messages` - The conversation messages
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::AnthropicMessagesPrompt;
    /// use llm_kit_anthropic::prompt::message::AnthropicMessage;
    /// use llm_kit_anthropic::prompt::message::user::AnthropicUserMessage;
    ///
    /// let prompt = AnthropicMessagesPrompt::new(vec![
    ///     AnthropicMessage::User(AnthropicUserMessage::from_text("What is Rust?"))
    /// ]);
    /// ```
    pub fn new(messages: Vec<AnthropicMessage>) -> Self {
        Self {
            system: None,
            messages,
        }
    }

    /// Creates a new messages prompt with a single user message.
    ///
    /// # Arguments
    ///
    /// * `text` - The user message text
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::AnthropicMessagesPrompt;
    ///
    /// let prompt = AnthropicMessagesPrompt::from_user_text("Hello, Claude!");
    /// ```
    pub fn from_user_text(text: impl Into<String>) -> Self {
        use self::message::user::AnthropicUserMessage;

        Self::new(vec![AnthropicMessage::User(
            AnthropicUserMessage::from_text(text),
        )])
    }

    /// Sets the system messages.
    ///
    /// # Arguments
    ///
    /// * `system` - The system messages
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::AnthropicMessagesPrompt;
    /// use llm_kit_anthropic::prompt::message::content::text::AnthropicTextContent;
    ///
    /// let prompt = AnthropicMessagesPrompt::from_user_text("Hello!")
    ///     .with_system(vec![AnthropicTextContent::new("You are a helpful assistant.")]);
    /// ```
    pub fn with_system(mut self, system: Vec<AnthropicTextContent>) -> Self {
        self.system = Some(system);
        self
    }

    /// Sets a single system message.
    ///
    /// # Arguments
    ///
    /// * `text` - The system message text
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::AnthropicMessagesPrompt;
    ///
    /// let prompt = AnthropicMessagesPrompt::from_user_text("Hello!")
    ///     .with_system_text("You are a helpful assistant.");
    /// ```
    pub fn with_system_text(mut self, text: impl Into<String>) -> Self {
        self.system = Some(vec![AnthropicTextContent::new(text)]);
        self
    }

    /// Removes the system messages.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::AnthropicMessagesPrompt;
    ///
    /// let prompt = AnthropicMessagesPrompt::from_user_text("Hello!")
    ///     .with_system_text("You are helpful.")
    ///     .without_system();
    ///
    /// assert!(prompt.system.is_none());
    /// ```
    pub fn without_system(mut self) -> Self {
        self.system = None;
        self
    }

    /// Adds a message to the conversation.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to add
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::AnthropicMessagesPrompt;
    /// use llm_kit_anthropic::prompt::message::AnthropicMessage;
    /// use llm_kit_anthropic::prompt::message::assistant::AnthropicAssistantMessage;
    ///
    /// let prompt = AnthropicMessagesPrompt::from_user_text("Hello!")
    ///     .with_message(AnthropicMessage::Assistant(
    ///         AnthropicAssistantMessage::text("Hi there!")
    ///     ));
    /// ```
    pub fn with_message(mut self, message: AnthropicMessage) -> Self {
        self.messages.push(message);
        self
    }

    /// Adds a user message to the conversation.
    ///
    /// # Arguments
    ///
    /// * `text` - The user message text
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::AnthropicMessagesPrompt;
    ///
    /// let prompt = AnthropicMessagesPrompt::from_user_text("Hello!")
    ///     .with_user_message("How are you?");
    /// ```
    pub fn with_user_message(mut self, text: impl Into<String>) -> Self {
        use self::message::user::AnthropicUserMessage;

        self.messages
            .push(AnthropicMessage::User(AnthropicUserMessage::from_text(
                text,
            )));
        self
    }

    /// Adds an assistant message to the conversation.
    ///
    /// # Arguments
    ///
    /// * `text` - The assistant message text
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::AnthropicMessagesPrompt;
    ///
    /// let prompt = AnthropicMessagesPrompt::from_user_text("Hello!")
    ///     .with_assistant_message("Hi! How can I help you?");
    /// ```
    pub fn with_assistant_message(mut self, text: impl Into<String>) -> Self {
        use self::message::assistant::AnthropicAssistantMessage;

        self.messages.push(AnthropicMessage::Assistant(
            AnthropicAssistantMessage::text(text),
        ));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt::message::assistant::AnthropicAssistantMessage;
    use crate::prompt::message::user::AnthropicUserMessage;
    use serde_json::json;

    #[test]
    fn test_new() {
        let prompt = AnthropicMessagesPrompt::new(vec![AnthropicMessage::User(
            AnthropicUserMessage::from_text("Hello"),
        )]);

        assert!(prompt.system.is_none());
        assert_eq!(prompt.messages.len(), 1);
    }

    #[test]
    fn test_from_user_text() {
        let prompt = AnthropicMessagesPrompt::from_user_text("Hello, Claude!");

        assert!(prompt.system.is_none());
        assert_eq!(prompt.messages.len(), 1);
        match &prompt.messages[0] {
            AnthropicMessage::User(msg) => {
                assert_eq!(msg.content.len(), 1);
            }
            _ => panic!("Expected User message"),
        }
    }

    #[test]
    fn test_with_system() {
        let prompt = AnthropicMessagesPrompt::from_user_text("Hello")
            .with_system(vec![AnthropicTextContent::new("You are helpful.")]);

        assert!(prompt.system.is_some());
        assert_eq!(prompt.system.as_ref().unwrap().len(), 1);
        assert_eq!(prompt.system.as_ref().unwrap()[0].text, "You are helpful.");
    }

    #[test]
    fn test_with_system_text() {
        let prompt = AnthropicMessagesPrompt::from_user_text("Hello")
            .with_system_text("You are a helpful assistant.");

        assert!(prompt.system.is_some());
        assert_eq!(prompt.system.as_ref().unwrap().len(), 1);
        assert_eq!(
            prompt.system.as_ref().unwrap()[0].text,
            "You are a helpful assistant."
        );
    }

    #[test]
    fn test_without_system() {
        let prompt = AnthropicMessagesPrompt::from_user_text("Hello")
            .with_system_text("System message")
            .without_system();

        assert!(prompt.system.is_none());
    }

    #[test]
    fn test_with_message() {
        let prompt = AnthropicMessagesPrompt::from_user_text("Hello").with_message(
            AnthropicMessage::Assistant(AnthropicAssistantMessage::text("Hi there!")),
        );

        assert_eq!(prompt.messages.len(), 2);
        match &prompt.messages[1] {
            AnthropicMessage::Assistant(_) => (),
            _ => panic!("Expected Assistant message"),
        }
    }

    #[test]
    fn test_with_user_message() {
        let prompt =
            AnthropicMessagesPrompt::from_user_text("Hello").with_user_message("How are you?");

        assert_eq!(prompt.messages.len(), 2);
        match &prompt.messages[1] {
            AnthropicMessage::User(_) => (),
            _ => panic!("Expected User message"),
        }
    }

    #[test]
    fn test_with_assistant_message() {
        let prompt = AnthropicMessagesPrompt::from_user_text("Hello")
            .with_assistant_message("I'm doing great!");

        assert_eq!(prompt.messages.len(), 2);
        match &prompt.messages[1] {
            AnthropicMessage::Assistant(_) => (),
            _ => panic!("Expected Assistant message"),
        }
    }

    #[test]
    fn test_builder_chaining() {
        let prompt = AnthropicMessagesPrompt::from_user_text("Hello")
            .with_system_text("You are helpful.")
            .with_assistant_message("Hi!")
            .with_user_message("How are you?")
            .with_assistant_message("Great!");

        assert!(prompt.system.is_some());
        assert_eq!(prompt.messages.len(), 4);
    }

    #[test]
    fn test_serialize_without_system() {
        let prompt = AnthropicMessagesPrompt::from_user_text("Hello, Claude!");
        let json = serde_json::to_value(&prompt).unwrap();

        assert_eq!(
            json,
            json!({
                "messages": [
                    {
                        "role": "user",
                        "content": [
                            {
                                "type": "text",
                                "text": "Hello, Claude!"
                            }
                        ]
                    }
                ]
            })
        );
    }

    #[test]
    fn test_serialize_with_system() {
        let prompt = AnthropicMessagesPrompt::from_user_text("Hello!")
            .with_system_text("You are a helpful assistant.");
        let json = serde_json::to_value(&prompt).unwrap();

        assert_eq!(
            json,
            json!({
                "system": [
                    {
                        "type": "text",
                        "text": "You are a helpful assistant."
                    }
                ],
                "messages": [
                    {
                        "role": "user",
                        "content": [
                            {
                                "type": "text",
                                "text": "Hello!"
                            }
                        ]
                    }
                ]
            })
        );
    }

    #[test]
    fn test_serialize_conversation() {
        let prompt = AnthropicMessagesPrompt::from_user_text("Hello!")
            .with_assistant_message("Hi there!")
            .with_user_message("How are you?");
        let json = serde_json::to_value(&prompt).unwrap();

        assert_eq!(
            json,
            json!({
                "messages": [
                    {
                        "role": "user",
                        "content": [
                            {
                                "type": "text",
                                "text": "Hello!"
                            }
                        ]
                    },
                    {
                        "role": "assistant",
                        "content": [
                            {
                                "type": "text",
                                "text": "Hi there!"
                            }
                        ]
                    },
                    {
                        "role": "user",
                        "content": [
                            {
                                "type": "text",
                                "text": "How are you?"
                            }
                        ]
                    }
                ]
            })
        );
    }

    #[test]
    fn test_deserialize_without_system() {
        let json = json!({
            "messages": [
                {
                    "role": "user",
                    "content": [
                        {
                            "type": "text",
                            "text": "Hello, Claude!"
                        }
                    ]
                }
            ]
        });

        let prompt: AnthropicMessagesPrompt = serde_json::from_value(json).unwrap();

        assert!(prompt.system.is_none());
        assert_eq!(prompt.messages.len(), 1);
    }

    #[test]
    fn test_deserialize_with_system() {
        let json = json!({
            "system": [
                {
                    "type": "text",
                    "text": "You are helpful."
                }
            ],
            "messages": [
                {
                    "role": "user",
                    "content": [
                        {
                            "type": "text",
                            "text": "Hello!"
                        }
                    ]
                }
            ]
        });

        let prompt: AnthropicMessagesPrompt = serde_json::from_value(json).unwrap();

        assert!(prompt.system.is_some());
        assert_eq!(prompt.system.as_ref().unwrap().len(), 1);
        assert_eq!(prompt.messages.len(), 1);
    }

    #[test]
    fn test_deserialize_conversation() {
        let json = json!({
            "messages": [
                {
                    "role": "user",
                    "content": [
                        {
                            "type": "text",
                            "text": "Hello!"
                        }
                    ]
                },
                {
                    "role": "assistant",
                    "content": [
                        {
                            "type": "text",
                            "text": "Hi there!"
                        }
                    ]
                }
            ]
        });

        let prompt: AnthropicMessagesPrompt = serde_json::from_value(json).unwrap();

        assert_eq!(prompt.messages.len(), 2);
    }

    #[test]
    fn test_roundtrip_serialization() {
        let original = AnthropicMessagesPrompt::from_user_text("Hello")
            .with_system_text("You are helpful.")
            .with_assistant_message("Hi!")
            .with_user_message("How are you?");

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicMessagesPrompt = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_equality() {
        let prompt1 = AnthropicMessagesPrompt::from_user_text("Hello");
        let prompt2 = AnthropicMessagesPrompt::from_user_text("Hello");
        let prompt3 = AnthropicMessagesPrompt::from_user_text("Goodbye");

        assert_eq!(prompt1, prompt2);
        assert_ne!(prompt1, prompt3);
    }

    #[test]
    fn test_clone() {
        let prompt1 =
            AnthropicMessagesPrompt::from_user_text("Hello").with_system_text("You are helpful.");
        let prompt2 = prompt1.clone();

        assert_eq!(prompt1, prompt2);
    }

    #[test]
    fn test_empty_messages() {
        let prompt = AnthropicMessagesPrompt::new(vec![]);

        assert!(prompt.messages.is_empty());
    }

    #[test]
    fn test_multiple_system_messages() {
        let prompt = AnthropicMessagesPrompt::from_user_text("Hello").with_system(vec![
            AnthropicTextContent::new("System message 1"),
            AnthropicTextContent::new("System message 2"),
        ]);

        assert_eq!(prompt.system.as_ref().unwrap().len(), 2);
    }
}
