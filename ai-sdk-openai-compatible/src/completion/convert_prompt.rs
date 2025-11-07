use crate::completion::prompt::OpenAICompatibleCompletionPrompt;
use ai_sdk_provider::language_model::prompt::{
    LanguageModelAssistantMessagePart, LanguageModelMessage, LanguageModelPrompt,
    LanguageModelUserMessagePart,
};

#[cfg(test)]
use ai_sdk_provider::language_model::prompt::message::{
    LanguageModelAssistantMessage, LanguageModelSystemMessage, LanguageModelToolMessage,
    LanguageModelUserMessage,
};

#[cfg(test)]
use ai_sdk_provider::language_model::prompt::message::parts::{
    LanguageModelTextPart, LanguageModelToolCallPart,
};

/// Converts a language model prompt to OpenAI-compatible completion format.
///
/// # Arguments
///
/// * `prompt` - The language model prompt to convert
/// * `user` - The prefix for user messages (defaults to "user")
/// * `assistant` - The prefix for assistant messages (defaults to "assistant")
///
/// # Returns
///
/// A `CompletionPrompt` containing the formatted prompt text and stop sequences
///
/// # Errors
///
/// Returns an error if:
/// - A system message appears after the first message
/// - Tool call messages are encountered (unsupported)
/// - Tool messages are encountered (unsupported)
///
/// # Example
///
/// ```ignore
/// let prompt = vec![
///     Message::System {
///         content: "You are a helpful assistant.".to_string(),
///         provider_options: None,
///     },
///     Message::User {
///         content: vec![UserMessagePart::Text {
///             text: "Hello!".to_string(),
///             provider_options: None,
///         }],
///         provider_options: None,
///     },
/// ];
///
/// let result = convert_to_openai_compatible_completion_prompt(
///     prompt,
///     Some("user"),
///     Some("assistant")
/// )?;
/// ```
pub fn convert_to_openai_compatible_completion_prompt(
    mut prompt: LanguageModelPrompt,
    user: Option<&str>,
    assistant: Option<&str>,
) -> Result<OpenAICompatibleCompletionPrompt, String> {
    let user_prefix = user.unwrap_or("user");
    let assistant_prefix = assistant.unwrap_or("assistant");

    let mut text = String::new();

    // If first message is a system message, add it to the text
    if let Some(LanguageModelMessage::System(sys)) = prompt.first() {
        text.push_str(&sys.content);
        text.push_str("\n\n");
        prompt = prompt.into_iter().skip(1).collect();
    }

    // Process remaining messages
    for message in prompt {
        match message {
            LanguageModelMessage::System(sys) => {
                return Err(format!(
                    "Unexpected system message in prompt: {}",
                    sys.content
                ));
            }

            LanguageModelMessage::User(user) => {
                let user_message: String = user
                    .content
                    .into_iter()
                    .filter_map(|part| match part {
                        LanguageModelUserMessagePart::Text(text_part) => Some(text_part.text),
                        LanguageModelUserMessagePart::File(_) => None,
                    })
                    .collect::<Vec<_>>()
                    .join("");

                text.push_str(&format!("{}:\n{}\n\n", user_prefix, user_message));
            }

            LanguageModelMessage::Assistant(asst) => {
                let mut assistant_message = String::new();

                for part in asst.content {
                    match part {
                        LanguageModelAssistantMessagePart::Text(text_part) => {
                            assistant_message.push_str(&text_part.text);
                        }
                        LanguageModelAssistantMessagePart::ToolCall(_) => {
                            return Err("Unsupported functionality: tool-call messages".to_string());
                        }
                        // Ignore other assistant message types
                        _ => {}
                    }
                }

                text.push_str(&format!("{}:\n{}\n\n", assistant_prefix, assistant_message));
            }

            LanguageModelMessage::Tool(_) => {
                return Err("Unsupported functionality: tool messages".to_string());
            }
        }
    }

    // Add assistant message prefix
    text.push_str(&format!("{}:\n", assistant_prefix));

    Ok(OpenAICompatibleCompletionPrompt {
        prompt: text,
        stop_sequences: Some(vec![format!("\n{}:", user_prefix)]),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_user_message() {
        let prompt = vec![LanguageModelMessage::User(LanguageModelUserMessage::new(
            vec![LanguageModelUserMessagePart::Text(
                LanguageModelTextPart::new("Hello!"),
            )],
        ))];

        let result = convert_to_openai_compatible_completion_prompt(prompt, None, None).unwrap();

        assert_eq!(result.prompt, "user:\nHello!\n\nassistant:\n");
        assert_eq!(result.stop_sequences, Some(vec!["\nuser:".to_string()]));
    }

    #[test]
    fn test_system_message_first() {
        let prompt = vec![
            LanguageModelMessage::System(LanguageModelSystemMessage::new(
                "You are a helpful assistant.".to_string(),
            )),
            LanguageModelMessage::User(LanguageModelUserMessage::new(vec![
                LanguageModelUserMessagePart::Text(LanguageModelTextPart::new("Hello!")),
            ])),
        ];

        let result = convert_to_openai_compatible_completion_prompt(prompt, None, None).unwrap();

        assert_eq!(
            result.prompt,
            "You are a helpful assistant.\n\nuser:\nHello!\n\nassistant:\n"
        );
        assert_eq!(result.stop_sequences, Some(vec!["\nuser:".to_string()]));
    }

    #[test]
    fn test_system_message_not_first_errors() {
        let prompt = vec![
            LanguageModelMessage::User(LanguageModelUserMessage::new(vec![
                LanguageModelUserMessagePart::Text(LanguageModelTextPart::new("Hello!")),
            ])),
            LanguageModelMessage::System(LanguageModelSystemMessage::new(
                "You are a helpful assistant.".to_string(),
            )),
        ];

        let result = convert_to_openai_compatible_completion_prompt(prompt, None, None);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unexpected system message"));
    }

    #[test]
    fn test_conversation() {
        let prompt = vec![
            LanguageModelMessage::User(LanguageModelUserMessage::new(vec![
                LanguageModelUserMessagePart::Text(LanguageModelTextPart::new("What is 2+2?")),
            ])),
            LanguageModelMessage::Assistant(LanguageModelAssistantMessage::new(vec![
                LanguageModelAssistantMessagePart::Text(LanguageModelTextPart::new(
                    "The answer is 4.",
                )),
            ])),
            LanguageModelMessage::User(LanguageModelUserMessage::new(vec![
                LanguageModelUserMessagePart::Text(LanguageModelTextPart::new("What about 3+3?")),
            ])),
        ];

        let result = convert_to_openai_compatible_completion_prompt(prompt, None, None).unwrap();

        assert_eq!(
            result.prompt,
            "user:\nWhat is 2+2?\n\nassistant:\nThe answer is 4.\n\nuser:\nWhat about 3+3?\n\nassistant:\n"
        );
    }

    #[test]
    fn test_custom_prefixes() {
        let prompt = vec![LanguageModelMessage::User(LanguageModelUserMessage::new(
            vec![LanguageModelUserMessagePart::Text(
                LanguageModelTextPart::new("Hello!"),
            )],
        ))];

        let result =
            convert_to_openai_compatible_completion_prompt(prompt, Some("Human"), Some("AI"))
                .unwrap();

        assert_eq!(result.prompt, "Human:\nHello!\n\nAI:\n");
        assert_eq!(result.stop_sequences, Some(vec!["\nHuman:".to_string()]));
    }

    #[test]
    fn test_multiple_text_parts() {
        let prompt = vec![LanguageModelMessage::User(LanguageModelUserMessage::new(
            vec![
                LanguageModelUserMessagePart::Text(LanguageModelTextPart::new("Hello ")),
                LanguageModelUserMessagePart::Text(LanguageModelTextPart::new("world!")),
            ],
        ))];

        let result = convert_to_openai_compatible_completion_prompt(prompt, None, None).unwrap();

        assert_eq!(result.prompt, "user:\nHello world!\n\nassistant:\n");
    }

    #[test]
    fn test_tool_call_errors() {
        use serde_json::json;

        let prompt = vec![LanguageModelMessage::Assistant(
            LanguageModelAssistantMessage::new(vec![LanguageModelAssistantMessagePart::ToolCall(
                LanguageModelToolCallPart::new("call_123", "get_weather", json!({})),
            )]),
        )];

        let result = convert_to_openai_compatible_completion_prompt(prompt, None, None);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("tool-call messages"));
    }

    #[test]
    fn test_tool_message_errors() {
        use ai_sdk_provider::language_model::prompt::{
            LanguageModelToolResultOutput, LanguageModelToolResultPart,
        };

        let prompt = vec![LanguageModelMessage::Tool(LanguageModelToolMessage::new(
            vec![LanguageModelToolResultPart::new(
                "call_123",
                "get_weather",
                LanguageModelToolResultOutput::Text {
                    value: "Sunny".to_string(),
                },
            )],
        ))];

        let result = convert_to_openai_compatible_completion_prompt(prompt, None, None);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("tool messages"));
    }

    #[test]
    fn test_empty_prompt() {
        let prompt = vec![];

        let result = convert_to_openai_compatible_completion_prompt(prompt, None, None).unwrap();

        assert_eq!(result.prompt, "assistant:\n");
        assert_eq!(result.stop_sequences, Some(vec!["\nuser:".to_string()]));
    }

    #[test]
    fn test_file_parts_filtered() {
        use ai_sdk_provider::language_model::prompt::LanguageModelDataContent;
        use ai_sdk_provider::language_model::prompt::message::parts::LanguageModelFilePart;

        let prompt = vec![LanguageModelMessage::User(LanguageModelUserMessage::new(
            vec![
                LanguageModelUserMessagePart::Text(LanguageModelTextPart::new("Look at this: ")),
                LanguageModelUserMessagePart::File(LanguageModelFilePart::new(
                    LanguageModelDataContent::Url("https://example.com/image.jpg".parse().unwrap()),
                    "image/jpeg",
                )),
                LanguageModelUserMessagePart::Text(LanguageModelTextPart::new("Cool!")),
            ],
        ))];

        let result = convert_to_openai_compatible_completion_prompt(prompt, None, None).unwrap();

        // File parts should be filtered out
        assert_eq!(result.prompt, "user:\nLook at this: Cool!\n\nassistant:\n");
    }

    #[test]
    fn test_assistant_with_text_and_tool_call() {
        use serde_json::json;

        let prompt = vec![LanguageModelMessage::Assistant(
            LanguageModelAssistantMessage::new(vec![
                LanguageModelAssistantMessagePart::Text(LanguageModelTextPart::new(
                    "Let me check that.",
                )),
                LanguageModelAssistantMessagePart::ToolCall(LanguageModelToolCallPart::new(
                    "call_123",
                    "search",
                    json!({}),
                )),
            ]),
        )];

        let result = convert_to_openai_compatible_completion_prompt(prompt, None, None);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("tool-call messages"));
    }
}
