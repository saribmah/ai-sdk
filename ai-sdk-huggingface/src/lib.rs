//! # Hugging Face Provider for AI SDK Rust
//!
//! This crate provides a Hugging Face provider implementation for the AI SDK,
//! supporting the Hugging Face Responses API.
//!
//! ## Features
//!
//! - Text generation with streaming support
//! - Tool calling (function calling)
//! - Multimodal inputs (text + images)
//! - Reasoning content
//! - Source annotations
//! - Structured output (JSON schema)
//!
//! ## Example
//!
//! ```ignore
//! use ai_sdk_huggingface::{create_huggingface, HuggingFaceProviderSettings};
//! use ai_sdk_core::{GenerateText, prompt::Prompt};
//!
//! #[tokio::main]
//! async fn main() {
//!     let provider = create_huggingface(
//!         HuggingFaceProviderSettings::new()
//!             .with_api_key("your-api-key")
//!             .load_api_key_from_env()
//!     );
//!
//!     let model = provider.responses("meta-llama/Llama-3.1-8B-Instruct");
//!
//!     let result = GenerateText::new(model, Prompt::text("Hello!"))
//!         .execute()
//!         .await
//!         .unwrap();
//!
//!     println!("{}", result.text);
//! }
//! ```

pub mod client;
pub mod error;
pub mod provider;
pub mod responses;
pub mod settings;

// Re-exports
pub use error::{HuggingFaceErrorData, HuggingFaceErrorDetail};
pub use provider::{HuggingFaceProvider, create_huggingface};
pub use responses::{
    HuggingFaceResponsesModelId, HuggingFaceResponsesSettings, HuggingFaceResponsesTool,
    HuggingFaceResponsesToolChoice,
};
pub use settings::HuggingFaceProviderSettings;

// Re-export common model constants
pub use responses::settings::{
    DEEPSEEK_R1, DEEPSEEK_V3_1, GEMMA_2_9B_IT, KIMI_K2_INSTRUCT, LLAMA_3_1_8B_INSTRUCT,
    LLAMA_3_1_70B_INSTRUCT, LLAMA_3_3_70B_INSTRUCT, QWEN2_5_7B_INSTRUCT, QWEN3_32B,
};
