use crate::error::AISDKError;
use crate::prompt::message::{Message, UserContent, UserMessage};
use crate::prompt::{Prompt, PromptContent};
use serde::{Deserialize, Serialize};

/// A standardized prompt that always has messages.
/// This is the normalized format used internally after processing a Prompt.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StandardizedPrompt {
    /// System message (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    /// Messages (always present in standardized form)
    pub messages: Vec<Message>,
}

impl StandardizedPrompt {
    /// Create a new standardized prompt
    pub fn new(messages: Vec<Message>) -> Self {
        Self {
            system: None,
            messages,
        }
    }

    /// Create a standardized prompt with a system message
    pub fn with_system(system: impl Into<String>, messages: Vec<Message>) -> Self {
        Self {
            system: Some(system.into()),
            messages,
        }
    }
}

impl From<Prompt> for StandardizedPrompt {
    /// Convert a Prompt into a StandardizedPrompt.
    /// If the prompt contains text, it will be converted into a UserModelMessage.
    fn from(prompt: Prompt) -> Self {
        let messages = match prompt.content {
            PromptContent::Text { text } => {
                // Convert text to a user message
                vec![Message::User(UserMessage {
                    role: "user".to_string(),
                    content: UserContent::Text(text),
                    provider_options: None,
                })]
            }
            PromptContent::Messages { messages } => messages,
        };

        Self {
            system: prompt.system,
            messages,
        }
    }
}

/// Convert a Prompt into a StandardizedPrompt
pub fn standardize_prompt(prompt: Prompt) -> StandardizedPrompt {
    prompt.into()
}

