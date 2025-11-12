//! OpenAI Chat API Request and Response Types
//!
//! This module defines the API request and response structures for OpenAI's chat completion endpoint.

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// Tool calling function definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIChatFunctionTool {
    /// The type of the tool (always "function" for function tools).
    #[serde(rename = "type")]
    pub tool_type: String,
    /// Function details.
    pub function: FunctionDefinition,
}

/// Function definition for tool calling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDefinition {
    /// The name of the function.
    pub name: String,
    /// Description of what the function does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// JSON Schema for the function parameters.
    pub parameters: JsonValue,
    /// Whether to use strict schema validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

/// Tool choice strategy for function calling.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenAIChatToolChoice {
    /// Let the model decide.
    Auto(String), // "auto"
    /// No tool calling.
    None(String), // "none"
    /// Tool calling is required.
    Required(String), // "required"
    /// Specific function to call.
    Function {
        /// Type field (always "function")
        #[serde(rename = "type")]
        tool_type: String,
        /// The function to call
        function: FunctionName,
    },
}

/// Function name for specific tool choice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionName {
    /// The name of the function to call.
    pub name: String,
}

/// Chat completion response from OpenAI API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIChatResponse {
    /// Unique identifier for the completion.
    pub id: Option<String>,
    /// Unix timestamp of when the completion was created.
    pub created: Option<i64>,
    /// Model used for the completion.
    pub model: Option<String>,
    /// List of completion choices.
    pub choices: Vec<ChatChoice>,
    /// Token usage information.
    pub usage: Option<Usage>,
}

/// A single choice in the chat completion response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChoice {
    /// The index of this choice in the list.
    pub index: u32,
    /// The generated message.
    pub message: ChatMessage,
    /// The reason generation stopped.
    pub finish_reason: Option<String>,
    /// Log probabilities (if requested).
    pub logprobs: Option<Logprobs>,
}

/// A message in the chat completion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// The role of the message author.
    pub role: Option<String>,
    /// The content of the message.
    pub content: Option<String>,
    /// Tool calls made by the assistant.
    pub tool_calls: Option<Vec<ToolCall>>,
    /// URL citations/annotations.
    pub annotations: Option<Vec<Annotation>>,
}

/// A tool call made by the assistant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// Unique identifier for the tool call.
    pub id: Option<String>,
    /// Type of tool (always "function").
    #[serde(rename = "type")]
    pub tool_type: String,
    /// Function call details.
    pub function: FunctionCall,
}

/// Function call details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    /// Name of the function.
    pub name: String,
    /// JSON string of function arguments.
    pub arguments: String,
}

/// URL citation annotation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    /// Type of annotation (e.g., "url_citation").
    #[serde(rename = "type")]
    pub annotation_type: String,
    /// Start index in the text.
    pub start_index: u32,
    /// End index in the text.
    pub end_index: u32,
    /// URL being cited.
    pub url: String,
    /// Title of the cited source.
    pub title: String,
}

/// Token usage information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    /// Number of tokens in the prompt.
    pub prompt_tokens: Option<u64>,
    /// Number of tokens in the completion.
    pub completion_tokens: Option<u64>,
    /// Total number of tokens.
    pub total_tokens: Option<u64>,
    /// Detailed prompt token breakdown.
    pub prompt_tokens_details: Option<PromptTokenDetails>,
    /// Detailed completion token breakdown.
    pub completion_tokens_details: Option<CompletionTokenDetails>,
}

/// Detailed prompt token information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTokenDetails {
    /// Number of cached tokens.
    pub cached_tokens: Option<u64>,
}

/// Detailed completion token information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionTokenDetails {
    /// Number of reasoning tokens (for reasoning models).
    pub reasoning_tokens: Option<u64>,
    /// Number of accepted prediction tokens.
    pub accepted_prediction_tokens: Option<u64>,
    /// Number of rejected prediction tokens.
    pub rejected_prediction_tokens: Option<u64>,
}

/// Log probabilities information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logprobs {
    /// Token log probabilities.
    pub content: Option<Vec<TokenLogprob>>,
}

/// Log probability for a single token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenLogprob {
    /// The token.
    pub token: String,
    /// Log probability of the token.
    pub logprob: f64,
    /// Top alternative tokens with their log probabilities.
    pub top_logprobs: Vec<TopLogprob>,
}

/// Alternative token with log probability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopLogprob {
    /// The token.
    pub token: String,
    /// Log probability of the token.
    pub logprob: f64,
}

/// Streaming chat completion chunk from OpenAI API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIChatChunk {
    /// Unique identifier for the completion.
    pub id: Option<String>,
    /// Unix timestamp of when the chunk was created.
    pub created: Option<i64>,
    /// Model used for the completion.
    pub model: Option<String>,
    /// List of choice deltas.
    pub choices: Vec<ChatChoiceDelta>,
    /// Token usage information (typically in final chunk).
    pub usage: Option<Usage>,
}

/// A delta choice in the streaming response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChoiceDelta {
    /// The index of this choice.
    pub index: u32,
    /// The delta message.
    pub delta: Option<ChatMessageDelta>,
    /// The reason generation stopped (in final chunk).
    pub finish_reason: Option<String>,
    /// Log probabilities.
    pub logprobs: Option<Logprobs>,
}

/// Delta message in streaming response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessageDelta {
    /// Role of the message (usually only in first chunk).
    pub role: Option<String>,
    /// Content delta.
    pub content: Option<String>,
    /// Reasoning content delta (for reasoning models).
    pub reasoning_content: Option<String>,
    /// Alternative reasoning field (some providers).
    pub reasoning: Option<String>,
    /// Tool call deltas.
    pub tool_calls: Option<Vec<ToolCallDelta>>,
    /// Annotation deltas.
    pub annotations: Option<Vec<Annotation>>,
}

/// Tool call delta in streaming response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallDelta {
    /// Index of the tool call.
    pub index: Option<u64>,
    /// ID of the tool call (in first chunk).
    pub id: Option<String>,
    /// Type of tool (in first chunk).
    #[serde(rename = "type")]
    pub tool_type: Option<String>,
    /// Function call delta.
    pub function: Option<FunctionCallDelta>,
}

/// Function call delta.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCallDelta {
    /// Function name (in first chunk).
    pub name: Option<String>,
    /// Function arguments delta.
    pub arguments: Option<String>,
}
