use llm_kit_openai_compatible::OpenAICompatibleChatLanguageModel;

/// Groq chat language model implementation.
///
/// This wraps the OpenAI-compatible chat model with Groq-specific configuration.
pub type GroqChatLanguageModel = OpenAICompatibleChatLanguageModel;

// Re-export config type for consistency
pub use llm_kit_openai_compatible::OpenAICompatibleChatConfig as GroqChatConfig;
