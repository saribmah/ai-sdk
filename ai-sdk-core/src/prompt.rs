pub mod call_settings;
pub mod convert_to_language_model_prompt;
pub mod create_tool_model_output;
pub mod standardize;

use crate::message::ModelMessage;
use serde::{Deserialize, Serialize};

/// Prompt part of the AI function options.
/// It contains a system message, a simple text prompt, or a list of messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prompt {
    /// System message to include in the prompt. Can be used with any content type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    /// The main content of the prompt - either text or messages
    #[serde(flatten)]
    pub content: PromptContent,
}

/// The content of a prompt - either a simple text string or a list of messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PromptContent {
    /// A simple text prompt
    Text {
        #[serde(rename = "prompt")]
        text: String,
    },
    /// A list of messages
    Messages {
        #[serde(rename = "messages")]
        messages: Vec<ModelMessage>,
    },
}

impl Prompt {
    /// Creates a new prompt with text content
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::prompt::Prompt;
    ///
    /// let prompt = Prompt::text("What is the weather?");
    /// ```
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            system: None,
            content: PromptContent::Text { text: text.into() },
        }
    }

    /// Creates a new prompt with messages
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::prompt::Prompt;
    /// use ai_sdk_core::message::{ModelMessage, UserModelMessage};
    ///
    /// let messages = vec![
    ///     ModelMessage::User(UserModelMessage::new("Hello")),
    /// ];
    /// let prompt = Prompt::messages(messages);
    /// ```
    pub fn messages(messages: Vec<ModelMessage>) -> Self {
        Self {
            system: None,
            content: PromptContent::Messages { messages },
        }
    }

    /// Sets the system message for this prompt
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::prompt::Prompt;
    ///
    /// let prompt = Prompt::text("What is the weather?")
    ///     .with_system("You are a helpful weather assistant");
    /// ```
    pub fn with_system(mut self, system: impl Into<String>) -> Self {
        self.system = Some(system.into());
        self
    }

    /// Gets a reference to the messages in this prompt.
    /// Returns None if the prompt is text-based.
    pub fn get_messages(&self) -> Option<&Vec<ModelMessage>> {
        match &self.content {
            PromptContent::Messages { messages } => Some(messages),
            PromptContent::Text { .. } => None,
        }
    }

    /// Gets a mutable reference to the messages in this prompt.
    /// Returns None if the prompt is text-based.
    pub fn get_messages_mut(&mut self) -> Option<&mut Vec<ModelMessage>> {
        match &mut self.content {
            PromptContent::Messages { messages } => Some(messages),
            PromptContent::Text { .. } => None,
        }
    }

    /// Gets the text content if this is a text prompt.
    /// Returns None if the prompt is message-based.
    pub fn get_text(&self) -> Option<&str> {
        match &self.content {
            PromptContent::Text { text } => Some(text.as_str()),
            PromptContent::Messages { .. } => None,
        }
    }

    /// Converts this prompt into its constituent parts
    pub fn into_parts(self) -> (Option<String>, PromptContent) {
        (self.system, self.content)
    }
}

impl From<String> for Prompt {
    fn from(text: String) -> Self {
        Self::text(text)
    }
}

impl From<&str> for Prompt {
    fn from(text: &str) -> Self {
        Self::text(text)
    }
}

impl From<Vec<ModelMessage>> for Prompt {
    fn from(messages: Vec<ModelMessage>) -> Self {
        Self::messages(messages)
    }
}

impl PromptContent {
    /// Checks if this content is text-based
    pub fn is_text(&self) -> bool {
        matches!(self, PromptContent::Text { .. })
    }

