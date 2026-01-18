use futures::StreamExt;
/// Streaming tool calling example using xAI provider with only llm-kit-provider.
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
/// export XAI_API_KEY="your-api-key"
/// cargo run --example stream_tool_calling -p llm-kit-xai
/// ```
use llm_kit_provider::language_model::call_options::LanguageModelCallOptions;
use llm_kit_provider::language_model::prompt::LanguageModelMessage;
use llm_kit_provider::language_model::stream_part::LanguageModelStreamPart;
use llm_kit_provider::language_model::tool::LanguageModelTool;
use llm_kit_provider::language_model::tool::function_tool::LanguageModelFunctionTool;
use llm_kit_xai::XaiClient;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌊 xAI Streaming Tool Calling Example (Provider-Only)\n");

    // Get API key from environment
    let api_key = std::env::var("XAI_API_KEY").map_err(
        |_| "XAI_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create xAI provider using client builder
    let provider = XaiClient::new().api_key(api_key).build();

    // Create a language model
    let model = provider.chat_model("grok-beta");

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
            LanguageModelStreamPart::Error(error) => {
                println!("\n❌ Error: {}", error.error);
            }
            _ => {
                // Other stream parts (ToolCall, ToolResult, File, Source, Raw, Image)
            }
        }
    }

    println!("📝 Total text characters: {}\n", text_buffer.len());

    // Example 2: Stock price query with streaming
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 2: Stock Price Query");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt2 = vec![LanguageModelMessage::user_text(
        "What's the current price of Apple stock (AAPL)?",
    )];

    let options2 = LanguageModelCallOptions::new(prompt2).with_tools(tools.clone());

    println!("💬 User: What's the current price of Apple stock (AAPL)?\n");
    println!("🤖 Assistant (streaming):\n");

    let result2 = model.do_stream(options2).await?;
    let mut stream2 = result2.stream;

    let mut tool_calls_detected = 0;
    let mut text_chunks = 0;

    while let Some(part) = stream2.next().await {
        match part {
            LanguageModelStreamPart::TextDelta(text_delta) => {
                print!("{}", text_delta.delta);
                text_chunks += 1;
                std::io::Write::flush(&mut std::io::stdout()).ok();
            }
            LanguageModelStreamPart::ToolInputStart(tool_start) => {
                println!("\n\n🔧 Tool Call: {}", tool_start.tool_name);
                println!("   ID: {}", tool_start.id);
                tool_calls_detected += 1;
                tool_input_buffer.clear();
            }
            LanguageModelStreamPart::ToolInputDelta(tool_delta) => {
                tool_input_buffer.push_str(&tool_delta.delta);
            }
            LanguageModelStreamPart::ToolInputEnd(_) => {
                if !tool_input_buffer.is_empty() {
                    println!("   Arguments: {}", tool_input_buffer);
                }
                tool_input_buffer.clear();
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

    println!("\n📈 Stream Statistics:");
    println!("   Text chunks: {}", text_chunks);
    println!("   Tool calls detected: {}\n", tool_calls_detected);

    // Example 3: Multiple tools in one query
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 3: Multiple Tools");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt3 = vec![LanguageModelMessage::user_text(
        "Search for Tesla news and get the stock price for TSLA.",
    )];

    let options3 = LanguageModelCallOptions::new(prompt3).with_tools(tools.clone());

    println!("💬 User: Search for Tesla news and get the stock price for TSLA.\n");
    println!("🤖 Assistant (streaming):\n");

    let result3 = model.do_stream(options3).await?;
    let mut stream3 = result3.stream;

    let mut tool_call_count = 0;
    let mut tool_names = Vec::new();

    while let Some(part) = stream3.next().await {
        match part {
            LanguageModelStreamPart::TextDelta(text_delta) => {
                print!("{}", text_delta.delta);
                std::io::Write::flush(&mut std::io::stdout()).ok();
            }
            LanguageModelStreamPart::ToolInputStart(tool_start) => {
                println!(
                    "\n\n🔧 Tool Call #{}: {}",
                    tool_call_count + 1,
                    tool_start.tool_name
                );
                println!("   ID: {}", tool_start.id);
                tool_names.push(tool_start.tool_name.clone());
                tool_call_count += 1;
                tool_input_buffer.clear();
            }
            LanguageModelStreamPart::ToolInputDelta(tool_delta) => {
                tool_input_buffer.push_str(&tool_delta.delta);
            }
            LanguageModelStreamPart::ToolInputEnd(_) => {
                if !tool_input_buffer.is_empty() {
                    println!("   Arguments: {}", tool_input_buffer);
                }
                tool_input_buffer.clear();
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

    println!("\n📈 Summary:");
    println!("   Total tool calls: {}", tool_call_count);
    if !tool_names.is_empty() {
        println!("   Tools used: {:?}", tool_names);
    }

    println!("\n✅ All examples completed successfully!");
    println!("\n💡 Key Features Demonstrated:");
    println!("   ✓ Using do_stream() with tools (provider-only)");
    println!("   ✓ Real-time tool call streaming");
    println!("   ✓ Processing stream parts with tool information");
    println!("   ✓ Handling multiple tool calls");
    println!("   ✓ Tool input delta accumulation");
    println!(
        "\n📌 Note: This example demonstrates streaming tool calls but does not execute them."
    );
    println!("   Tool execution requires llm-kit-core's StreamText builder.");

    Ok(())
}
