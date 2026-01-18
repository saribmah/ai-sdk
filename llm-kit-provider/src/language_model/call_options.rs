use crate::language_model::prompt::LanguageModelPrompt;
use crate::language_model::tool::LanguageModelTool;
use crate::language_model::tool_choice::LanguageModelToolChoice;
use crate::shared::provider_options::SharedProviderOptions;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tokio_util::sync;

/// Language model call options.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageModelCallOptions {
    /// A language mode prompt is a standardized prompt type.
    ///
    /// Note: This is **not** the user-facing prompt. The LLM Kit methods will map the
    /// user-facing prompt types such as chat or instruction prompts to this format.
    /// That approach allows us to evolve the user facing prompts without breaking
    /// the language model interface.
    pub prompt: LanguageModelPrompt,

    /// Maximum number of tokens to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,

    /// Temperature setting. The range depends on the provider and model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,

    /// Stop sequences.
    /// If set, the model will stop generating text when one of the stop sequences is generated.
    /// Providers may have limits on the number of stop sequences.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,

    /// Nucleus sampling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,

    /// Only sample from the top K options for each subsequent token.
    ///
    /// Used to remove "long tail" low probability responses.
    /// Recommended for advanced use cases only. You usually only need to use temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,

    /// Presence penalty setting. It affects the likelihood of the model to
    /// repeat information that is already in the prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,

    /// Frequency penalty setting. It affects the likelihood of the model
    /// to repeatedly use the same words or phrases.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,

    /// Response format. The output can either be text or JSON. Default is text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<LanguageModelResponseFormat>,

    /// The seed (integer) to use for random sampling. If set and supported
    /// by the model, calls will generate deterministic results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,

    /// The tools that are available for the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<LanguageModelTool>>,

    /// Specifies how the tool should be selected. Defaults to 'auto'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<LanguageModelToolChoice>,

    /// Include raw chunks in the stream. Only applicable for streaming calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_raw_chunks: Option<bool>,

    /// Additional HTTP headers to be sent with the request.
    /// Only applicable for HTTP-based providers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,

    /// Additional provider-specific options. They are passed through
    /// to the provider from the LLM Kit and enable provider-specific
    /// functionality that can be fully encapsulated in the provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<SharedProviderOptions>,

    /// Abort/cancellation signal (not serialized, used for runtime control).
    #[serde(skip)]
    pub abort_signal: Option<sync::CancellationToken>,
}

/// Response format specification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum LanguageModelResponseFormat {
    /// Text output format
    Text,

    /// JSON output format with optional schema
    #[serde(rename_all = "camelCase")]
    Json {
        /// JSON schema that the generated output should conform to.
        #[serde(skip_serializing_if = "Option::is_none")]
        schema: Option<Value>,

        /// Name of output that should be generated.
        /// Used by some providers for additional LLM guidance.
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,

        /// Description of the output that should be generated.
        /// Used by some providers for additional LLM guidance.
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
}

impl LanguageModelCallOptions {
    /// Create new call options with just a prompt
    pub fn new(prompt: LanguageModelPrompt) -> Self {
        Self {
            prompt,
            max_output_tokens: None,
            temperature: None,
            stop_sequences: None,
            top_p: None,
            top_k: None,
            presence_penalty: None,
            frequency_penalty: None,
            response_format: None,
            seed: None,
            tools: None,
            tool_choice: None,
            include_raw_chunks: None,
            headers: None,
            provider_options: None,
            abort_signal: None,
        }
    }

    /// Set the maximum number of tokens to generate.
    pub fn with_max_output_tokens(mut self, tokens: u32) -> Self {
        self.max_output_tokens = Some(tokens);
        self
    }

    /// Set the temperature for sampling (0.0 to 2.0).
    pub fn with_temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Set sequences that will stop generation when encountered.
    pub fn with_stop_sequences(mut self, sequences: Vec<String>) -> Self {
        self.stop_sequences = Some(sequences);
        self
    }

    /// Set the nucleus sampling parameter (0.0 to 1.0).
    pub fn with_top_p(mut self, top_p: f64) -> Self {
        self.top_p = Some(top_p);
        self
    }

    /// Set the top-K sampling parameter.
    pub fn with_top_k(mut self, top_k: u32) -> Self {
        self.top_k = Some(top_k);
        self
    }

    /// Set the presence penalty (-2.0 to 2.0).
    pub fn with_presence_penalty(mut self, penalty: f64) -> Self {
        self.presence_penalty = Some(penalty);
        self
    }

    /// Set the frequency penalty (-2.0 to 2.0).
    pub fn with_frequency_penalty(mut self, penalty: f64) -> Self {
        self.frequency_penalty = Some(penalty);
        self
    }

    /// Set the response format (e.g., JSON mode).
    pub fn with_response_format(mut self, format: LanguageModelResponseFormat) -> Self {
        self.response_format = Some(format);
        self
    }

    /// Set the random seed for deterministic generation.
    pub fn with_seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Set the tools available for the model to call.
    pub fn with_tools(mut self, tools: Vec<LanguageModelTool>) -> Self {
        self.tools = Some(tools);
        self
    }

    /// Set the tool choice strategy.
    pub fn with_tool_choice(mut self, choice: LanguageModelToolChoice) -> Self {
        self.tool_choice = Some(choice);
        self
    }

    /// Set whether to include raw response chunks in the stream.
    pub fn with_include_raw_chunks(mut self, include: bool) -> Self {
        self.include_raw_chunks = Some(include);
        self
    }

    /// Set additional HTTP headers for the request.
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Set provider-specific options.
    pub fn with_provider_options(mut self, options: SharedProviderOptions) -> Self {
        self.provider_options = Some(options);
        self
    }

    /// Set an abort signal to cancel the request.
    pub fn with_abort_signal(mut self, signal: sync::CancellationToken) -> Self {
        self.abort_signal = Some(signal);
        self
    }
}

impl LanguageModelResponseFormat {
    /// Create a text response format
    pub fn text() -> Self {
        Self::Text
    }

    /// Create a JSON response format without schema
    pub fn json() -> Self {
        Self::Json {
            schema: None,
            name: None,
            description: None,
        }
    }

    /// Create a JSON response format with schema
    pub fn json_with_schema(schema: Value) -> Self {
        Self::Json {
            schema: Some(schema),
            name: None,
            description: None,
        }
    }

    /// Create a JSON response format with all options
    pub fn json_with_options(
        schema: Option<Value>,
        name: Option<String>,
        description: Option<String>,
    ) -> Self {
        Self::Json {
            schema,
            name,
            description,
        }
    }
}
