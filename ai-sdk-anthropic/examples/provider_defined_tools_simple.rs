/// Simplified provider-defined tools example with Anthropic Claude.
///
/// This example demonstrates using Anthropic's provider-defined bash tool.
///
/// Run with:
/// ```bash
/// export ANTHROPIC_API_KEY="your-api-key"
/// cargo run --example provider_defined_tools_simple -p ai-sdk-anthropic
/// ```
use ai_sdk_anthropic::{AnthropicProviderSettings, anthropic_tools, create_anthropic};
use ai_sdk_core::prompt::Prompt;
use ai_sdk_core::{GenerateText, ToolSet};
use ai_sdk_provider::language_model::LanguageModel;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🛠️  Anthropic Provider-Defined Tools Example\n");

    // Get API key from environment
    let api_key = env::var("ANTHROPIC_API_KEY").map_err(
        |_| "ANTHROPIC_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create Anthropic provider
    let settings = AnthropicProviderSettings::new().with_api_key(api_key);
    let provider = create_anthropic(settings);
    // Note: Provider-defined tools require Claude 3 Opus or Claude 3.5 Sonnet
    let model = Arc::new(provider.language_model("claude-3-opus-20240229".to_string()));

    println!("✓ Model loaded: {}\n", model.model_id());

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example: Using bash_20241022 Provider Tool");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Create the bash tool (provider-defined)
    let bash_tool = anthropic_tools::bash_20241022(None);

    let mut tools = ToolSet::new();
    tools.insert("bash".to_string(), bash_tool);

    println!("📋 Tool registered: bash_20241022 (Anthropic provider-defined tool)");
    println!("   This tool allows the model to execute bash commands.\n");

    println!("💬 User: List the files in the current directory using ls.\n");

    let prompt = Prompt::text(
        "List the files in the current directory using the bash tool with 'ls -la' command.",
    );

    let result = GenerateText::new(model, prompt)
        .tools(tools)
        .max_output_tokens(2048)
        .execute()
        .await?;

    println!("🤖 Assistant: {}\n", result.text);

    println!(
        "📊 Usage: {} input tokens, {} output tokens",
        result.usage.input_tokens, result.usage.output_tokens
    );
    println!("🏁 Finish reason: {:?}", result.finish_reason);
    println!("\n📝 Steps executed: {}", result.steps.len());

    for (i, step) in result.steps.iter().enumerate() {
        println!("\nStep {}:", i + 1);
        println!("  - Finish reason: {:?}", step.finish_reason);

        let tool_calls = step.tool_calls();
        if !tool_calls.is_empty() {
            println!("  - Tool calls: {}", tool_calls.len());
            for tc in tool_calls {
                println!("    → {}", tc.tool_name);
            }
        }

        let tool_results = step.tool_results();
        if !tool_results.is_empty() {
            println!("  - Tool results: {}", tool_results.len());
        }
    }

    println!("\n✅ Example completed successfully!");
    println!("\n💡 Key Feature Demonstrated:");
    println!("   ✓ Provider-defined tools (bash_20241022)");
    println!("   ✓ Automatic beta flag handling");
    println!("   ✓ Tool execution and results");

    Ok(())
}
