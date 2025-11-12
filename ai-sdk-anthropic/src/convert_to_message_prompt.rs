use crate::get_cache_control::{CacheControlContext, CacheControlValidator};
use crate::prompt::AnthropicMessagesPrompt;
use crate::prompt::message::content::document::{AnthropicDocumentContent, DocumentCitations};
use crate::prompt::message::content::image::AnthropicImageContent;
use crate::prompt::message::content::source_type::AnthropicContentSource;
use crate::prompt::message::content::text::AnthropicTextContent;
use crate::prompt::message::content::tool_result::{
    AnthropicNestedContent, AnthropicToolResultContent, ToolResultContentType,
};
use crate::prompt::message::user::{AnthropicUserMessage, UserMessageContent};
use crate::provider_metadata_utils::{get_document_metadata, should_enable_citations};
use ai_sdk_provider::error::ProviderError;
use ai_sdk_provider::language_model::call_warning::LanguageModelCallWarning;
use ai_sdk_provider::language_model::prompt::message::{
    LanguageModelAssistantMessage, LanguageModelDataContent, LanguageModelSystemMessage,
    LanguageModelToolMessage, LanguageModelToolResultContentItem, LanguageModelToolResultOutput,
    LanguageModelUserMessage,
};
use ai_sdk_provider::language_model::prompt::{LanguageModelMessage, LanguageModelPrompt};
use std::collections::HashSet;

/// A block of system messages
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SystemBlock {
    pub messages: Vec<LanguageModelSystemMessage>,
}

/// A block of assistant messages
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AssistantBlock {
    pub messages: Vec<LanguageModelAssistantMessage>,
}

/// A block of user messages (includes tool messages)
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct UserBlock {
    pub messages: Vec<UserOrToolMessage>,
}

/// User or tool message enum for UserBlock
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum UserOrToolMessage {
    User(LanguageModelUserMessage),
    Tool(LanguageModelToolMessage),
}

/// A block of consecutive messages with the same role type
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Block {
    System(SystemBlock),
    Assistant(AssistantBlock),
    User(UserBlock),
}

impl Block {
    /// Get the type of this block as a string
    #[allow(dead_code)]
    pub fn block_type(&self) -> &str {
        match self {
            Block::System(_) => "system",
            Block::Assistant(_) => "assistant",
            Block::User(_) => "user",
        }
    }
}

/// Helper to extract provider_options from user message part
#[allow(dead_code)]
fn get_part_provider_options(
    part: &ai_sdk_provider::language_model::prompt::message::LanguageModelUserMessagePart,
) -> &Option<ai_sdk_provider::shared::provider_options::SharedProviderOptions> {
    match part {
        ai_sdk_provider::language_model::prompt::message::LanguageModelUserMessagePart::Text(
            text_part,
        ) => &text_part.provider_options,
        ai_sdk_provider::language_model::prompt::message::LanguageModelUserMessagePart::File(
            file_part,
        ) => &file_part.provider_options,
    }
}

/// Helper to extract provider_options from assistant message part
#[allow(dead_code)]
fn get_assistant_part_provider_options(
    part: &ai_sdk_provider::language_model::prompt::message::LanguageModelAssistantMessagePart,
) -> &Option<ai_sdk_provider::shared::provider_options::SharedProviderOptions> {
    match part {
        ai_sdk_provider::language_model::prompt::message::LanguageModelAssistantMessagePart::Text(text_part) => {
            &text_part.provider_options
        }
        ai_sdk_provider::language_model::prompt::message::LanguageModelAssistantMessagePart::File(file_part) => {
            &file_part.provider_options
        }
        ai_sdk_provider::language_model::prompt::message::LanguageModelAssistantMessagePart::Reasoning(reasoning_part) => {
            &reasoning_part.provider_options
        }
        ai_sdk_provider::language_model::prompt::message::LanguageModelAssistantMessagePart::ToolCall(tool_call_part) => {
            &tool_call_part.provider_options
        }
        ai_sdk_provider::language_model::prompt::message::LanguageModelAssistantMessagePart::ToolResult(tool_result_part) => {
            &tool_result_part.provider_options
        }
    }
}

