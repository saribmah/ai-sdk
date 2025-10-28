use ai_sdk_provider::language_model::prompt::{
    AssistantMessagePart, Message, Prompt, UserMessagePart,
};

/// Result of converting a prompt to OpenAI-compatible completion format
#[derive(Debug, Clone, PartialEq)]
pub struct CompletionPrompt {
    /// The formatted prompt text
    pub prompt: String,
    /// Optional stop sequences to use with the completion
    pub stop_sequences: Option<Vec<String>>,
}

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
    mut prompt: Prompt,
    user: Option<&str>,
    assistant: Option<&str>,
) -> Result<CompletionPrompt, String> {
    let user_prefix = user.unwrap_or("user");
    let assistant_prefix = assistant.unwrap_or("assistant");

    let mut text = String::new();

    // If first message is a system message, add it to the text
    if let Some(Message::System { content, .. }) = prompt.first() {
        text.push_str(content);
        text.push_str("\n\n");
        prompt = prompt.into_iter().skip(1).collect();
    }

    // Process remaining messages
    for message in prompt {
        match message {
            Message::System { content, .. } => {
                return Err(format!(
                    "Unexpected system message in prompt: {}",
                    content
                ));
            }

            Message::User { content, .. } => {
                let user_message: String = content
                    .into_iter()
                    .filter_map(|part| match part {
                        UserMessagePart::Text { text, .. } => Some(text),
                        UserMessagePart::File { .. } => None,
                    })
                    .collect::<Vec<_>>()
                    .join("");

                text.push_str(&format!("{}:\n{}\n\n", user_prefix, user_message));
            }

            Message::Assistant { content, .. } => {
                let mut assistant_message = String::new();

                for part in content {
                    match part {
                        AssistantMessagePart::Text { text, .. } => {
                            assistant_message.push_str(&text);
                        }
                        AssistantMessagePart::ToolCall { .. } => {
                            return Err("Unsupported functionality: tool-call messages".to_string());
                        }
                        // Ignore other assistant message types
                        _ => {}
                    }
                }

                text.push_str(&format!(
                    "{}:\n{}\n\n",
                    assistant_prefix, assistant_message
                ));
            }

            Message::Tool { .. } => {
                return Err("Unsupported functionality: tool messages".to_string());
            }
        }
    }

    // Add assistant message prefix
    text.push_str(&format!("{}:\n", assistant_prefix));

    Ok(CompletionPrompt {
        prompt: text,
        stop_sequences: Some(vec![format!("\n{}:", user_prefix)]),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_user_message() {
        let prompt = vec![Message::User {
            content: vec![UserMessagePart::Text {
                text: "Hello!".to_string(),
                provider_options: None,
            }],
            provider_options: None,
        }];

        let result =
            convert_to_openai_compatible_completion_prompt(prompt, None, None).unwrap();

        assert_eq!(result.prompt, "user:\nHello!\n\nassistant:\n");
        assert_eq!(result.stop_sequences, Some(vec!["\nuser:".to_string()]));
    }

    #[test]
    fn test_system_message_first() {
        let prompt = vec![
            Message::System {
                content: "You are a helpful assistant.".to_string(),
                provider_options: None,
            },
            Message::User {
                content: vec![UserMessagePart::Text {
                    text: "Hello!".to_string(),
                    provider_options: None,
                }],
                provider_options: None,
            },
        ];

        let result =
            convert_to_openai_compatible_completion_prompt(prompt, None, None).unwrap();

        assert_eq!(
            result.prompt,
            "You are a helpful assistant.\n\nuser:\nHello!\n\nassistant:\n"
        );
        assert_eq!(result.stop_sequences, Some(vec!["\nuser:".to_string()]));
    }

    #[test]
    fn test_system_message_not_first_errors() {
        let prompt = vec![
            Message::User {
                content: vec![UserMessagePart::Text {
                    text: "Hello!".to_string(),
                    provider_options: None,
                }],
                provider_options: None,
            },
            Message::System {
                content: "You are a helpful assistant.".to_string(),
                provider_options: None,
            },
        ];

        let result = convert_to_openai_compatible_completion_prompt(prompt, None, None);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unexpected system message"));
    }

    #[test]
    fn test_conversation() {
        let prompt = vec![
            Message::User {
                content: vec![UserMessagePart::Text {
                    text: "What is 2+2?".to_string(),
                    provider_options: None,
                }],
                provider_options: None,
            },
            Message::Assistant {
                content: vec![AssistantMessagePart::Text {
                    text: "The answer is 4.".to_string(),
                    provider_options: None,
                }],
                provider_options: None,
            },
            Message::User {
                content: vec![UserMessagePart::Text {
                    text: "What about 3+3?".to_string(),
                    provider_options: None,
                }],
                provider_options: None,
            },
        ];

        let result =
            convert_to_openai_compatible_completion_prompt(prompt, None, None).unwrap();

        assert_eq!(
            result.prompt,
            "user:\nWhat is 2+2?\n\nassistant:\nThe answer is 4.\n\nuser:\nWhat about 3+3?\n\nassistant:\n"
        );
    }

    #[test]
    fn test_custom_prefixes() {
        let prompt = vec![Message::User {
            content: vec![UserMessagePart::Text {
                text: "Hello!".to_string(),
                provider_options: None,
            }],
            provider_options: None,
        }];

        let result =
            convert_to_openai_compatible_completion_prompt(prompt, Some("Human"), Some("AI"))
                .unwrap();

        assert_eq!(result.prompt, "Human:\nHello!\n\nAI:\n");
        assert_eq!(result.stop_sequences, Some(vec!["\nHuman:".to_string()]));
    }

    #[test]
    fn test_multiple_text_parts() {
        let prompt = vec![Message::User {
            content: vec![
                UserMessagePart::Text {
                    text: "Hello ".to_string(),
                    provider_options: None,
                },
                UserMessagePart::Text {
                    text: "world!".to_string(),
                    provider_options: None,
                },
            ],
            provider_options: None,
        }];

        let result =
            convert_to_openai_compatible_completion_prompt(prompt, None, None).unwrap();

        assert_eq!(result.prompt, "user:\nHello world!\n\nassistant:\n");
    }

    #[test]
    fn test_tool_call_errors() {
        use serde_json::json;

        let prompt = vec![Message::Assistant {
            content: vec![AssistantMessagePart::ToolCall {
                tool_call_id: "call_123".to_string(),
                tool_name: "get_weather".to_string(),
                input: json!({}),
                provider_executed: None,
                provider_options: None,
            }],
            provider_options: None,
        }];

        let result = convert_to_openai_compatible_completion_prompt(prompt, None, None);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("tool-call messages"));
    }

    #[test]
    fn test_tool_message_errors() {
        use ai_sdk_provider::language_model::prompt::{ToolResultOutput, ToolResultPart};

        let prompt = vec![Message::Tool {
            content: vec![ToolResultPart::new(
                "call_123",
                "get_weather",
                ToolResultOutput::Text {
                    value: "Sunny".to_string(),
                },
            )],
            provider_options: None,
        }];

        let result = convert_to_openai_compatible_completion_prompt(prompt, None, None);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("tool messages"));
    }

    #[test]
    fn test_empty_prompt() {
        let prompt = vec![];

        let result =
            convert_to_openai_compatible_completion_prompt(prompt, None, None).unwrap();

        assert_eq!(result.prompt, "assistant:\n");
        assert_eq!(result.stop_sequences, Some(vec!["\nuser:".to_string()]));
    }

    #[test]
    fn test_file_parts_filtered() {
        use ai_sdk_provider::language_model::prompt::DataContent;

        let prompt = vec![Message::User {
            content: vec![
                UserMessagePart::Text {
                    text: "Look at this: ".to_string(),
                    provider_options: None,
                },
                UserMessagePart::File {
                    filename: None,
                    data: DataContent::Url("https://example.com/image.jpg".parse().unwrap()),
                    media_type: "image/jpeg".to_string(),
                    provider_options: None,
                },
                UserMessagePart::Text {
                    text: "Cool!".to_string(),
                    provider_options: None,
                },
            ],
            provider_options: None,
        }];

        let result =
            convert_to_openai_compatible_completion_prompt(prompt, None, None).unwrap();

        // File parts should be filtered out
        assert_eq!(result.prompt, "user:\nLook at this: Cool!\n\nassistant:\n");
    }

    #[test]
    fn test_assistant_with_text_and_tool_call() {
        use serde_json::json;

        let prompt = vec![Message::Assistant {
            content: vec![
                AssistantMessagePart::Text {
                    text: "Let me check that.".to_string(),
                    provider_options: None,
                },
                AssistantMessagePart::ToolCall {
                    tool_call_id: "call_123".to_string(),
                    tool_name: "search".to_string(),
                    input: json!({}),
                    provider_executed: None,
                    provider_options: None,
                },
            ],
            provider_options: None,
        }];

        let result = convert_to_openai_compatible_completion_prompt(prompt, None, None);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("tool-call messages"));
    }
}
