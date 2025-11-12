//! OpenAI Chat Model Configuration and Options
//!
//! This module defines the model IDs and provider-specific options for OpenAI chat models.

use serde::{Deserialize, Serialize};

/// OpenAI chat model identifier.
///
/// Supports both specific model versions and the "string & {}" pattern for custom models.
/// Based on <https://platform.openai.com/docs/models>
pub type OpenAIChatModelId = String;

/// Provider-specific options for OpenAI chat models.
///
/// These options correspond to OpenAI-specific settings that can be passed via `provider_options`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenAIChatLanguageModelOptions {
    /// Modify the likelihood of specified tokens appearing in the completion.
    ///
    /// Accepts a map of tokens (specified by their token ID in the GPT tokenizer)
    /// to an associated bias value from -100 to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<std::collections::HashMap<String, f64>>,

    /// Return the log probabilities of the tokens.
    ///
    /// Setting to true will return the log probabilities of the tokens that were generated.
    /// Setting to a number will return the log probabilities of the top n tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<LogprobsOption>,

    /// Whether to enable parallel function calling during tool use.
    /// Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    /// A unique identifier representing your end-user.
    /// Can help OpenAI to monitor and detect abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    /// Reasoning effort for reasoning models.
    /// Defaults to `medium`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffort>,

    /// Maximum number of completion tokens to generate.
    /// Useful for reasoning models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u32>,

    /// Whether to enable persistence in responses API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,

    /// Metadata to associate with the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,

    /// Parameters for prediction mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction: Option<serde_json::Value>,

    /// Whether to use structured outputs.
    /// Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_outputs: Option<bool>,

    /// Service tier for the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,

    /// Whether to use strict JSON schema validation.
    /// Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict_json_schema: Option<bool>,

    /// Controls the verbosity of the model's responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_verbosity: Option<TextVerbosity>,

    /// A cache key for prompt caching.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,

    /// A stable identifier used to help detect users violating OpenAI's usage policies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_identifier: Option<String>,
}

/// Log probabilities option - can be boolean or a number.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LogprobsOption {
    /// Return log probabilities (true) or not (false).
    Bool(bool),
    /// Return log probabilities of the top n tokens.
    Number(u32),
}

/// Reasoning effort levels for reasoning models.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffort {
    /// Minimal reasoning effort.
    Minimal,
    /// Low reasoning effort.
    Low,
    /// Medium reasoning effort (default).
    Medium,
    /// High reasoning effort.
    High,
}

/// Service tier options for the OpenAI API.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceTier {
    /// Default service tier (project settings).
    Auto,
    /// 50% cheaper processing with increased latency (o3, o4-mini only).
    Flex,
    /// Higher-speed processing with low latency at premium cost (Enterprise).
    Priority,
    /// Standard pricing and performance.
    Default,
}

/// Text verbosity levels for model responses.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TextVerbosity {
    /// Low verbosity - concise responses.
    Low,
    /// Medium verbosity.
    Medium,
    /// High verbosity - verbose responses.
    High,
}

impl Default for OpenAIChatLanguageModelOptions {
    fn default() -> Self {
        Self {
            logit_bias: None,
            logprobs: None,
            parallel_tool_calls: None,
            user: None,
            reasoning_effort: None,
            max_completion_tokens: None,
            store: None,
            metadata: None,
            prediction: None,
            structured_outputs: Some(true), // Default to true
            service_tier: None,
            strict_json_schema: Some(false), // Default to false
            text_verbosity: None,
            prompt_cache_key: None,
            safety_identifier: None,
        }
    }
}

/// Check if a model is a reasoning model (o1, o3, etc.).
pub fn is_reasoning_model(model_id: &str) -> bool {
    model_id.starts_with("o1") || model_id.starts_with("o3") || model_id.starts_with("o4-mini")
}

/// Check if a model supports flex processing.
pub fn supports_flex_processing(model_id: &str) -> bool {
    model_id.starts_with("o3")
        || model_id.starts_with("o4-mini")
        || (model_id.starts_with("gpt-5") && !model_id.starts_with("gpt-5-chat"))
}

/// Check if a model supports priority processing.
pub fn supports_priority_processing(model_id: &str) -> bool {
    model_id.starts_with("gpt-4")
        || model_id.starts_with("gpt-5-mini")
        || (model_id.starts_with("gpt-5")
            && !model_id.starts_with("gpt-5-nano")
            && !model_id.starts_with("gpt-5-chat"))
        || model_id.starts_with("o3")
        || model_id.starts_with("o4-mini")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_reasoning_model() {
        assert!(is_reasoning_model("o1"));
        assert!(is_reasoning_model("o1-2024-12-17"));
        assert!(is_reasoning_model("o3-mini"));
        assert!(is_reasoning_model("o4-mini"));
        assert!(!is_reasoning_model("gpt-4"));
        assert!(!is_reasoning_model("gpt-4o"));
    }

    #[test]
    fn test_supports_flex_processing() {
        assert!(supports_flex_processing("o3"));
        assert!(supports_flex_processing("o4-mini"));
        assert!(supports_flex_processing("gpt-5"));
        assert!(!supports_flex_processing("gpt-5-chat"));
        assert!(!supports_flex_processing("gpt-4"));
    }

    #[test]
    fn test_supports_priority_processing() {
        assert!(supports_priority_processing("gpt-4"));
        assert!(supports_priority_processing("gpt-5"));
        assert!(supports_priority_processing("gpt-5-mini"));
        assert!(supports_priority_processing("o3"));
        assert!(supports_priority_processing("o4-mini"));
        assert!(!supports_priority_processing("gpt-5-nano"));
        assert!(!supports_priority_processing("gpt-5-chat"));
    }
}