/// Helper function to convert LanguageModelDataContent to base64 string
#[allow(dead_code)]
fn convert_to_base64(data: &LanguageModelDataContent) -> String {
    match data {
        LanguageModelDataContent::Base64(s) => s.clone(),
        LanguageModelDataContent::Bytes(bytes) => {
            use base64::{Engine as _, engine::general_purpose};
            general_purpose::STANDARD.encode(bytes)
        }
        LanguageModelDataContent::Url(_url) => {
            // URLs should not be converted to base64
            // The caller should check if data is URL before calling this
            String::new()
        }
    }
}

/// Helper function to convert LanguageModelDataContent to plain text string
#[allow(dead_code)]
fn convert_to_string(data: &LanguageModelDataContent) -> String {
    match data {
        LanguageModelDataContent::Base64(s) => s.clone(),
        LanguageModelDataContent::Bytes(bytes) => String::from_utf8_lossy(bytes).to_string(),
        LanguageModelDataContent::Url(url) => url.to_string(),
    }
}

/// Groups messages into blocks of consecutive messages with the same role type.
///
/// System messages are grouped into SystemBlocks.
/// Assistant messages are grouped into AssistantBlocks.
/// User and tool messages are grouped together into UserBlocks.
///
/// This follows Anthropic's API requirement where messages must alternate between
/// user and assistant roles, with tool messages treated as part of user blocks.
#[allow(dead_code)]
pub fn group_into_blocks(prompt: LanguageModelPrompt) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();
    let mut current_block: Option<Block> = None;

    for message in prompt {
        match message {
            LanguageModelMessage::System(system_msg) => {
                match &mut current_block {
                    Some(Block::System(block)) => {
                        // Continue adding to the current system block
                        block.messages.push(system_msg);
                    }
                    _ => {
                        // Start a new system block
                        if let Some(block) = current_block.take() {
                            blocks.push(block);
                        }
                        current_block = Some(Block::System(SystemBlock {
                            messages: vec![system_msg],
                        }));
                    }
                }
            }
            LanguageModelMessage::Assistant(assistant_msg) => {
                match &mut current_block {
                    Some(Block::Assistant(block)) => {
                        // Continue adding to the current assistant block
                        block.messages.push(assistant_msg);
                    }
                    _ => {
                        // Start a new assistant block
                        if let Some(block) = current_block.take() {
                            blocks.push(block);
                        }
                        current_block = Some(Block::Assistant(AssistantBlock {
                            messages: vec![assistant_msg],
                        }));
                    }
                }
            }
            LanguageModelMessage::User(user_msg) => {
                match &mut current_block {
                    Some(Block::User(block)) => {
                        // Continue adding to the current user block
                        block.messages.push(UserOrToolMessage::User(user_msg));
                    }
                    _ => {
                        // Start a new user block
                        if let Some(block) = current_block.take() {
                            blocks.push(block);
                        }
                        current_block = Some(Block::User(UserBlock {
                            messages: vec![UserOrToolMessage::User(user_msg)],
                        }));
                    }
                }
            }
            LanguageModelMessage::Tool(tool_msg) => {
                match &mut current_block {
                    Some(Block::User(block)) => {
                        // Continue adding to the current user block (tool messages are treated as user)
                        block.messages.push(UserOrToolMessage::Tool(tool_msg));
                    }
                    _ => {
                        // Start a new user block
                        if let Some(block) = current_block.take() {
                            blocks.push(block);
                        }
                        current_block = Some(Block::User(UserBlock {
                            messages: vec![UserOrToolMessage::Tool(tool_msg)],
                        }));
                    }
                }
            }
        }
    }

    // Push the last block if it exists
    if let Some(block) = current_block {
        blocks.push(block);
    }

    blocks
}

/// Result of converting a prompt to Anthropic's Messages API format
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ConvertToMessagePromptResult {
    /// The converted Anthropic messages prompt
    pub prompt: AnthropicMessagesPrompt,
    /// Beta features that need to be enabled for this request
    pub betas: HashSet<String>,
}

