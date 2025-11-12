//! Basic chat completion example using OpenAI provider
//!
//! Run with: cargo run --example basic_chat

use ai_sdk_openai::openai;
use ai_sdk_provider::language_model::LanguageModel;
use ai_sdk_provider::language_model::call_options::LanguageModelCallOptions;
use ai_sdk_provider::language_model::prompt::message::LanguageModelMessage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider (reads OPENAI_API_KEY from environment)
    let provider = openai();

    // Create a chat model
    let model = provider.chat("gpt-4o-mini");

    println!("Using model: {}", model.model_id());
    println!("Provider: {}", model.provider());

    // Create a simple prompt
    let prompt = vec![
        LanguageModelMessage::system("You are a helpful assistant."),
        LanguageModelMessage::user_text("What is the capital of France?"),
    ];

    // Create call options
    let options = LanguageModelCallOptions {
        prompt,
        max_output_tokens: Some(100),
        temperature: Some(0.7),
        top_p: None,
        top_k: None,
        frequency_penalty: None,
        presence_penalty: None,
        stop_sequences: None,
        seed: None,
        response_format: None,
        tools: None,
        tool_choice: None,
        headers: None,
        provider_options: None,
        abort_signal: None,
        include_raw_chunks: None,
    };

    // Generate response
    println!("\nGenerating response...");
    let response = model.do_generate(options).await?;

    // Print response
    println!("\nResponse:");
    for content in &response.content {
        if let ai_sdk_provider::language_model::content::LanguageModelContent::Text(text) = content
        {
            println!("{}", text.text);
        }
    }

    // Print usage
    println!("\nUsage:");
    println!("  Input tokens: {}", response.usage.input_tokens);
    println!("  Output tokens: {}", response.usage.output_tokens);
    println!("  Total tokens: {}", response.usage.total_tokens);

    // Print finish reason
    println!("\nFinish reason: {:?}", response.finish_reason);

    // Print warnings if any
    if !response.warnings.is_empty() {
        println!("\nWarnings:");
        for warning in &response.warnings {
            println!("  {:?}", warning);
        }
    }

    Ok(())
}
