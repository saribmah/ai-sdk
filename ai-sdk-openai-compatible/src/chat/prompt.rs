/// Message types for OpenAI-compatible chat prompts.
pub mod message;

pub use message::OpenAICompatibleMessage;

/// OpenAI-compatible chat prompt (array of messages)
pub type OpenAICompatibleChatPrompt = Vec<OpenAICompatibleMessage>;