/// Converts a language model prompt to Anthropic's Messages API format.
///
/// This function takes a generic language model prompt and transforms it into the
/// specific format required by Anthropic's Messages API, handling:
/// - Message grouping and role alternation
/// - System message extraction
/// - Tool message merging with user messages
/// - Cache control validation
/// - Beta feature detection
///
/// # Arguments
///
/// * `prompt` - The language model prompt to convert
/// * `send_reasoning` - Whether to include reasoning/thinking content in the output
/// * `warnings` - Mutable vector to collect any warnings during conversion
/// * `cache_control_validator` - Optional validator for cache control settings
///
/// # Returns
///
/// A `ConvertToMessagePromptResult` containing:
/// - `prompt`: The converted Anthropic messages prompt
/// - `betas`: Set of beta feature names that need to be enabled
///
/// # Example
///
/// ```ignore
/// use ai_sdk_anthropic::convert_to_message_prompt::{convert_to_message_prompt, ConvertToMessagePromptResult};
/// use ai_sdk_anthropic::get_cache_control::CacheControlValidator;
/// use ai_sdk_provider::language_model::prompt::LanguageModelMessage;
///
/// let prompt = vec![
///     LanguageModelMessage::user_text("Hello!"),
/// ];
/// let mut warnings = Vec::new();
/// let validator = CacheControlValidator::new();
///
/// let result = convert_to_message_prompt(
///     prompt,
///     false,
///     &mut warnings,
///     Some(validator),
/// );
///
/// println!("Betas needed: {:?}", result.betas);
/// ```
#[allow(dead_code)]
#[allow(clippy::result_large_err)]
pub fn convert_to_message_prompt(
    prompt: LanguageModelPrompt,
    _send_reasoning: bool,
    _warnings: &mut Vec<LanguageModelCallWarning>,
    cache_control_validator: Option<CacheControlValidator>,
) -> Result<ConvertToMessagePromptResult, ProviderError> {
    let mut betas: HashSet<String> = HashSet::new();
    let blocks = group_into_blocks(prompt);
    let mut validator = cache_control_validator.unwrap_or_default();

    let mut system: Option<Vec<AnthropicTextContent>> = None;
    let mut messages: Vec<crate::prompt::message::AnthropicMessage> = vec![];

    for (_i, block) in blocks.iter().enumerate() {
        let _is_last_block = _i == blocks.len() - 1;

        match block {
            Block::System(system_block) => {
                // Check if we already have system messages
                if system.is_some() {
                    return Err(ProviderError::unsupported_functionality_with_message(
                        "multiple_system_blocks",
                        "Multiple system messages that are separated by user/assistant messages",
                    ));
                }

                // Convert system messages to Anthropic text content
                system = Some(
                    system_block
                        .messages
                        .iter()
                        .map(|msg| {
                            let cache_control = validator.get_cache_control(
                                msg.provider_options.as_ref(),
                                &CacheControlContext::new("system message", true),
                            );

                            let mut text_content = AnthropicTextContent::new(&msg.content);
                            if let Some(cc) = cache_control {
                                text_content = text_content.with_cache_control(cc);
                            }
                            text_content
                        })
                        .collect(),
                );
            }
            Block::Assistant(assistant_block) => {
                // Combines multiple assistant messages in this block into a single message
                let mut anthropic_content: Vec<
                    crate::prompt::message::assistant::AnthropicAssistantMessageContent,
                > = Vec::new();

                // Track MCP tool use IDs
                let mut mcp_tool_use_ids: std::collections::HashSet<String> =
                    std::collections::HashSet::new();

                for (j, message) in assistant_block.messages.iter().enumerate() {
                    let is_last_message = j == assistant_block.messages.len() - 1;

                    for (k, part) in message.content.iter().enumerate() {
                        let is_last_content_part = k == message.content.len() - 1;

                        // Get cache control from part or message
                        let cache_control = validator
                            .get_cache_control(
                                get_assistant_part_provider_options(part).as_ref(),
                                &CacheControlContext::new("assistant message part", true),
                            )
                            .or_else(|| {
                                if is_last_content_part {
                                    validator.get_cache_control(
                                        message.provider_options.as_ref(),
                                        &CacheControlContext::new("assistant message", true),
                                    )
                                } else {
                                    None
                                }
                            });

                        match part {
                            ai_sdk_provider::language_model::prompt::message::LanguageModelAssistantMessagePart::Text(text_part) => {
                                let text = if _is_last_block && is_last_message && is_last_content_part {
                                    // Trim the last text part if it's the last message in the block
                                    // because Anthropic does not allow trailing whitespace
                                    // in pre-filled assistant responses
                                    text_part.text.trim()
                                } else {
                                    &text_part.text
                                };

                                let mut text_content = crate::prompt::message::content::text::AnthropicTextContent::new(text);
                                if let Some(cc) = cache_control {
                                    text_content = text_content.with_cache_control(cc);
                                }
                                anthropic_content.push(text_content.into());
                            }
                            ai_sdk_provider::language_model::prompt::message::LanguageModelAssistantMessagePart::Reasoning(reasoning_part) => {
                                if _send_reasoning {
                                    if let Some(reasoning_metadata) = crate::provider_metadata_utils::get_reasoning_metadata(
                                        reasoning_part.provider_options.as_ref()
                                    ) {
                                        if let Some(signature) = reasoning_metadata.signature {
                                            // Note: thinking blocks cannot have cache_control directly
                                            // They are cached implicitly when in previous assistant turns
                                            // Validate to provide helpful error message
                                            validator.get_cache_control(
                                                reasoning_part.provider_options.as_ref(),
                                                &CacheControlContext::new("thinking block", false),
                                            );

                                            anthropic_content.push(
                                                crate::prompt::message::content::thinking::AnthropicThinkingContent::new(
                                                    &reasoning_part.text,
                                                    signature,
                                                ).into()
                                            );
                                        } else if let Some(redacted_data) = reasoning_metadata.redacted_data {
                                            // Note: redacted thinking blocks cannot have cache_control directly
                                            // They are cached implicitly when in previous assistant turns
                                            // Validate to provide helpful error message
                                            validator.get_cache_control(
                                                reasoning_part.provider_options.as_ref(),
                                                &CacheControlContext::new("redacted thinking block", false),
                                            );

                                            anthropic_content.push(
                                                crate::prompt::message::content::redacted_thinking::AnthropicRedactedThinkingContent::new(
                                                    redacted_data
                                                ).into()
                                            );
                                        } else {
                                            _warnings.push(LanguageModelCallWarning::other(
                                                "unsupported reasoning metadata".to_string()
                                            ));
                                        }
                                    } else {
                                        _warnings.push(LanguageModelCallWarning::other(
                                            "unsupported reasoning metadata".to_string()
                                        ));
                                    }
                                } else {
                                    _warnings.push(LanguageModelCallWarning::other(
                                        "sending reasoning content is disabled for this model".to_string()
                                    ));
                                }
                            }
                            ai_sdk_provider::language_model::prompt::message::LanguageModelAssistantMessagePart::ToolCall(tool_call_part) => {
                                // Handle provider-executed tool calls
                                if tool_call_part.provider_executed.unwrap_or(false) {
                                    // Check if this is an MCP tool use
                                    if let Some(tool_options) = crate::provider_metadata_utils::get_tool_call_options(
                                        tool_call_part.provider_options.as_ref()
                                    )
                                        && tool_options.tool_type.as_deref() == Some("mcp-tool-use") {
                                            mcp_tool_use_ids.insert(tool_call_part.tool_call_id.clone());

                                            if let Some(server_name) = tool_options.server_name {
                                                let mut mcp_tool_use = crate::prompt::message::content::mcp_tool_use::AnthropicMcpToolUseContent::new(
                                                    &tool_call_part.tool_call_id,
                                                    &tool_call_part.tool_name,
                                                    server_name,
                                                    tool_call_part.input.clone(),
                                                );
                                                if let Some(cc) = cache_control {
                                                    mcp_tool_use = mcp_tool_use.with_cache_control(cc);
                                                }
                                                anthropic_content.push(mcp_tool_use.into());
                                            } else {
                                                _warnings.push(LanguageModelCallWarning::other(
                                                    "mcp tool use server name is required and must be a string".to_string()
                                                ));
                                            }
                                            continue;
                                        }

                                    // Handle code_execution tool (20250825 version with subtypes)
                                    if tool_call_part.tool_name == "code_execution" {
                                        if let Some(input_obj) = tool_call_part.input.as_object()
                                            && let Some(input_type) = input_obj.get("type").and_then(|v| v.as_str())
                                                && (input_type == "bash_code_execution" || input_type == "text_editor_code_execution") {
                                                    let server_tool_type = if input_type == "bash_code_execution" {
                                                        crate::prompt::message::content::server_tool_use::ServerToolType::BashCodeExecution
                                                    } else {
                                                        crate::prompt::message::content::server_tool_use::ServerToolType::TextEditorCodeExecution
                                                    };

                                                    let mut server_tool_use = crate::prompt::message::content::server_tool_use::AnthropicServerToolUseContent::new(
                                                        &tool_call_part.tool_call_id,
                                                        server_tool_type,
                                                        tool_call_part.input.clone(),
                                                    );
                                                    if let Some(cc) = cache_control {
                                                        server_tool_use = server_tool_use.with_cache_control(cc);
                                                    }
                                                    anthropic_content.push(server_tool_use.into());
                                                    continue;
                                                }
                                        // code_execution 20250522 version (no subtype)
                                        let mut server_tool_use = crate::prompt::message::content::server_tool_use::AnthropicServerToolUseContent::code_execution(
                                            &tool_call_part.tool_call_id,
                                            tool_call_part.input.clone(),
                                        );
                                        if let Some(cc) = cache_control {
                                            server_tool_use = server_tool_use.with_cache_control(cc);
                                        }
                                        anthropic_content.push(server_tool_use.into());
                                        continue;
                                    }

                                    // Handle web_fetch and web_search
                                    if tool_call_part.tool_name == "web_fetch" {
                                        let mut server_tool_use = crate::prompt::message::content::server_tool_use::AnthropicServerToolUseContent::web_fetch(
                                            &tool_call_part.tool_call_id,
                                            tool_call_part.input.clone(),
                                        );
                                        if let Some(cc) = cache_control {
                                            server_tool_use = server_tool_use.with_cache_control(cc);
                                        }
                                        anthropic_content.push(server_tool_use.into());
                                        continue;
                                    }

                                    if tool_call_part.tool_name == "web_search" {
                                        let mut server_tool_use = crate::prompt::message::content::server_tool_use::AnthropicServerToolUseContent::web_search(
                                            &tool_call_part.tool_call_id,
                                            tool_call_part.input.clone(),
                                        );
                                        if let Some(cc) = cache_control {
                                            server_tool_use = server_tool_use.with_cache_control(cc);
                                        }
                                        anthropic_content.push(server_tool_use.into());
                                        continue;
                                    }

                                    // Unsupported provider-executed tool
                                    _warnings.push(LanguageModelCallWarning::other(
                                        format!("provider executed tool call for tool {} is not supported", tool_call_part.tool_name)
                                    ));
                                    continue;
                                }

                                // Regular tool call (non-provider-executed)
                                let mut tool_call = crate::prompt::message::content::tool_call::AnthropicToolCallContent::new(
                                    &tool_call_part.tool_call_id,
                                    &tool_call_part.tool_name,
                                    tool_call_part.input.clone(),
                                );
                                if let Some(cc) = cache_control {
                                    tool_call = tool_call.with_cache_control(cc);
                                }
                                anthropic_content.push(tool_call.into());
                            }
                            ai_sdk_provider::language_model::prompt::message::LanguageModelAssistantMessagePart::ToolResult(_tool_result_part) => {
                                // Tool results in assistant messages will be handled in the next part
                                // For now, we'll add a placeholder warning
                                _warnings.push(LanguageModelCallWarning::other(
                                    "tool result conversion in assistant messages not yet implemented".to_string()
                                ));
                            }
                            ai_sdk_provider::language_model::prompt::message::LanguageModelAssistantMessagePart::File(_file_part) => {
                                // Files in assistant messages are not directly supported
                                _warnings.push(LanguageModelCallWarning::other(
                                    "file content in assistant messages is not supported by Anthropic".to_string()
                                ));
                            }
                        }
                    }
                }

                // Push the combined assistant message
                messages.push(crate::prompt::message::AnthropicMessage::Assistant(
                    crate::prompt::message::assistant::AnthropicAssistantMessage::new(
                        anthropic_content,
                    ),
                ));
            }
            Block::User(user_block) => {
                // Combines all user and tool messages in this block into a single message
                let mut anthropic_content: Vec<UserMessageContent> = Vec::new();

                for message in &user_block.messages {
                    match message {
                        UserOrToolMessage::User(user_msg) => {
                            // Iterate through all content parts in the user message
                            for (j, part) in user_msg.content.iter().enumerate() {
                                let is_last_part = j == user_msg.content.len() - 1;

                                // Get cache control from part or message
                                let cache_control = validator
                                    .get_cache_control(
                                        get_part_provider_options(part).as_ref(),
                                        &CacheControlContext::new("user message part", true),
                                    )
                                    .or_else(|| {
                                        if is_last_part {
                                            validator.get_cache_control(
                                                user_msg.provider_options.as_ref(),
                                                &CacheControlContext::new("user message", true),
                                            )
                                        } else {
                                            None
                                        }
                                    });

                                match part {
                                    ai_sdk_provider::language_model::prompt::message::LanguageModelUserMessagePart::Text(text_part) => {
                                        let mut text_content = AnthropicTextContent::new(&text_part.text);
                                        if let Some(cc) = cache_control {
                                            text_content = text_content.with_cache_control(cc);
                                        }
                                        anthropic_content.push(text_content.into());
                                    }
                                    ai_sdk_provider::language_model::prompt::message::LanguageModelUserMessagePart::File(file_part) => {
                                        // Handle different file types based on media type
                                        if file_part.media_type.starts_with("image/") {
                                            // Image file
                                            let source = match &file_part.data {
                                                LanguageModelDataContent::Url(url) => {
                                                    AnthropicContentSource::url(url.to_string())
                                                }
                                                _ => {
                                                    let media_type = if file_part.media_type == "image/*" {
                                                        "image/jpeg"
                                                    } else {
                                                        &file_part.media_type
                                                    };
                                                    AnthropicContentSource::base64(
                                                        media_type,
                                                        convert_to_base64(&file_part.data),
                                                    )
                                                }
                                            };

                                            let mut image_content = AnthropicImageContent::new(source);
                                            if let Some(cc) = cache_control {
                                                image_content = image_content.with_cache_control(cc);
                                            }
                                            anthropic_content.push(image_content.into());
                                        } else if file_part.media_type == "application/pdf" {
                                            // PDF document
                                            betas.insert("pdfs-2024-09-25".to_string());

                                            let enable_citations = should_enable_citations(
                                                file_part.provider_options.as_ref()
                                            );

                                            let metadata = get_document_metadata(
                                                file_part.provider_options.as_ref()
                                            );

                                            let source = match &file_part.data {
                                                LanguageModelDataContent::Url(url) => {
                                                    AnthropicContentSource::url(url.to_string())
                                                }
                                                _ => {
                                                    AnthropicContentSource::base64(
                                                        "application/pdf",
                                                        convert_to_base64(&file_part.data),
                                                    )
                                                }
                                            };

                                            let mut document_content = AnthropicDocumentContent::new(source);

                                            // Set title (prefer metadata.title, fallback to filename)
                                            if let Some(title) = metadata.title.or_else(|| file_part.filename.clone()) {
                                                document_content = document_content.with_title(title);
                                            }

                                            // Set context if provided
                                            if let Some(context) = metadata.context {
                                                document_content = document_content.with_context(context);
                                            }

                                            // Set citations if enabled
                                            if enable_citations {
                                                document_content = document_content.with_citations(
                                                    DocumentCitations::enabled()
                                                );
                                            }

                                            // Set cache control
                                            if let Some(cc) = cache_control {
                                                document_content = document_content.with_cache_control(cc);
                                            }

                                            anthropic_content.push(document_content.into());
                                        } else if file_part.media_type == "text/plain" {
                                            // Text document
                                            let enable_citations = should_enable_citations(
                                                file_part.provider_options.as_ref()
                                            );

                                            let metadata = get_document_metadata(
                                                file_part.provider_options.as_ref()
                                            );

                                            let source = match &file_part.data {
                                                LanguageModelDataContent::Url(url) => {
                                                    AnthropicContentSource::url(url.to_string())
                                                }
                                                _ => {
                                                    AnthropicContentSource::text(
                                                        convert_to_string(&file_part.data)
                                                    )
                                                }
                                            };

                                            let mut document_content = AnthropicDocumentContent::new(source);

                                            // Set title (prefer metadata.title, fallback to filename)
                                            if let Some(title) = metadata.title.or_else(|| file_part.filename.clone()) {
                                                document_content = document_content.with_title(title);
                                            }

                                            // Set context if provided
                                            if let Some(context) = metadata.context {
                                                document_content = document_content.with_context(context);
                                            }

                                            // Set citations if enabled
                                            if enable_citations {
                                                document_content = document_content.with_citations(
                                                    DocumentCitations::enabled()
                                                );
                                            }

                                            // Set cache control
                                            if let Some(cc) = cache_control {
                                                document_content = document_content.with_cache_control(cc);
                                            }

                                            anthropic_content.push(document_content.into());
                                        } else {
                                            // Unsupported media type
                                            return Err(ProviderError::unsupported_functionality_with_message(
                                                format!("media_type_{}", file_part.media_type),
                                                format!("Unsupported media type: {}", file_part.media_type),
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                        UserOrToolMessage::Tool(tool_msg) => {
                            // Handle tool result messages
                            for (i, part) in tool_msg.content.iter().enumerate() {
                                let is_last_part = i == tool_msg.content.len() - 1;

                                // Get cache control from part or message
                                let cache_control = validator
                                    .get_cache_control(
                                        part.provider_options.as_ref(),
                                        &CacheControlContext::new("tool result part", true),
                                    )
                                    .or_else(|| {
                                        if is_last_part {
                                            validator.get_cache_control(
                                                tool_msg.provider_options.as_ref(),
                                                &CacheControlContext::new(
                                                    "tool result message",
                                                    true,
                                                ),
                                            )
                                        } else {
                                            None
                                        }
                                    });

                                // Convert tool result output to content
                                let content_value: ToolResultContentType = match &part.output {
                                    LanguageModelToolResultOutput::Content { value } => {
                                        // Map content items to Anthropic nested content
                                        let nested_content: Vec<AnthropicNestedContent> = value
                                            .iter()
                                            .filter_map(|content_part| {
                                                match content_part {
                                                    LanguageModelToolResultContentItem::Text { text } => {
                                                        Some(AnthropicNestedContent::Text {
                                                            text: text.clone(),
                                                        })
                                                    }
                                                    LanguageModelToolResultContentItem::Media { data, media_type } => {
                                                        // Check if it's a supported media type
                                                        if media_type.starts_with("image/") {
                                                            Some(AnthropicNestedContent::Image {
                                                                source: AnthropicContentSource::base64(
                                                                    media_type.clone(),
                                                                    data.clone(),
                                                                ),
                                                            })
                                                        } else if media_type == "application/pdf" {
                                                            betas.insert("pdfs-2024-09-25".to_string());
                                                            Some(AnthropicNestedContent::Document {
                                                                source: AnthropicContentSource::base64(
                                                                    media_type.clone(),
                                                                    data.clone(),
                                                                ),
                                                                title: None,
                                                                context: None,
                                                                citations: None,
                                                            })
                                                        } else {
                                                            _warnings.push(LanguageModelCallWarning::other(
                                                                format!("unsupported tool content part media type: {}", media_type)
                                                            ));
                                                            None
                                                        }
                                                    }
                                                }
                                            })
                                            .collect();

                                        ToolResultContentType::Array(nested_content)
                                    }
                                    LanguageModelToolResultOutput::Text { value } => {
                                        ToolResultContentType::String(value.clone())
                                    }
                                    LanguageModelToolResultOutput::ErrorText { value } => {
                                        ToolResultContentType::String(value.clone())
                                    }
                                    LanguageModelToolResultOutput::Json { value } => {
                                        ToolResultContentType::String(
                                            serde_json::to_string(value).unwrap_or_default(),
                                        )
                                    }
                                    LanguageModelToolResultOutput::ErrorJson { value } => {
                                        ToolResultContentType::String(
                                            serde_json::to_string(value).unwrap_or_default(),
                                        )
                                    }
                                };

                                // Determine if this is an error
                                let is_error = matches!(
                                    part.output,
                                    LanguageModelToolResultOutput::ErrorText { .. }
                                        | LanguageModelToolResultOutput::ErrorJson { .. }
                                );

                                let mut tool_result = AnthropicToolResultContent::new(
                                    &part.tool_call_id,
                                    content_value,
                                );

                                if is_error {
                                    tool_result = tool_result.with_error(true);
                                }

                                if let Some(cc) = cache_control {
                                    tool_result = tool_result.with_cache_control(cc);
                                }

                                anthropic_content.push(tool_result.into());
                            }
                        }
                    }
                }

                // Push the combined user message
                messages.push(crate::prompt::message::AnthropicMessage::User(
                    AnthropicUserMessage::new(anthropic_content),
                ));
            }
        }
    }

    // TODO: Implement the rest of the conversion logic
    // - Convert user/assistant blocks to Anthropic messages
    // - Handle tool messages
    // - Detect beta features (e.g., extended thinking, prompt caching)

    // Build the final prompt
    let mut anthropic_prompt = AnthropicMessagesPrompt::new(messages);
    if let Some(sys) = system
        && !sys.is_empty()
    {
        anthropic_prompt = anthropic_prompt.with_system(sys);
    }

    Ok(ConvertToMessagePromptResult {
        prompt: anthropic_prompt,
        betas,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ai_sdk_provider::language_model::prompt::message::LanguageModelSystemMessage;

    #[test]
    fn test_convert_single_system_message() {
        let prompt = vec![LanguageModelMessage::System(
            LanguageModelSystemMessage::new("You are a helpful assistant."),
        )];

        let result = convert_to_message_prompt(prompt, false, &mut vec![], None);

        assert!(result.is_ok());
        let result = result.unwrap();

        // Check that system messages were converted
        assert!(result.prompt.system.is_some());
        let system = result.prompt.system.unwrap();
        assert_eq!(system.len(), 1);
        assert_eq!(system[0].text, "You are a helpful assistant.");
        assert_eq!(system[0].content_type, "text");
    }

    #[test]
    fn test_convert_multiple_system_messages_in_one_block() {
        let prompt = vec![
            LanguageModelMessage::System(LanguageModelSystemMessage::new("System 1")),
            LanguageModelMessage::System(LanguageModelSystemMessage::new("System 2")),
        ];

        let result = convert_to_message_prompt(prompt, false, &mut vec![], None);

        assert!(result.is_ok());
        let result = result.unwrap();

        // Check that both system messages were converted
        assert!(result.prompt.system.is_some());
        let system = result.prompt.system.unwrap();
        assert_eq!(system.len(), 2);
        assert_eq!(system[0].text, "System 1");
        assert_eq!(system[1].text, "System 2");
    }

    #[test]
    fn test_convert_separated_system_messages_error() {
        let prompt = vec![
            LanguageModelMessage::System(LanguageModelSystemMessage::new("System 1")),
            LanguageModelMessage::user_text("User message"),
            LanguageModelMessage::System(LanguageModelSystemMessage::new("System 2")),
        ];

        let result = convert_to_message_prompt(prompt, false, &mut vec![], None);

        assert!(result.is_err());
        match result.unwrap_err() {
            ProviderError::UnsupportedFunctionality {
                functionality,
                message,
            } => {
                assert_eq!(functionality, "multiple_system_blocks");
                assert!(message.contains("separated by user/assistant messages"));
            }
            _ => panic!("Expected UnsupportedFunctionality error"),
        }
    }

    #[test]
    fn test_group_into_blocks_basic() {
        let prompt = vec![
            LanguageModelMessage::System(LanguageModelSystemMessage::new("System")),
            LanguageModelMessage::user_text("User"),
            LanguageModelMessage::assistant_text("Assistant"),
        ];

        let blocks = group_into_blocks(prompt);

        assert_eq!(blocks.len(), 3);
        assert!(matches!(blocks[0], Block::System(_)));
        assert!(matches!(blocks[1], Block::User(_)));
        assert!(matches!(blocks[2], Block::Assistant(_)));
    }

    #[test]
    fn test_group_into_blocks_consecutive_same_role() {
        let prompt = vec![
            LanguageModelMessage::System(LanguageModelSystemMessage::new("System 1")),
            LanguageModelMessage::System(LanguageModelSystemMessage::new("System 2")),
            LanguageModelMessage::user_text("User 1"),
            LanguageModelMessage::user_text("User 2"),
        ];

        let blocks = group_into_blocks(prompt);

        assert_eq!(blocks.len(), 2);

        // First block should be System with 2 messages
        match &blocks[0] {
            Block::System(block) => {
                assert_eq!(block.messages.len(), 2);
            }
            _ => panic!("Expected SystemBlock"),
        }

        // Second block should be User with 2 messages
        match &blocks[1] {
            Block::User(block) => {
                assert_eq!(block.messages.len(), 2);
            }
            _ => panic!("Expected UserBlock"),
        }
    }

    #[test]
    fn test_empty_prompt() {
        let prompt = vec![];
        let result = convert_to_message_prompt(prompt, false, &mut vec![], None);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.prompt.system.is_none());
        assert_eq!(result.prompt.messages.len(), 0);
    }

    #[test]
    fn test_betas_initialized() {
        let prompt = vec![];
        let result = convert_to_message_prompt(prompt, false, &mut vec![], None);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.betas.len(), 0);
    }
}
