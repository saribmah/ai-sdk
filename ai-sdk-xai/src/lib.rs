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
//! use ai_sdk_provider::LanguageModel;
//! use ai_sdk_provider::language_model::call_options::LanguageModelCallOptions;
//! use ai_sdk_provider::language_model::prompt::LanguageModelMessage;
//! use ai_sdk_provider::language_model::tool::LanguageModelTool;
//! use ai_sdk_provider::language_model::tool::function_tool::LanguageModelFunctionTool;
//! use ai_sdk_xai::XaiClient;
//! use serde_json::json;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = XaiClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.chat_model("grok-beta");
//!
//! // Define a tool using ai-sdk-provider types
//! let weather_tool = LanguageModelFunctionTool::new(
//!     "get_weather",
//!     json!({
//!         "type": "object",
//!         "properties": {
//!             "city": {"type": "string", "description": "The city name"}
//!         },
//!         "required": ["city"]
//!     }),
//! )
//! .with_description("Get the current weather");
//!
//! let tools = vec![LanguageModelTool::Function(weather_tool)];
//!
//! let prompt = vec![LanguageModelMessage::user_text("What's the weather in Tokyo?")];
//! let options = LanguageModelCallOptions::new(prompt).with_tools(tools);
//!
//! let result = model.do_generate(options).await?;
//! # Ok(())
//! # }
//! ```
//!
//! For full tool execution with ai-sdk-core, see the examples directory.
//!
//! ## Image Generation
//!
//! ```no_run
//! use ai_sdk_provider::ImageModel;
//! use ai_sdk_provider::image_model::call_options::ImageModelCallOptions;
//! use ai_sdk_xai::XaiClient;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = XaiClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.image_model("grok-2-vision-1212");
//!
//! let options = ImageModelCallOptions::new(
//!     "A futuristic cityscape at sunset".to_string(),
//!     1
//! );
//!
//! let result = model.do_generate(options).await?;
//!
//! println!("Generated {} image(s)", result.images.len());
//! # Ok(())
//! # }
//! ```
//!
//! For more image generation examples with ai-sdk-core, see the examples directory.

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
