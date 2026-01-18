//! Groq provider implementation for the AI SDK.
//!
//! This crate provides a provider implementation for Groq's chat models,
//! supporting ultra-fast inference with open-source models.
//!
//! # Features
//!
//! - Chat completions with Llama, Gemma, and other open-source models
//! - Ultra-fast inference speeds
//! - Streaming support
//! - Tool calling
//! - Whisper transcription support
//! - Groq-specific metadata (cached tokens)
//! - Image URL support
//!
//! # Examples
//!
//! ## Basic Usage with Client Builder (Recommended)
//!
//! ```no_run
//! use llm_kit_groq::GroqClient;
//!
//! // Create a provider using the client builder
//! let provider = GroqClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.chat_model("llama-3.1-8b-instant");
//! ```
//!
//! ## Alternative: Direct Instantiation
//!
//! ```no_run
//! use llm_kit_groq::{GroqProvider, GroqProviderSettings};
//!
//! // Create a provider using settings
//! let provider = GroqProvider::new(
//!     GroqProviderSettings::new()
//!         .with_api_key("your-api-key")
//! );
//!
//! let model = provider.chat_model("llama-3.3-70b-versatile");
//! ```
//!
//! ## Chained Usage
//!
//! ```no_run
//! use llm_kit_groq::GroqClient;
//!
//! let model = GroqClient::new()
//!     .api_key("your-api-key")
//!     .build()
//!     .chat_model("llama-3.1-8b-instant");
//! ```
//!
//! ## Environment Variable
//!
//! ```no_run
//! use llm_kit_groq::GroqClient;
//!
//! // API key will be read from GROQ_API_KEY environment variable
//! let provider = GroqClient::new()
//!     .load_api_key_from_env()
//!     .build();
//!
//! let model = provider.chat_model("llama-3.1-8b-instant");
//! ```
//!
//! ## Text Generation
//!
//! ```no_run
//! use llm_kit_groq::GroqClient;
//! use llm_kit_provider::LanguageModel;
//! use llm_kit_provider::language_model::call_options::LanguageModelCallOptions;
//! use llm_kit_provider::language_model::prompt::LanguageModelMessage;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = GroqClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.chat_model("llama-3.1-8b-instant");
//!
//! let prompt = vec![LanguageModelMessage::user_text(
//!     "Explain quantum computing in simple terms"
//! )];
//!
//! let options = LanguageModelCallOptions::new(prompt)
//!     .with_temperature(0.7)
//!     .with_max_output_tokens(500);
//!
//! let result = model.do_generate(options).await?;
//!
//! // Extract text from content
//! let text = result.content.iter()
//!     .filter_map(|c| match c {
//!         llm_kit_provider::language_model::content::LanguageModelContent::Text(t) => Some(t.text.clone()),
//!         _ => None,
//!     })
//!     .collect::<Vec<_>>()
//!     .join("");
//!
//! println!("Response: {}", text);
//! # Ok(())
//! # }
//! ```
//!
//! ## Streaming
//!
//! ```no_run
//! use llm_kit_groq::GroqClient;
//! use llm_kit_provider::LanguageModel;
//! use llm_kit_provider::language_model::call_options::LanguageModelCallOptions;
//! use llm_kit_provider::language_model::prompt::LanguageModelMessage;
//! use llm_kit_provider::language_model::stream_part::LanguageModelStreamPart;
//! use futures_util::StreamExt;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = GroqClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.chat_model("llama-3.1-8b-instant");
//!
//! let prompt = vec![LanguageModelMessage::user_text("Tell me a story")];
//! let options = LanguageModelCallOptions::new(prompt);
//!
//! let result = model.do_stream(options).await?;
//! let mut stream = result.stream;
//!
//! // Stream text deltas
//! while let Some(part) = stream.next().await {
//!     if let LanguageModelStreamPart::TextDelta(delta) = part {
//!         print!("{}", delta.delta);
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Tool Calling
//!
//! Groq supports tool/function calling. See the `examples/chat_tool_calling.rs`  
//! and `examples/stream_tool_calling.rs` examples for complete working implementations.
//!
//! ## Groq-Specific Metadata
//!
//! Groq provides cached token statistics in the metadata:
//!
//! ```no_run
//! use llm_kit_groq::GroqClient;
//! use llm_kit_provider::LanguageModel;
//! use llm_kit_provider::language_model::call_options::LanguageModelCallOptions;
//! use llm_kit_provider::language_model::prompt::LanguageModelMessage;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = GroqClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.chat_model("llama-3.1-8b-instant");
//!
//! let prompt = vec![LanguageModelMessage::user_text("Hello")];
//! let options = LanguageModelCallOptions::new(prompt);
//!
//! let result = model.do_generate(options).await?;
//!
//! // Access Groq-specific metadata
//! if let Some(provider_metadata) = &result.provider_metadata {
//!     if let Some(groq) = provider_metadata.get("groq") {
//!         println!("Cached tokens: {:?}", groq.get("cachedTokens"));
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Transcription (Whisper)
//!
//! Groq provides ultra-fast Whisper transcription:
//!
//! ```no_run
//! use llm_kit_groq::{GroqClient, GroqTranscriptionOptions};
//! use llm_kit_provider::transcription_model::call_options::TranscriptionModelCallOptions;
//! use llm_kit_provider::TranscriptionModel;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = GroqClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.transcription_model("whisper-large-v3");
//!
//! // Read audio file
//! let audio_data = std::fs::read("audio.mp3")?;
//!
//! // Configure options
//! let mut provider_options = std::collections::HashMap::new();
//! let mut groq_opts_map = std::collections::HashMap::new();
//! let groq_options = GroqTranscriptionOptions::new()
//!     .with_language("en")
//!     .with_verbose_json();
//!
//! // Convert to nested structure
//! let groq_value = serde_json::to_value(&groq_options)?;
//! if let serde_json::Value::Object(map) = groq_value {
//!     for (k, v) in map {
//!         groq_opts_map.insert(k, v);
//!     }
//! }
//! provider_options.insert("groq".to_string(), groq_opts_map);
//!
//! let options = TranscriptionModelCallOptions::mp3(audio_data)
//!     .with_provider_options(provider_options);
//!
//! let result = model.do_generate(options).await?;
//! println!("Transcription: {}", result.text);
//! # Ok(())
//! # }
//! ```

/// Chat completion implementation for Groq models.
pub mod chat;

/// Client builder for creating Groq providers.
pub mod client;

/// Error types for Groq provider operations.
pub mod error;

/// Provider implementation and creation functions.
pub mod provider;

/// Settings and configuration for Groq providers.
pub mod settings;

/// Speech synthesis implementation for Groq text-to-speech models.
pub mod speech;

/// Transcription implementation for Groq Whisper models.
pub mod transcription;

// Re-export main types from chat
pub use chat::{
    GroqChatConfig, GroqChatLanguageModel, GroqChatModelId, GroqMetadataExtractor,
    GroqProviderOptions, GroqUsage, ReasoningFormat, ServiceTier,
};

pub use client::GroqClient;
pub use error::{GroqError, GroqErrorData};
pub use provider::GroqProvider;
pub use settings::GroqProviderSettings;

// Re-export speech types
pub use speech::{GroqSpeechConfig, GroqSpeechModel, GroqSpeechOptions};

// Re-export transcription types
pub use transcription::{
    GroqTranscriptionConfig, GroqTranscriptionModel, GroqTranscriptionModelId,
    GroqTranscriptionOptions,
};
