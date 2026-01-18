/// Result of converting a prompt to OpenAI-compatible completion format
#[derive(Debug, Clone, PartialEq)]
pub struct OpenAICompatibleCompletionPrompt {
    /// The formatted prompt text
    pub prompt: String,
    /// Optional stop sequences to use with the completion
    pub stop_sequences: Option<Vec<String>>,
}