/// Validates and converts a Prompt into a StandardizedPrompt.
///
/// This function performs validation similar to the TypeScript implementation:
/// - Ensures messages are not empty
/// - Returns detailed error messages for invalid prompts
///
/// # Errors
///
/// Returns `AISDKError::InvalidPrompt` if:
/// - The prompt contains an empty messages array
///
/// # Examples
///
/// ```
/// use ai_sdk_core::prompt::{Prompt, standardize::validate_and_standardize};
///
/// let prompt = Prompt::text("Hello, world!");
/// let standardized = validate_and_standardize(prompt).unwrap();
/// assert_eq!(standardized.messages.len(), 1);
/// ```
pub fn validate_and_standardize(prompt: Prompt) -> Result<StandardizedPrompt, AISDKError> {
    // Extract messages based on content type
    let messages = match &prompt.content {
        PromptContent::Text { text } => {
            // Convert text to a user message
            vec![Message::User(UserMessage {
                role: "user".to_string(),
                content: UserContent::Text(text.clone()),
                provider_options: None,
            })]
        }
        PromptContent::Messages { messages } => messages.clone(),
    };

    // Validate that messages are not empty
    if messages.is_empty() {
        return Err(AISDKError::invalid_prompt("messages must not be empty"));
    }

    Ok(StandardizedPrompt {
        system: prompt.system,
        messages,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt::message::UserContent;

    #[test]
    fn test_standardized_prompt_new() {
        let messages = vec![Message::User(UserMessage {
            role: "user".to_string(),
            content: UserContent::Text("Hello".to_string()),
            provider_options: None,
        })];
        let standardized = StandardizedPrompt::new(messages.clone());
        assert_eq!(standardized.messages, messages);
        assert!(standardized.system.is_none());
    }

    #[test]
    fn test_standardized_prompt_with_system() {
        let messages = vec![Message::User(UserMessage {
            role: "user".to_string(),
            content: UserContent::Text("Hello".to_string()),
            provider_options: None,
        })];
        let standardized = StandardizedPrompt::with_system("System message", messages.clone());
        assert_eq!(standardized.messages, messages);
        assert_eq!(standardized.system, Some("System message".to_string()));
    }

    #[test]
    fn test_from_prompt_text() {
        let prompt = Prompt::text("What is the weather?");
        let standardized: StandardizedPrompt = prompt.into();

        assert_eq!(standardized.messages.len(), 1);
        assert!(standardized.system.is_none());

        match &standardized.messages[0] {
            Message::User(msg) => match &msg.content {
                UserContent::Text(text) => {
                    assert_eq!(text, "What is the weather?");
                }
                _ => panic!("Expected text content"),
            },
            _ => panic!("Expected user message"),
        }
    }

    #[test]
    fn test_from_prompt_text_with_system() {
        let prompt = Prompt::text("What is the weather?").with_system("You are helpful");
        let standardized: StandardizedPrompt = prompt.into();

        assert_eq!(standardized.messages.len(), 1);
        assert_eq!(standardized.system, Some("You are helpful".to_string()));

        match &standardized.messages[0] {
            Message::User(msg) => match &msg.content {
                UserContent::Text(text) => {
                    assert_eq!(text, "What is the weather?");
                }
                _ => panic!("Expected text content"),
            },
            _ => panic!("Expected user message"),
        }
    }

    #[test]
    fn test_from_prompt_messages() {
        let messages = vec![
            Message::User(UserMessage {
                role: "user".to_string(),
                content: UserContent::Text("Hello".to_string()),
                provider_options: None,
            }),
            Message::User(UserMessage {
                role: "user".to_string(),
                content: UserContent::Text("How are you?".to_string()),
                provider_options: None,
            }),
        ];
        let prompt = Prompt::messages(messages.clone());
        let standardized: StandardizedPrompt = prompt.into();

        assert_eq!(standardized.messages.len(), 2);
        assert_eq!(standardized.messages, messages);
        assert!(standardized.system.is_none());
    }

    #[test]
    fn test_from_prompt_messages_with_system() {
        let messages = vec![Message::User(UserMessage {
            role: "user".to_string(),
            content: UserContent::Text("Hello".to_string()),
            provider_options: None,
        })];
        let prompt = Prompt::messages(messages.clone()).with_system("System message");
        let standardized: StandardizedPrompt = prompt.into();

        assert_eq!(standardized.messages, messages);
        assert_eq!(standardized.system, Some("System message".to_string()));
    }

    #[test]
    fn test_standardize_prompt_function() {
        let prompt = Prompt::text("Test");
        let standardized = standardize_prompt(prompt);

        assert_eq!(standardized.messages.len(), 1);
        match &standardized.messages[0] {
            Message::User(msg) => match &msg.content {
                UserContent::Text(text) => {
                    assert_eq!(text, "Test");
                }
                _ => panic!("Expected text content"),
            },
            _ => panic!("Expected user message"),
        }
    }

    #[test]
    fn test_serialize_standardized_prompt() {
        let messages = vec![Message::User(UserMessage {
            role: "user".to_string(),
            content: UserContent::Text("Hello".to_string()),
            provider_options: None,
        })];
        let standardized = StandardizedPrompt::new(messages);
        let json = serde_json::to_string(&standardized).unwrap();
        assert!(json.contains("\"messages\""));
        assert!(!json.contains("system"));
    }

    #[test]
    fn test_serialize_standardized_prompt_with_system() {
        let messages = vec![Message::User(UserMessage {
            role: "user".to_string(),
            content: UserContent::Text("Hello".to_string()),
            provider_options: None,
        })];
        let standardized = StandardizedPrompt::with_system("System", messages);
        let json = serde_json::to_string(&standardized).unwrap();
        assert!(json.contains("\"messages\""));
        assert!(json.contains("\"system\":\"System\""));
    }

    #[test]
    fn test_deserialize_standardized_prompt() {
        let json = r#"{"messages":[{"role":"user","content":"Hello"}]}"#;
        let standardized: StandardizedPrompt = serde_json::from_str(json).unwrap();
        assert_eq!(standardized.messages.len(), 1);
        assert!(standardized.system.is_none());
    }

    #[test]
    fn test_deserialize_standardized_prompt_with_system() {
        let json = r#"{"system":"System message","messages":[{"role":"user","content":"Hello"}]}"#;
        let standardized: StandardizedPrompt = serde_json::from_str(json).unwrap();
        assert_eq!(standardized.messages.len(), 1);
        assert_eq!(standardized.system, Some("System message".to_string()));
    }

    // Validation tests

    #[test]
    fn test_validate_and_standardize_text_prompt() {
        let prompt = Prompt::text("Hello, world!");
        let result = validate_and_standardize(prompt);
        assert!(result.is_ok());

        let standardized = result.unwrap();
        assert_eq!(standardized.messages.len(), 1);
        match &standardized.messages[0] {
            Message::User(msg) => match &msg.content {
                UserContent::Text(text) => {
                    assert_eq!(text, "Hello, world!");
                }
                _ => panic!("Expected text content"),
            },
            _ => panic!("Expected user message"),
        }
    }

    #[test]
    fn test_validate_and_standardize_messages_prompt() {
        let messages = vec![
            Message::User(UserMessage {
                role: "user".to_string(),
                content: UserContent::Text("Hello".to_string()),
                provider_options: None,
            }),
            Message::User(UserMessage {
                role: "user".to_string(),
                content: UserContent::Text("How are you?".to_string()),
                provider_options: None,
            }),
        ];
        let prompt = Prompt::messages(messages.clone());
        let result = validate_and_standardize(prompt);
        assert!(result.is_ok());

        let standardized = result.unwrap();
        assert_eq!(standardized.messages.len(), 2);
        assert_eq!(standardized.messages, messages);
    }

    #[test]
    fn test_validate_and_standardize_with_system() {
        let prompt = Prompt::text("Hello").with_system("You are helpful");
        let result = validate_and_standardize(prompt);
        assert!(result.is_ok());

        let standardized = result.unwrap();
        assert_eq!(standardized.system, Some("You are helpful".to_string()));
        assert_eq!(standardized.messages.len(), 1);
    }

    #[test]
    fn test_validate_and_standardize_empty_messages_fails() {
        let prompt = Prompt::messages(vec![]);
        let result = validate_and_standardize(prompt);
        assert!(result.is_err());

        match result {
            Err(AISDKError::InvalidPrompt { message }) => {
                assert_eq!(message, "messages must not be empty");
            }
            _ => panic!("Expected InvalidPrompt error"),
        }
    }
}
