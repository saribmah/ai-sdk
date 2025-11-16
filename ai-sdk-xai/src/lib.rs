//! xAI (Grok) provider implementation for the AI SDK.
//!
//! This crate provides a provider implementation for xAI's Grok models,
//! supporting chat completions, image generation, reasoning modes, and integrated search.
//!
//! # Examples
//!
//! ## Basic Usage with Client Builder (Recommended)
//!
//! ```no_run
//! use ai_sdk_xai::XaiClient;
//!
//! // Create a provider using the client builder
//! let provider = XaiClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.chat_model("grok-4");
//! ```
//!
//! ## Alternative: Direct Instantiation
//!
//! ```no_run
//! use ai_sdk_xai::{XaiProvider, XaiProviderSettings};
//!
//! // Create a provider using settings directly
//! let provider = XaiProvider::new(
//!     XaiProviderSettings::new()
//!         .with_api_key("your-api-key")
//! );
//!
//! let model = provider.chat_model("grok-4");
//! ```
//!
//! ## Chained Usage
//!
//! ```no_run
//! use ai_sdk_xai::XaiClient;
//!
//! let model = XaiClient::new()
//!     .api_key("your-api-key")
//!     .build()
//!     .chat_model("grok-3-fast");
//! ```
//!
//! ## Environment Variable
//!
//! ```no_run
//! use ai_sdk_xai::XaiClient;
//!
//! // API key will be read from XAI_API_KEY environment variable
//! let provider = XaiClient::new().build();
//! let model = provider.chat_model("grok-4-fast-reasoning");
//! ```
//!
//! ## Tool Calling
//!
//! ```no_run
//! use ai_sdk_xai::XaiClient;
//! use ai_sdk_core::{GenerateText, ToolSet, prompt::Prompt};
//! use ai_sdk_provider_utils::tool::{Tool, ToolExecutionOutput};
//! use serde_json::json;
//! use std::sync::Arc;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = XaiClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.chat_model("grok-2-latest");
//!
//! // Define a tool
//! let weather_tool = Tool::function(json!({
//!     "type": "object",
//!     "properties": {
//!         "city": {"type": "string", "description": "The city name"}
//!     },
//!     "required": ["city"]
//! }))
//! .with_description("Get the current weather")
//! .with_execute(Arc::new(|input, _| {
//!     let result = json!({"temperature": 72, "conditions": "sunny"});
//!     ToolExecutionOutput::Single(Box::pin(async move { Ok(result) }))
//! }));
//!
//! let mut tools = ToolSet::new();
//! tools.insert("get_weather".to_string(), weather_tool);
//!
//! let result = GenerateText::new(
//!     model,
//!     Prompt::text("What's the weather in Tokyo?")
//! )
//! .tools(tools)
//! .execute()
//! .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Image Generation
//!
//! ```no_run
//! use ai_sdk_xai::XaiClient;
//! use ai_sdk_core::GenerateImage;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = XaiClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.image_model("grok-2-image");
//!
//! let result = GenerateImage::new(
//!     model,
//!     "A futuristic cityscape at sunset".to_string(),
//! ).execute().await?;
//!
//! println!("Generated {} image(s)", result.images.len());
//! # Ok(())
//! # }
//! ```

/// Chat completion implementation for xAI models.
pub mod chat;

/// Client builder for creating xAI providers.
pub mod client;

/// Error types for xAI provider operations.
pub mod error;

/// Provider implementation and creation functions.
pub mod provider;

/// Settings and configuration for xAI providers.
pub mod settings;

// Re-export main types from chat
pub use chat::{
    SearchParameters, SearchSource, XaiChatLanguageModel, XaiChatMessage, XaiChatModelId,
    XaiProviderOptions, XaiUserContent, convert_to_xai_chat_messages,
};

pub use client::XaiClient;
pub use error::{XaiErrorData, XaiErrorDetails};
pub use provider::XaiProvider;
pub use settings::XaiProviderSettings;
