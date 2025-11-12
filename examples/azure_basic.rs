/// Example demonstrating basic Azure OpenAI provider usage
///
/// This example shows how to:
/// - Create an Azure OpenAI provider
/// - Use different authentication methods
/// - Generate text with a chat model
/// - Use embeddings
/// - Handle different URL formats
///
/// Prerequisites:
/// - Set AZURE_RESOURCE_NAME or AZURE_BASE_URL environment variable
/// - Set AZURE_API_KEY environment variable
/// - Deploy models in Azure OpenAI (e.g., gpt-4, text-embedding-ada-002)
use ai_sdk_azure::{AzureOpenAIProviderSettings, create_azure};
use ai_sdk_core::prompt::Prompt;
use ai_sdk_core::{Embed, GenerateText};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔷 Azure OpenAI Provider Examples\n");

    // Example 1: Using resource name (most common)
    println!("Example 1: Chat with resource name");
    println!("=====================================");
    example_with_resource_name().await?;

    // Example 2: Using custom base URL
    println!("\nExample 2: Chat with custom base URL");
    println!("=====================================");
    example_with_base_url().await?;

    // Example 3: Using deployment-based URLs
    println!("\nExample 3: Chat with deployment-based URLs");
    println!("==========================================");
    example_with_deployment_based_urls().await?;

    // Example 4: Text embeddings
    println!("\nExample 4: Text embeddings");
    println!("==========================");
    example_embeddings().await?;

    // Example 5: Using default provider from environment
    println!("\nExample 5: Using default provider");
    println!("==================================");
    example_default_provider().await?;

    Ok(())
}

/// Example 1: Using Azure OpenAI with resource name
async fn example_with_resource_name() -> Result<(), Box<dyn std::error::Error>> {
    // Get credentials from environment
    let resource_name = std::env::var("AZURE_RESOURCE_NAME")
        .expect("AZURE_RESOURCE_NAME environment variable not set");
    let api_key =
        std::env::var("AZURE_API_KEY").expect("AZURE_API_KEY environment variable not set");

    // Create provider with resource name
    // This constructs URL: https://{resource_name}.openai.azure.com/openai/v1
    let provider = create_azure(
        AzureOpenAIProviderSettings::new()
            .with_resource_name(resource_name)
            .with_api_key(api_key)
            .with_api_version("2024-02-15-preview"), // Optional: specify API version
    );

    // Get a chat model using your deployment name
    // Replace "gpt-4" with your actual deployment name in Azure
    let model = provider.chat_model("gpt-4");

    println!("Model: {}", model.model_id());
    println!("Provider: {}", model.provider());

    // Generate text
    let result = GenerateText::new(model, Prompt::text("What is Azure OpenAI?"))
        .max_output_tokens(100)
        .execute()
        .await?;

    println!("\nResponse: {}", result.text);
    println!(
        "Tokens used: {} input, {} output",
        result.usage.input_tokens, result.usage.output_tokens
    );

    Ok(())
}

/// Example 2: Using Azure OpenAI with custom base URL
async fn example_with_base_url() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        std::env::var("AZURE_API_KEY").expect("AZURE_API_KEY environment variable not set");

    // You can also use a custom base URL instead of resource name
    // Useful for proxy servers or custom endpoints
    let base_url = std::env::var("AZURE_BASE_URL").unwrap_or_else(|_| {
        let resource_name = std::env::var("AZURE_RESOURCE_NAME")
            .expect("Either AZURE_BASE_URL or AZURE_RESOURCE_NAME must be set");
        format!("https://{}.openai.azure.com/openai", resource_name)
    });

    let provider = create_azure(
        AzureOpenAIProviderSettings::new()
            .with_base_url(base_url)
            .with_api_key(api_key),
    );

    let model = provider.chat_model("gpt-4");

    let result = GenerateText::new(model, Prompt::text("Count from 1 to 5"))
        .execute()
        .await?;

    println!("Response: {}", result.text);

    Ok(())
}

/// Example 3: Using deployment-based URLs (legacy format)
async fn example_with_deployment_based_urls() -> Result<(), Box<dyn std::error::Error>> {
    let resource_name = std::env::var("AZURE_RESOURCE_NAME")
        .expect("AZURE_RESOURCE_NAME environment variable not set");
    let api_key =
        std::env::var("AZURE_API_KEY").expect("AZURE_API_KEY environment variable not set");

    // Enable deployment-based URLs
    // Format: https://{resource}.openai.azure.com/openai/deployments/{deployment}/chat/completions
    let provider = create_azure(
        AzureOpenAIProviderSettings::new()
            .with_resource_name(resource_name)
            .with_api_key(api_key)
            .with_use_deployment_based_urls(true), // Enable legacy format
    );

    let model = provider.chat_model("gpt-4");

    let result = GenerateText::new(model, Prompt::text("Say hello"))
        .execute()
        .await?;

    println!("Response: {}", result.text);

    Ok(())
}

/// Example 4: Using text embeddings
async fn example_embeddings() -> Result<(), Box<dyn std::error::Error>> {
    let resource_name = std::env::var("AZURE_RESOURCE_NAME")
        .expect("AZURE_RESOURCE_NAME environment variable not set");
    let api_key =
        std::env::var("AZURE_API_KEY").expect("AZURE_API_KEY environment variable not set");

    let provider = create_azure(
        AzureOpenAIProviderSettings::new()
            .with_resource_name(resource_name)
            .with_api_key(api_key),
    );

    // Use your embedding model deployment name
    let embedding_model = provider.text_embedding_model("text-embedding-ada-002");

    println!("Embedding model: {}", embedding_model.model_id());

    // Generate embeddings
    let result = Embed::new(
        embedding_model,
        "Azure OpenAI provides powerful AI capabilities".to_string(),
    )
    .execute()
    .await?;

    println!("Embedding dimensions: {}", result.embedding.len());
    println!(
        "First 5 values: {:?}",
        &result.embedding[..5.min(result.embedding.len())]
    );

    Ok(())
}

/// Example 5: Using the default provider from environment variables
async fn example_default_provider() -> Result<(), Box<dyn std::error::Error>> {
    // This reads AZURE_RESOURCE_NAME/AZURE_BASE_URL and AZURE_API_KEY from environment
    use ai_sdk_azure::azure;

    let provider = azure();
    let model = provider.chat_model("gpt-4");

    let result = GenerateText::new(model, Prompt::text("What's 2+2?"))
        .execute()
        .await?;

    println!("Response: {}", result.text);

    Ok(())
}
