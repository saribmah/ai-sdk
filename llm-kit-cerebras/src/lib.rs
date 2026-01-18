//! Cerebras provider for the AI SDK.
//!
//! This crate provides a provider implementation for Cerebras, offering high-speed
//! AI model inference powered by Cerebras Wafer-Scale Engines and CS-3 systems.
//!
//! # Features
//!
//! - **Chat Completions**: Full support for chat-based language models
//! - **Streaming**: Real-time streaming of model responses
//! - **Tool Calling**: Function calling capabilities for building agents
//! - **Structured Outputs**: JSON schema-based structured output generation
//! - **Reasoning Models**: Support for reasoning/thinking models
//!
//! # Examples
//!
//! ## Basic Usage with Client Builder (Recommended)
//!
//! ```no_run
//! use llm_kit_cerebras::CerebrasClient;
//!
//! // Create a provider using the client builder
//! let provider = CerebrasClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.chat_model("llama-3.3-70b");
//! ```
//!
//! ## Alternative: Using Settings Directly
//!
//! ```no_run
//! use llm_kit_cerebras::{CerebrasProvider, CerebrasProviderSettings};
//!
//! // Create a provider using settings
//! let provider = CerebrasProvider::new(
//!     CerebrasProviderSettings::new("https://api.cerebras.ai/v1")
//!         .with_api_key("your-api-key")
//! );
//!
//! let model = provider.chat_model("llama-3.3-70b");
//! ```
//!
//! ## Chained Usage
//!
//! ```no_run
//! use llm_kit_cerebras::CerebrasClient;
//!
//! let model = CerebrasClient::new()
//!     .api_key("your-api-key")
//!     .build()
//!     .chat_model("llama-3.3-70b");
//! ```
//!
//! ## Using Model Constants
//!
//! ```no_run
//! use llm_kit_cerebras::{CerebrasClient, chat::models};
//!
//! let provider = CerebrasClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! // Use predefined model constants
//! let model = provider.chat_model(models::LLAMA_3_3_70B);
//! ```
//!
//! ## Custom Headers
//!
//! ```no_run
//! use llm_kit_cerebras::CerebrasClient;
//!
//! let provider = CerebrasClient::new()
//!     .api_key("your-api-key")
//!     .header("X-Custom-Header", "value")
//!     .build();
//!
//! let model = provider.chat_model("llama-3.3-70b");
//! ```
//!
//! ## Environment Variable for API Key
//!
//! The provider will automatically read the API key from the `CEREBRAS_API_KEY`
//! environment variable if not provided explicitly:
//!
//! ```no_run
//! use llm_kit_cerebras::CerebrasClient;
//!
//! // API key will be read from CEREBRAS_API_KEY environment variable
//! let provider = CerebrasClient::new().build();
//!
//! let model = provider.chat_model("llama-3.3-70b");
//! ```
//!
//! # Available Models
//!
//! Cerebras offers several high-performance language models:
//!
//! ## Production Models
//! - `llama3.1-8b` - Llama 3.1 8B parameter model
//! - `llama-3.3-70b` - Llama 3.3 70B parameter model
//! - `gpt-oss-120b` - GPT-OSS 120B parameter model
//! - `qwen-3-32b` - Qwen 3 32B parameter model
//!
//! ## Preview Models
//! - `qwen-3-235b-a22b-instruct-2507` - Qwen 3 235B instruct model
//! - `qwen-3-235b-a22b-thinking-2507` - Qwen 3 235B thinking/reasoning model
//! - `zai-glm-4.6` - ZAI GLM 4.6 model
//!
//! For more information, see: <https://inference-docs.cerebras.ai/models/overview>
//!
//! # Note
//!
//! Due to high demand in the early launch phase, context windows are temporarily
//! limited to 8192 tokens in the Free Tier.

/// Chat model types and identifiers
pub mod chat;

/// Client builder for creating Cerebras providers
pub mod client;

/// Error types for Cerebras operations
pub mod error;

/// Provider implementation and creation functions
pub mod provider;

/// Settings and configuration for Cerebras providers
pub mod settings;

// Re-export main types
pub use chat::CerebrasChatModelId;
pub use client::CerebrasClient;
pub use error::CerebrasErrorData;
pub use provider::CerebrasProvider;
pub use settings::CerebrasProviderSettings;

/// Default Cerebras provider instance using environment variables.
///
/// This creates a provider that will read the API key from the
/// `CEREBRAS_API_KEY` environment variable.
///
/// # Examples
///
/// ```no_run
/// use llm_kit_cerebras::cerebras;
///
/// let model = cerebras().chat_model("llama-3.3-70b");
/// ```
pub fn cerebras() -> CerebrasProvider {
    CerebrasClient::new().build()
}
