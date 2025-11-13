use ai_sdk_core::{GenerateText, prompt::Prompt};
/// xAI provider options example demonstrating xAI-specific features.
///
/// This example shows how to use provider-specific options to control
/// xAI's unique features like reasoning effort and web search integration.
///
/// Run with:
/// ```bash
/// export XAI_API_KEY="your-api-key"
/// cargo run --example xai_provider_options
/// ```
use ai_sdk_xai::XaiClient;
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 AI SDK Rust - xAI Provider Options Example\n");

    // Get API key from environment
    let api_key = env::var("XAI_API_KEY").map_err(
        |_| "XAI_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment\n");

    // Create xAI provider
    let provider = XaiClient::new().api_key(api_key).build();
    let model = provider.chat_model("grok-2-1212");

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 1: High Reasoning Effort");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Configure provider options for high reasoning effort
    let mut xai_options = HashMap::new();
    xai_options.insert("reasoningEffort".to_string(), serde_json::json!("high"));

    let mut provider_options = HashMap::new();
    provider_options.insert("xai".to_string(), xai_options);

    let result = GenerateText::new(
        model.clone(),
        Prompt::text("Explain the concept of quantum entanglement in simple terms."),
    )
    .provider_options(provider_options)
    .temperature(0.7)
    .execute()
    .await?;

    println!("📝 Response:\n{}\n", result.text);
    println!(
        "💭 Reasoning tokens used: {}",
        result.usage.reasoning_tokens
    );
    println!("📊 Total tokens: {}\n", result.usage.total_tokens);

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 2: Web Search Integration");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Configure provider options with web search
    let mut xai_options = HashMap::new();
    xai_options.insert(
        "searchParameters".to_string(),
        serde_json::json!({
            "mode": "on",
            "return_citations": true,
            "max_search_results": 5
        }),
    );

    let mut provider_options = HashMap::new();
    provider_options.insert("xai".to_string(), xai_options);

    let result = GenerateText::new(
        model.clone(),
        Prompt::text("What are the latest developments in AI safety research?"),
    )
    .provider_options(provider_options)
    .temperature(0.7)
    .execute()
    .await?;

    println!("📝 Response:\n{}\n", result.text);

    // Check for sources (citations)
    let source_count = result
        .content
        .iter()
        .filter(|content| matches!(content, ai_sdk_core::output::Output::Source(_)))
        .count();

    if source_count > 0 {
        println!("📚 Found {} citation(s) in the response", source_count);
    }

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 3: Combined Options");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Combine reasoning effort with parallel function calling
    let mut xai_options = HashMap::new();
    xai_options.insert("reasoningEffort".to_string(), serde_json::json!("high"));
    xai_options.insert(
        "parallelFunctionCalling".to_string(),
        serde_json::json!(true),
    );

    let mut provider_options = HashMap::new();
    provider_options.insert("xai".to_string(), xai_options);

    let result = GenerateText::new(
        model,
        Prompt::text("Compare and contrast machine learning and deep learning."),
    )
    .provider_options(provider_options)
    .temperature(0.7)
    .max_output_tokens(500)
    .execute()
    .await?;

    println!("📝 Response:\n{}\n", result.text);
    println!("💭 Reasoning tokens: {}", result.usage.reasoning_tokens);
    println!("📊 Total tokens: {}", result.usage.total_tokens);

    println!("\n✅ Example completed successfully!");

    Ok(())
}
