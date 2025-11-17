/// Streaming tool calling example using Together AI provider with only ai-sdk-provider.
///
/// This example demonstrates:
/// - Using LanguageModel::do_stream() directly with tools (no ai-sdk-core)
/// - Processing tool calls in streams
/// - Handling stream parts with tool information
/// - NOTE: This example shows tool definitions and stream parts but does not execute tools
///   (tool execution requires ai-sdk-core). It validates that the provider correctly
///   streams tool calls.
///
/// Run with:
/// ```bash
/// export TOGETHER_AI_API_KEY="your-api-key"
/// cargo run --example stream_tool_calling -p ai-sdk-togetherai
/// ```
use ai_sdk_provider::language_model::call_options::LanguageModelCallOptions;
use ai_sdk_provider::language_model::prompt::LanguageModelMessage;
use ai_sdk_provider::language_model::stream_part::LanguageModelStreamPart;
use ai_sdk_provider::language_model::tool::LanguageModelTool;
use ai_sdk_provider::language_model::tool::function_tool::LanguageModelFunctionTool;
use ai_sdk_togetherai::TogetherAIClient;
use futures_util::StreamExt;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌊 Together AI Streaming Tool Calling Example (Provider-Only)\n");

    // Get API key from environment
    let api_key = std::env::var("TOGETHER_AI_API_KEY").map_err(
        |_| "TOGETHER_AI_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create Together AI provider using client builder
    let provider = TogetherAIClient::new().api_key(api_key).build();

    // Create a language model
    let model = provider.chat_model("meta-llama/Llama-3.3-70B-Instruct-Turbo");

    println!("✓ Model loaded: {}\n", model.model_id());

    // Define tools using ai-sdk-provider types
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

    // Tool 2: Stock Price
    let stock_tool = LanguageModelFunctionTool::new(
        "get_stock_price",
        json!({
            "type": "object",
            "properties": {
                "symbol": {
                    "type": "string",
                    "description": "Stock symbol (e.g., AAPL, GOOGL)"
                }
            },
            "required": ["symbol"]
        }),
    )
    .with_description("Get current stock price for a given symbol");

    let tools = vec![
        LanguageModelTool::Function(search_tool),
        LanguageModelTool::Function(stock_tool),
    ];

    println!("📋 Registered Tools:");
    println!("   1. search_web - Search the web");
    println!("   2. get_stock_price - Get stock prices\n");

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
                std::io::Write::flush(&mut std::io::stdout()).ok();
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
            LanguageModelStreamPart::ReasoningStart(_) => {
                // Reasoning content starting (extended thinking)
            }
            LanguageModelStreamPart::ReasoningDelta(_) => {
                // Reasoning content (not displayed in this example)
            }
            LanguageModelStreamPart::ReasoningEnd(_) => {
                // Reasoning completed
            }
            LanguageModelStreamPart::StreamStart(_) => {
                // Stream started
            }
            LanguageModelStreamPart::Finish(finish) => {
                println!(
                    "\n\n📊 Usage: {} input tokens, {} output tokens",
                    finish.usage.input_tokens, finish.usage.output_tokens
                );
                println!("🏁 Finish reason: {:?}", finish.finish_reason);
            }
            _ => {}
        }
    }

    println!("\n");

    // Example 2: Multiple tool calls in stream
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 2: Multiple Tool Calls");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt2 = vec![LanguageModelMessage::user_text(
        "Get the stock price for AAPL and GOOGL.",
    )];

    let options2 = LanguageModelCallOptions::new(prompt2)
        .with_tools(tools.clone())
        .with_max_output_tokens(1024);

    println!("💬 User: Get the stock price for AAPL and GOOGL.\n");
    println!("🤖 Assistant (streaming):\n");

    let result2 = model.do_stream(options2).await?;
    let mut stream2 = result2.stream;

    let mut tool_calls_count = 0;
    let mut current_tool_buffer = String::new();

    while let Some(part) = stream2.next().await {
        match part {
            LanguageModelStreamPart::TextDelta(text_delta) => {
                print!("{}", text_delta.delta);
                std::io::Write::flush(&mut std::io::stdout()).ok();
            }
            LanguageModelStreamPart::ToolInputStart(tool_start) => {
                tool_calls_count += 1;
                println!(
                    "\n\n🔧 Tool Call #{}: {}",
                    tool_calls_count, tool_start.tool_name
                );
                println!("   ID: {}", tool_start.id);
                current_tool_buffer.clear();
            }
            LanguageModelStreamPart::ToolInputDelta(tool_delta) => {
                current_tool_buffer.push_str(&tool_delta.delta);
            }
            LanguageModelStreamPart::ToolInputEnd(_) => {
                if !current_tool_buffer.is_empty() {
                    println!("   Arguments: {}", current_tool_buffer);
                }
            }
            LanguageModelStreamPart::Finish(finish) => {
                println!(
                    "\n\n📊 Usage: {} input tokens, {} output tokens",
                    finish.usage.input_tokens, finish.usage.output_tokens
                );
                println!("🏁 Finish reason: {:?}", finish.finish_reason);
            }
            _ => {}
        }
    }

    println!("\n");

    // Example 3: Stream part counting
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 3: Stream Part Analysis");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt3 = vec![LanguageModelMessage::user_text(
        "What's the weather like? Use the search tool to find out.",
    )];

    let options3 = LanguageModelCallOptions::new(prompt3).with_tools(tools);

    println!("💬 User: What's the weather like? Use the search tool.\n");
    println!("🤖 Assistant (streaming):\n");

    let result3 = model.do_stream(options3).await?;
    let mut stream3 = result3.stream;

    let mut stats = StreamStats::default();

    while let Some(part) = stream3.next().await {
        match &part {
            LanguageModelStreamPart::TextStart(_) => stats.text_starts += 1,
            LanguageModelStreamPart::TextDelta(delta) => {
                print!("{}", delta.delta);
                std::io::Write::flush(&mut std::io::stdout()).ok();
                stats.text_deltas += 1;
            }
            LanguageModelStreamPart::TextEnd(_) => stats.text_ends += 1,
            LanguageModelStreamPart::ToolInputStart(tool_start) => {
                println!("\n\n🔧 Tool: {}", tool_start.tool_name);
                stats.tool_starts += 1;
            }
            LanguageModelStreamPart::ToolInputDelta(_) => stats.tool_deltas += 1,
            LanguageModelStreamPart::ToolInputEnd(_) => stats.tool_ends += 1,
            LanguageModelStreamPart::Finish(finish) => {
                println!(
                    "\n\n📊 Usage: {} input tokens, {} output tokens",
                    finish.usage.input_tokens, finish.usage.output_tokens
                );
            }
            _ => {}
        }
    }

    println!("\n");
    println!("📈 Stream Statistics:");
    println!("   Text starts: {}", stats.text_starts);
    println!("   Text deltas: {}", stats.text_deltas);
    println!("   Text ends: {}", stats.text_ends);
    println!("   Tool starts: {}", stats.tool_starts);
    println!("   Tool deltas: {}", stats.tool_deltas);
    println!("   Tool ends: {}", stats.tool_ends);

    println!("\n✅ All examples completed successfully!");
    println!("\n💡 Key Features Demonstrated:");
    println!("   ✓ Using do_stream() with tools (provider-only)");
    println!("   ✓ Processing tool calls in streams");
    println!("   ✓ Handling ToolInputStart/Delta/End events");
    println!("   ✓ Multiple tool calls in one request");
    println!("   ✓ Stream part analysis");
    println!("\n⚠️  Note: This example shows tool streaming only. For full tool execution,");
    println!("   use ai-sdk-core's StreamText with tools.");

    Ok(())
}

#[derive(Default)]
struct StreamStats {
    text_starts: usize,
    text_deltas: usize,
    text_ends: usize,
    tool_starts: usize,
    tool_deltas: usize,
    tool_ends: usize,
}