    /// Checks if this content is message-based
    pub fn is_messages(&self) -> bool {
        matches!(self, PromptContent::Messages { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::{UserContent, UserModelMessage};

    #[test]
    fn test_text_prompt() {
        let prompt = Prompt::text("Hello, world!");
        assert_eq!(prompt.get_text(), Some("Hello, world!"));
        assert!(prompt.get_messages().is_none());
        assert!(prompt.system.is_none());
    }

    #[test]
    fn test_text_prompt_with_system() {
        let prompt = Prompt::text("Hello, world!").with_system("You are helpful");
        assert_eq!(prompt.get_text(), Some("Hello, world!"));
        assert_eq!(prompt.system, Some("You are helpful".to_string()));
    }

    #[test]
    fn test_messages_prompt() {
        let messages = vec![ModelMessage::User(UserModelMessage {
            role: "user".to_string(),
            content: UserContent::Text("Hello".to_string()),
            provider_options: None,
        })];
        let prompt = Prompt::messages(messages.clone());
        assert_eq!(prompt.get_messages(), Some(&messages));
        assert!(prompt.get_text().is_none());
    }

    #[test]
    fn test_messages_prompt_with_system() {
        let messages = vec![ModelMessage::User(UserModelMessage {
            role: "user".to_string(),
            content: UserContent::Text("Hello".to_string()),
            provider_options: None,
        })];
        let prompt = Prompt::messages(messages.clone()).with_system("You are helpful");
        assert_eq!(prompt.get_messages(), Some(&messages));
        assert_eq!(prompt.system, Some("You are helpful".to_string()));
    }

    #[test]
    fn test_from_string() {
        let prompt: Prompt = "Hello".to_string().into();
        assert_eq!(prompt.get_text(), Some("Hello"));
    }

    #[test]
    fn test_from_str() {
        let prompt: Prompt = "Hello".into();
        assert_eq!(prompt.get_text(), Some("Hello"));
    }

    #[test]
    fn test_from_messages() {
        let messages = vec![ModelMessage::User(UserModelMessage {
            role: "user".to_string(),
            content: UserContent::Text("Hello".to_string()),
            provider_options: None,
        })];
        let prompt: Prompt = messages.clone().into();
        assert_eq!(prompt.get_messages(), Some(&messages));
    }

    #[test]
    fn test_prompt_content_type_checks() {
        let text_content = PromptContent::Text {
            text: "Hello".to_string(),
        };
        assert!(text_content.is_text());
        assert!(!text_content.is_messages());

        let messages_content = PromptContent::Messages { messages: vec![] };
        assert!(!messages_content.is_text());
        assert!(messages_content.is_messages());
    }

    #[test]
    fn test_into_parts() {
        let prompt = Prompt::text("Hello").with_system("System message");
        let (system, content) = prompt.into_parts();
        assert_eq!(system, Some("System message".to_string()));
        assert!(content.is_text());
    }

    #[test]
    fn test_serialize_text_prompt() {
        let prompt = Prompt::text("Hello, world!");
        let json = serde_json::to_string(&prompt).unwrap();
        assert!(json.contains("\"prompt\":\"Hello, world!\""));
        assert!(!json.contains("system"));
    }

    #[test]
    fn test_serialize_text_prompt_with_system() {
        let prompt = Prompt::text("Hello").with_system("You are helpful");
        let json = serde_json::to_string(&prompt).unwrap();
        assert!(json.contains("\"prompt\":\"Hello\""));
        assert!(json.contains("\"system\":\"You are helpful\""));
    }

    #[test]
    fn test_serialize_messages_prompt() {
        let messages = vec![ModelMessage::User(UserModelMessage {
            role: "user".to_string(),
            content: UserContent::Text("Hello".to_string()),
            provider_options: None,
        })];
        let prompt = Prompt::messages(messages);
        let json = serde_json::to_string(&prompt).unwrap();
        assert!(json.contains("\"messages\""));
        assert!(!json.contains("system"));
    }

    #[test]
    fn test_deserialize_text_prompt() {
        let json = r#"{"prompt":"Hello, world!"}"#;
        let prompt: Prompt = serde_json::from_str(json).unwrap();
        assert_eq!(prompt.get_text(), Some("Hello, world!"));
        assert!(prompt.system.is_none());
    }

    #[test]
    fn test_deserialize_text_prompt_with_system() {
        let json = r#"{"system":"You are helpful","prompt":"Hello"}"#;
        let prompt: Prompt = serde_json::from_str(json).unwrap();
        assert_eq!(prompt.get_text(), Some("Hello"));
        assert_eq!(prompt.system, Some("You are helpful".to_string()));
    }

    #[test]
    fn test_deserialize_messages_prompt() {
        let json = r#"{"messages":[{"role":"user","content":"Hello"}]}"#;
        let prompt: Prompt = serde_json::from_str(json).unwrap();
        assert!(prompt.get_messages().is_some());
        assert_eq!(prompt.get_messages().unwrap().len(), 1);
    }
}
