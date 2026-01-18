use llm_kit_cerebras::{CerebrasClient, chat::models};
use llm_kit_core::GenerateText;
use llm_kit_core::output::Output;
use llm_kit_core::prompt::Prompt;
use std::env;

/// Reasoning model example with Cerebras.
///
/// This example demonstrates using Cerebras thinking models that support
/// reasoning/thinking content. These models can output their internal
/// reasoning process before providing the final answer.
///
/// Run with:
/// ```bash
/// export CEREBRAS_API_KEY="your-api-key"
/// cargo run --example cerebras_reasoning
/// ```
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 LLM Kit - Cerebras Reasoning Model Example\n");

    // Load environment variables from .env file if present
    dotenvy::dotenv().ok();

    // Get API key from environment
    let api_key = env::var("CEREBRAS_API_KEY").map_err(
        |_| "CEREBRAS_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create Cerebras provider
    let provider = CerebrasClient::new().api_key(api_key).build();

    // Use a reasoning/thinking model
    let model = provider.chat_model(models::QWEN_3_235B_THINKING);

    println!("✓ Model loaded: {}", model.model_id());
    println!("✓ Provider: {}\n", model.provider());
    println!("💭 This model supports reasoning output - it can show its thinking process!\n");

    // Create a prompt that requires reasoning
    let prompt = Prompt::text(
        "Solve this logic puzzle: Three friends - Alice, Bob, and Carol - each have a different pet: a cat, a dog, and a fish. \
         Alice doesn't have a dog. Bob is allergic to cats. Carol loves swimming with her pet. \
         Who has which pet?",
    );

    println!("📤 Sending prompt with logic puzzle...\n");

    // Generate response
    println!("⏳ Generating response with reasoning...\n");
    let result = GenerateText::new(model, prompt)
        .temperature(0.7)
        .max_output_tokens(1000)
        .execute()
        .await?;

    println!("✅ Response received!\n");

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Response Content");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Process the response - reasoning models may output reasoning content
    for (i, output) in result.content.iter().enumerate() {
        match output {
            Output::Reasoning(reasoning) => {
                println!("💭 Reasoning Process [{}]:", i + 1);
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                println!("{}", reasoning.text);
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
            }
            Output::Text(text_output) => {
                println!("📝 Final Answer [{}]:", i + 1);
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                println!("{}", text_output.text);
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
            }
            Output::ToolCall(_) => {
                println!("🔧 Tool Call [{}] (unexpected in this example)\n", i + 1);
            }
            Output::ToolResult(_) => {
                println!("✅ Tool Result [{}] (unexpected in this example)\n", i + 1);
            }
            Output::Source(_) => {
                println!("📚 Source [{}]\n", i + 1);
            }
            _ => {}
        }
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Metadata");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("📊 Finish Reason: {:?}", result.finish_reason);
    println!("📊 Input Tokens: {}", result.usage.input_tokens);
    println!("📊 Output Tokens: {}", result.usage.output_tokens);
    println!("📊 Total Tokens: {}", result.usage.total_tokens);

    if result.usage.reasoning_tokens > 0 {
        println!(
            "💭 Reasoning Tokens: {} (tokens used for thinking)",
            result.usage.reasoning_tokens
        );
    }

    println!("\n✅ Example completed successfully!");
    println!("\n💡 Note: Reasoning models show their step-by-step thinking process,");
    println!("   which helps you understand how they arrive at their conclusions.");

    Ok(())
}
