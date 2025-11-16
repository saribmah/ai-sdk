//! # Azure OpenAI Provider for AI SDK Rust
//!
//! This crate provides an Azure OpenAI provider implementation for the AI SDK.
//! It allows you to use Azure OpenAI models for text generation, embeddings,
//! and image generation.
//!
//! ## Features
//!
//! - **Chat Models**: GPT-4, GPT-3.5-turbo, and other chat models
//! - **Completion Models**: GPT-3.5-turbo-instruct and other completion models
//! - **Embedding Models**: text-embedding-ada-002 and other embedding models
//! - **Image Models**: DALL-E 3 and other image generation models
//! - **Azure-specific Authentication**: Uses `api-key` header
//! - **Flexible URL Formats**: Supports both v1 API and deployment-based URLs
//!
//! ## Quick Start (Recommended: Builder Pattern)
//!
//! ```no_run
//! use ai_sdk_azure::AzureClient;
//! use ai_sdk_core::GenerateText;
//! use ai_sdk_core::prompt::Prompt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create Azure OpenAI provider using the builder
//!     let provider = AzureClient::new()
//!         .resource_name("my-azure-resource")
//!         .api_key("your-api-key")
//!         .build();
//!
//!     // Get a chat model using your deployment name
//!     let model = provider.chat_model("gpt-4-deployment");
//!
//!     // Generate text
//!     let result = GenerateText::new(model, Prompt::text("Hello, Azure!"))
//!         .execute()
//!         .await?;
//!
//!     println!("Response: {}", result.text);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Alternative: Direct Instantiation
//!
//! ```no_run
//! use ai_sdk_azure::{AzureOpenAIProvider, AzureOpenAIProviderSettings};
//! use ai_sdk_core::GenerateText;
//! use ai_sdk_core::prompt::Prompt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create provider with settings
//!     let provider = AzureOpenAIProvider::new(
//!         AzureOpenAIProviderSettings::new()
//!             .with_resource_name("my-azure-resource")
//!             .with_api_key("your-api-key")
//!     );
//!
//!     let model = provider.chat_model("gpt-4-deployment");
//!     let result = GenerateText::new(model, Prompt::text("Hello, Azure!"))
//!         .execute()
//!         .await?;
//!
//!     println!("Response: {}", result.text);
//!     Ok(())
//! }
//! ```
//!
//! ## Configuration Options
//!
//! ### Using Resource Name
//!
//! ```no_run
//! use ai_sdk_azure::AzureClient;
//!
//! let provider = AzureClient::new()
//!     .resource_name("my-resource")
//!     .api_key("key")
//!     .build();
//! ```
//!
//! ### Using Custom Base URL
//!
//! ```no_run
//! use ai_sdk_azure::AzureClient;
//!
//! let provider = AzureClient::new()
//!     .base_url("https://my-resource.openai.azure.com/openai")
//!     .api_key("key")
//!     .build();
//! ```
//!
//! ### With Custom API Version
//!
//! ```no_run
//! use ai_sdk_azure::AzureClient;
//!
//! let provider = AzureClient::new()
//!     .resource_name("my-resource")
//!     .api_key("key")
//!     .api_version("2024-02-15-preview")
//!     .build();
//! ```
//!
//! ### With Custom Headers
//!
//! ```no_run
//! use ai_sdk_azure::AzureClient;
//!
//! let provider = AzureClient::new()
//!     .resource_name("my-resource")
//!     .api_key("key")
//!     .header("X-Custom-Header", "value")
//!     .build();
//! ```
//!
//! ### With Deployment-Based URLs (Legacy Format)
//!
//! ```no_run
//! use ai_sdk_azure::AzureClient;
//!
//! let provider = AzureClient::new()
//!     .resource_name("my-resource")
//!     .api_key("key")
//!     .use_deployment_based_urls(true)
//!     .build();
//! ```
//!
//! ## URL Formats
//!
//! Azure OpenAI supports two URL formats:
//!
//! ### V1 API Format (Default)
//! ```text
//! https://{resource}.openai.azure.com/openai/v1{path}?api-version={version}
//! ```
//!
//! ### Deployment-Based Format (Legacy)
//! ```text
//! https://{resource}.openai.azure.com/openai/deployments/{deployment}{path}?api-version={version}
//! ```
//!
//! Use `.with_use_deployment_based_urls(true)` to enable the legacy format.
//!
//! ## Environment Variables
//!
//! The provider will read from these environment variables if not explicitly configured:
//!
//! - `AZURE_API_KEY` - API key for authentication
//! - `AZURE_RESOURCE_NAME` - Azure OpenAI resource name
//!
//! ## Model Types
//!
//! ### Chat Models
//! Use `.chat_model()` or `.model()` for conversational AI:
//! ```no_run
//! # use ai_sdk_azure::AzureClient;
//! # let provider = AzureClient::new().resource_name("test").api_key("key").build();
//! let model = provider.chat_model("gpt-4-deployment");
//! ```
//!
//! ### Completion Models
//! Use `.completion_model()` for text completion:
//! ```no_run
//! # use ai_sdk_azure::AzureClient;
//! # let provider = AzureClient::new().resource_name("test").api_key("key").build();
//! let model = provider.completion_model("gpt-35-turbo-instruct");
//! ```
//!
//! ### Embedding Models
//! Use `.text_embedding_model()` for embeddings:
//! ```no_run
//! # use ai_sdk_azure::AzureClient;
//! # let provider = AzureClient::new().resource_name("test").api_key("key").build();
//! let model = provider.text_embedding_model("text-embedding-ada-002");
//! ```
//!
//! ### Image Models
//! Use `.image_model()` for image generation:
//! ```no_run
//! # use ai_sdk_azure::AzureClient;
//! # let provider = AzureClient::new().resource_name("test").api_key("key").build();
//! let model = provider.image_model("dall-e-3");
//! ```

mod client;
mod provider;
mod settings;

pub use client::AzureClient;
pub use provider::AzureOpenAIProvider;
pub use settings::AzureOpenAIProviderSettings;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_azure_client_builder() {
        let provider = AzureClient::new()
            .resource_name("test-resource")
            .api_key("test-key")
            .build();

        assert_eq!(provider.name(), "azure");
    }

    #[test]
    fn test_provider_direct_instantiation() {
        let provider = AzureOpenAIProvider::new(
            AzureOpenAIProviderSettings::new()
                .with_resource_name("test-resource")
                .with_api_key("test-key"),
        );

        assert_eq!(provider.name(), "azure");
    }

    #[test]
    fn test_provider_methods() {
        let provider = AzureClient::new()
            .resource_name("test-resource")
            .api_key("test-key")
            .build();

        // Test chat model
        let chat_model = provider.chat_model("gpt-4");
        assert_eq!(chat_model.provider(), "azure.chat");
        assert_eq!(chat_model.model_id(), "gpt-4");

        // Test completion model
        let completion_model = provider.completion_model("gpt-35-turbo-instruct");
        assert_eq!(completion_model.provider(), "azure.completion");
        assert_eq!(completion_model.model_id(), "gpt-35-turbo-instruct");

        // Test embedding model
        let embedding_model = provider.text_embedding_model("text-embedding-ada-002");
        assert_eq!(embedding_model.provider(), "azure.embedding");
        assert_eq!(embedding_model.model_id(), "text-embedding-ada-002");

        // Test image model
        let image_model = provider.image_model("dall-e-3");
        assert_eq!(image_model.provider(), "azure.image");
        assert_eq!(image_model.model_id(), "dall-e-3");
    }
}
