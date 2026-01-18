use futures_util::StreamExt;
/// Streaming tool calling example using Hugging Face provider with only llm-kit-provider.
///
/// This example demonstrates:
/// - Using LanguageModel::do_stream() directly with tools (no llm-kit-core)
/// - Processing tool calls in streams
/// - Handling stream parts with tool information
/// - NOTE: This example shows tool definitions and stream parts but does not execute tools
///   (tool execution requires llm-kit-core). It validates that the provider correctly
///   streams tool calls.
///
/// Run with:
/// ```bash
/// export HUGGINGFACE_API_KEY="your-api-key"
/// cargo run --example stream_tool_calling -p llm-kit-huggingface
/// ```
use llm_kit_huggingface::{HuggingFaceClient, LLAMA_3_1_8B_INSTRUCT};
use llm_kit_provider::language_model::call_options::LanguageModelCallOptions;
use llm_kit_provider::language_model::prompt::LanguageModelMessage;
use llm_kit_provider::language_model::stream_part::LanguageModelStreamPart;
use llm_kit_provider::language_model::tool::LanguageModelTool;
use llm_kit_provider::language_model::tool::function_tool::LanguageModelFunctionTool;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌊 Hugging Face Streaming Tool Calling Example (Provider-Only)\n");

    // Get API key from environment
    let api_key = std::env::var("HUGGINGFACE_API_KEY").map_err(
        |_| "HUGGINGFACE_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create Hugging Face provider using client builder
    let provider = HuggingFaceClient::new().api_key(api_key).build();

    // Create a language model
    let model = provider.responses(LLAMA_3_1_8B_INSTRUCT);

    println!("✓ Model loaded: {}\n", model.model_id());

    // Define tools using llm-kit-provider types
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Defining Tools");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Tool 1: Web Search
    let search_tool = LanguageModelFunctionTool::new(
        "search_web",
        json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "The search query"
                }
            },
            "required": ["query"]
        }),
    )
    .with_description("Search the web for information");

    // Tool 2: Get Weather
    let weather_tool = LanguageModelFunctionTool::new(
        "get_weather",
        json!({
            "type": "object",
            "properties": {
                "city": {
                    "type": "string",
                    "description": "The city to get weather for"
                }
            },
            "required": ["city"]
        }),
    )
    .with_description("Get current weather for a city");

    let tools = vec![
        LanguageModelTool::Function(search_tool),
        LanguageModelTool::Function(weather_tool),
    ];

    println!("📋 Registered Tools:");
    println!("   1. search_web - Search the web");
    println!("   2. get_weather - Get weather information\n");

    // Example 1: Basic streaming with tool calls
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 1: Streaming with Tool Calls");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt1 = vec![LanguageModelMessage::user_text(
        "Search for 'Rust programming language' and tell me about it.",
    )];

    let options1 = LanguageModelCallOptions::new(prompt1).with_tools(tools.clone());

    println!("💬 User: Search for 'Rust programming language' and tell me about it.\n");
    println!("🤖 Assistant (streaming):\n");

    let result1 = model.do_stream(options1).await?;
    let mut stream1 = result1.stream;

    let mut text_buffer = String::new();
    let mut tool_input_buffer = String::new();
    let mut current_tool_name = String::new();

    while let Some(part) = stream1.next().await {
        match part {
            LanguageModelStreamPart::TextStart(_) => {
                // New text content starting
            }
            LanguageModelStreamPart::TextDelta(text_delta) => {
                print!("{}", text_delta.delta);
                text_buffer.push_str(&text_delta.delta);
                use std::io::Write;
                std::io::stdout().flush()?;
            }
            LanguageModelStreamPart::TextEnd(_) => {
                // Text content completed
            }
            LanguageModelStreamPart::ToolInputStart(tool_start) => {
                println!("\n\n🔧 Tool Call Started: {}", tool_start.tool_name);
                println!("   ID: {}", tool_start.id);
                current_tool_name = tool_start.tool_name.clone();
                tool_input_buffer.clear();
            }
            LanguageModelStreamPart::ToolInputDelta(tool_delta) => {
                tool_input_buffer.push_str(&tool_delta.delta);
            }
            LanguageModelStreamPart::ToolInputEnd(_) => {
                if !tool_input_buffer.is_empty() {
                    println!("   Arguments: {}", tool_input_buffer);
                }
                current_tool_name.clear();
                tool_input_buffer.clear();
            }
            LanguageModelStreamPart::StreamStart(_) => {
                // Stream started
            }
            LanguageModelStreamPart::Finish(finish) => {
                println!("\n\n🏁 Stream finished: {:?}", finish.finish_reason);
                println!(
                    "📊 Usage: {} input tokens, {} output tokens",
                    finish.usage.input_tokens, finish.usage.output_tokens
                );
            }
            LanguageModelStreamPart::Error(error) => {
                println!("\n❌ Stream error: {:?}", error);
            }
            _ => {
                // Other parts
            }
        }
    }

    println!("\n");

    // Example 2: Multiple parallel tool calls
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 2: Parallel Tool Calls");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt2 = vec![LanguageModelMessage::user_text(
        "Get the weather for Tokyo, London, and New York.",
    )];

    let options2 = LanguageModelCallOptions::new(prompt2).with_tools(tools.clone());

    println!("💬 User: Get the weather for Tokyo, London, and New York.\n");
    println!("🤖 Assistant (streaming):\n");

    let result2 = model.do_stream(options2).await?;
    let mut stream2 = result2.stream;

    let mut tool_call_count = 0;

    while let Some(part) = stream2.next().await {
        match part {
            LanguageModelStreamPart::TextDelta(text_delta) => {
                print!("{}", text_delta.delta);
                use std::io::Write;
                std::io::stdout().flush()?;
            }
            LanguageModelStreamPart::ToolInputStart(tool_start) => {
                tool_call_count += 1;
                println!(
                    "\n\n🔧 Tool Call #{}: {}",
                    tool_call_count, tool_start.tool_name
                );
                println!("   ID: {}", tool_start.id);
            }
            LanguageModelStreamPart::ToolInputDelta(tool_delta) => {
                print!("{}", tool_delta.delta);
                use std::io::Write;
                std::io::stdout().flush()?;
            }
            LanguageModelStreamPart::ToolInputEnd(_) => {
                println!();
            }
            LanguageModelStreamPart::Finish(finish) => {
                println!("\n🏁 Stream finished: {:?}", finish.finish_reason);
                println!("   Total tool calls: {}", tool_call_count);
                println!(
                    "   📊 Usage: {} input tokens, {} output tokens",
                    finish.usage.input_tokens, finish.usage.output_tokens
                );
            }
            _ => {}
        }
    }

    println!("\n");

    // Example 3: Stream event analysis
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 3: Stream Event Analysis");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt3 = vec![LanguageModelMessage::user_text(
        "Search for 'AI advancements 2024' and summarize.",
    )];

    let options3 = LanguageModelCallOptions::new(prompt3).with_tools(tools);

    println!("💬 User: Search for 'AI advancements 2024' and summarize.\n");

    let result3 = model.do_stream(options3).await?;
    let mut stream3 = result3.stream;

    let mut event_counts = std::collections::HashMap::new();
    let mut full_text = String::new();

    while let Some(part) = stream3.next().await {
        let event_name = match &part {
            LanguageModelStreamPart::StreamStart(_) => "StreamStart",
            LanguageModelStreamPart::ResponseMetadata(_) => "ResponseMetadata",
            LanguageModelStreamPart::TextStart(_) => "TextStart",
            LanguageModelStreamPart::TextDelta(_) => "TextDelta",
            LanguageModelStreamPart::TextEnd(_) => "TextEnd",
            LanguageModelStreamPart::ReasoningStart(_) => "ReasoningStart",
            LanguageModelStreamPart::ReasoningDelta(_) => "ReasoningDelta",
            LanguageModelStreamPart::ReasoningEnd(_) => "ReasoningEnd",
            LanguageModelStreamPart::ToolInputStart(_) => "ToolInputStart",
            LanguageModelStreamPart::ToolInputDelta(_) => "ToolInputDelta",
            LanguageModelStreamPart::ToolInputEnd(_) => "ToolInputEnd",
            LanguageModelStreamPart::ToolCall(_) => "ToolCall",
            LanguageModelStreamPart::ToolResult(_) => "ToolResult",
            LanguageModelStreamPart::File(_) => "File",
            LanguageModelStreamPart::Source(_) => "Source",
            LanguageModelStreamPart::Finish(_) => "Finish",
            LanguageModelStreamPart::Error(_) => "Error",
            LanguageModelStreamPart::Raw(_) => "Raw",
        };

        *event_counts.entry(event_name).or_insert(0) += 1;

        // Collect text
        if let LanguageModelStreamPart::TextDelta(text_delta) = &part {
            full_text.push_str(&text_delta.delta);
            print!("{}", text_delta.delta);
            use std::io::Write;
            std::io::stdout().flush()?;
        }

        // Show tool calls
        if let LanguageModelStreamPart::ToolInputStart(tool_start) = &part {
            println!(
                "\n\n🔧 Tool: {} (ID: {})",
                tool_start.tool_name, tool_start.id
            );
        }
    }

    println!("\n\n📊 Stream Event Statistics:");
    let mut events: Vec<_> = event_counts.iter().collect();
    events.sort_by_key(|(name, _)| *name);
    for (event_name, count) in events {
        println!("   {}: {}", event_name, count);
    }

    println!("\n📝 Total text characters: {}", full_text.len());

    println!("\n✅ Example completed successfully!");
    println!("\n💡 Key Features Demonstrated:");
    println!("   ✓ Using do_stream() with tools (provider-only)");
    println!("   ✓ Processing ToolCallInputStart/Delta/End stream parts");
    println!("   ✓ Handling parallel tool calls in streams");
    println!("   ✓ Stream event analysis and counting");
    println!("   ✓ Real-time tool call detection");
    println!("\n⚠️  Note: This example shows tool definitions and stream parts only.");
    println!("   For full tool execution, use llm-kit-core's StreamText with tools.");

    Ok(())
}
