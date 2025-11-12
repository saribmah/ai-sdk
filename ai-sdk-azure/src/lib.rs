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
//! ## Usage
//!
//! ```no_run
//! use ai_sdk_azure::{create_azure, AzureOpenAIProviderSettings};
//! use ai_sdk_core::GenerateText;
//! use ai_sdk_core::prompt::Prompt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create Azure OpenAI provider
//!     let provider = create_azure(
//!         AzureOpenAIProviderSettings::new()
//!             .with_resource_name("my-azure-resource")
//!             .with_api_key("your-api-key")
//!     );
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
//! ## Configuration
//!
//! The provider can be configured using `AzureOpenAIProviderSettings`:
//!
//! ```no_run
//! use ai_sdk_azure::{create_azure, AzureOpenAIProviderSettings};
//!
//! // Using resource name (constructs URL automatically)
//! let provider = create_azure(
//!     AzureOpenAIProviderSettings::new()
//!         .with_resource_name("my-resource")
//!         .with_api_key("key")
//! );
//!
//! // Using custom base URL
//! let provider = create_azure(
//!     AzureOpenAIProviderSettings::new()
//!         .with_base_url("https://my-resource.openai.azure.com/openai")
//!         .with_api_key("key")
//! );
//!
//! // With custom API version
//! let provider = create_azure(
//!     AzureOpenAIProviderSettings::new()
//!         .with_resource_name("my-resource")
//!         .with_api_key("key")
//!         .with_api_version("2024-02-15-preview")
//! );
//!
//! // With deployment-based URLs (legacy format)
//! let provider = create_azure(
//!     AzureOpenAIProviderSettings::new()
//!         .with_resource_name("my-resource")
//!         .with_api_key("key")
//!         .with_use_deployment_based_urls(true)
//! );
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
//! # use ai_sdk_azure::{create_azure, AzureOpenAIProviderSettings};
//! # let provider = create_azure(AzureOpenAIProviderSettings::new().with_resource_name("test").with_api_key("key"));
//! let model = provider.chat_model("gpt-4-deployment");
//! ```
//!
//! ### Completion Models
//! Use `.completion_model()` for text completion:
//! ```no_run
//! # use ai_sdk_azure::{create_azure, AzureOpenAIProviderSettings};
//! # let provider = create_azure(AzureOpenAIProviderSettings::new().with_resource_name("test").with_api_key("key"));
//! let model = provider.completion_model("gpt-35-turbo-instruct");
//! ```
//!
//! ### Embedding Models
//! Use `.text_embedding_model()` for embeddings:
//! ```no_run
//! # use ai_sdk_azure::{create_azure, AzureOpenAIProviderSettings};
//! # let provider = create_azure(AzureOpenAIProviderSettings::new().with_resource_name("test").with_api_key("key"));
//! let model = provider.text_embedding_model("text-embedding-ada-002");
//! ```
//!
//! ### Image Models
//! Use `.image_model()` for image generation:
//! ```no_run
//! # use ai_sdk_azure::{create_azure, AzureOpenAIProviderSettings};
//! # let provider = create_azure(AzureOpenAIProviderSettings::new().with_resource_name("test").with_api_key("key"));
//! let model = provider.image_model("dall-e-3");
//! ```

mod provider;
mod settings;

pub use provider::{create_azure, AzureOpenAIProvider};
pub use settings::AzureOpenAIProviderSettings;

/// Default Azure OpenAI provider instance.
///
/// Creates a provider using environment variables:
/// - `AZURE_RESOURCE_NAME` or `AZURE_BASE_URL`
/// - `AZURE_API_KEY`
///
/// # Panics
///
/// Panics if required environment variables are not set.
///
/// # Example
///
/// ```no_run
/// use ai_sdk_azure::{create_azure, AzureOpenAIProviderSettings};
/// use ai_sdk_core::GenerateText;
/// use ai_sdk_core::prompt::Prompt;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Create Azure OpenAI provider
///     let provider = create_azure(
///         AzureOpenAIProviderSettings::new()
///             .with_resource_name("my-azure-resource")
///             .with_api_key("your-api-key")
///     );
///
///     // Get a chat model using your deployment name
///     let model = provider.chat_model("gpt-4-deployment");
///
///     // Generate text
///     let result = GenerateText::new(model, Prompt::text("Hello, Azure!"))
///         .execute()
///         .await?;
///
///     println!("Response: {}", result.text);
///
///     Ok(())
/// }
/// ```
pub fn azure() -> AzureOpenAIProvider {
    let resource_name = std::env::var("AZURE_RESOURCE_NAME").ok();
    let base_url = std::env::var("AZURE_BASE_URL").ok();
    let api_key = std::env::var("AZURE_API_KEY").ok();

    let mut settings = AzureOpenAIProviderSettings::new();

    if let Some(name) = resource_name {
        settings = settings.with_resource_name(name);
    } else if let Some(url) = base_url {
        settings = settings.with_base_url(url);
    }

    if let Some(key) = api_key {
        settings = settings.with_api_key(key);
    }

    create_azure(settings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_azure() {
        let provider = create_azure(
            AzureOpenAIProviderSettings::new()
                .with_resource_name("test-resource")
                .with_api_key("test-key"),
        );

        assert_eq!(provider.name(), "azure");
    }

    #[test]
    fn test_provider_methods() {
        let provider = create_azure(
            AzureOpenAIProviderSettings::new()
                .with_resource_name("test-resource")
                .with_api_key("test-key"),
        );

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
