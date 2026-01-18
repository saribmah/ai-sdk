use futures_util::StreamExt;
/// Streaming tool calling example using Baseten provider with only llm-kit-provider.
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
/// export BASETEN_API_KEY="your-api-key"
/// cargo run --example stream_tool_calling -p llm-kit-baseten
/// ```
use llm_kit_baseten::BasetenClient;
use llm_kit_provider::language_model::call_options::LanguageModelCallOptions;
use llm_kit_provider::language_model::prompt::LanguageModelMessage;
use llm_kit_provider::language_model::stream_part::LanguageModelStreamPart;
use llm_kit_provider::language_model::tool::LanguageModelTool;
use llm_kit_provider::language_model::tool::function_tool::LanguageModelFunctionTool;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌊 Baseten Streaming Tool Calling Example (Provider-Only)\n");

    // Get API key from environment
    let api_key = std::env::var("BASETEN_API_KEY").map_err(
        |_| "BASETEN_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create Baseten provider using client builder
    let provider = BasetenClient::new().api_key(api_key).build();

    // Create a language model (using Model APIs with DeepSeek)
    let model = provider.chat_model(Some("deepseek-ai/DeepSeek-V3-0324"));

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
                // Other parts (Raw, etc.)
            }
        }
    }

    println!("\n");

    // Example 2: Multiple parallel tool calls
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 2: Parallel Tool Calls");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt2 = vec![LanguageModelMessage::user_text(
        "Get the stock prices for AAPL, GOOGL, and MSFT.",
    )];

    let options2 = LanguageModelCallOptions::new(prompt2).with_tools(tools.clone());

    println!("💬 User: Get the stock prices for AAPL, GOOGL, and MSFT.\n");
    println!("🤖 Assistant (streaming):\n");

    let result2 = model.do_stream(options2).await?;
    let mut stream2 = result2.stream;

    let mut tool_call_count = 0;

    while let Some(part) = stream2.next().await {
        match part {
            LanguageModelStreamPart::TextDelta(text_delta) => {
                print!("{}", text_delta.delta);
                std::io::Write::flush(&mut std::io::stdout()).ok();
            }
            LanguageModelStreamPart::ToolInputStart(tool_start) => {
                tool_call_count += 1;
                println!(
                    "\n🔧 Tool Call #{}: {}",
                    tool_call_count, tool_start.tool_name
                );
                println!("   ID: {}", tool_start.id);
            }
            LanguageModelStreamPart::ToolInputDelta(tool_delta) => {
                print!("{}", tool_delta.delta);
                std::io::Write::flush(&mut std::io::stdout()).ok();
            }
            LanguageModelStreamPart::ToolInputEnd(_) => {
                println!();
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
            _ => {}
        }
    }

    println!("\n📊 Total tool calls: {}", tool_call_count);
    println!();

    // Example 3: Stream part analysis
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 3: Stream Part Analysis");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt3 = vec![LanguageModelMessage::user_text(
        "Search for 'artificial intelligence' on the web.",
    )];

    let options3 = LanguageModelCallOptions::new(prompt3).with_tools(tools);

    println!("💬 User: Search for 'artificial intelligence' on the web.\n");
    println!("📦 Stream Part Analysis:\n");

    let result3 = model.do_stream(options3).await?;
    let mut stream3 = result3.stream;

    let mut part_counts = StreamPartCounts::default();

    while let Some(part) = stream3.next().await {
        match part {
            LanguageModelStreamPart::StreamStart(_) => part_counts.stream_start += 1,
            LanguageModelStreamPart::TextStart(_) => part_counts.text_start += 1,
            LanguageModelStreamPart::TextDelta(delta) => {
                part_counts.text_delta += 1;
                part_counts.text_chars += delta.delta.len();
            }
            LanguageModelStreamPart::TextEnd(_) => part_counts.text_end += 1,
            LanguageModelStreamPart::ToolInputStart(_) => part_counts.tool_input_start += 1,
            LanguageModelStreamPart::ToolInputDelta(delta) => {
                part_counts.tool_input_delta += 1;
                part_counts.tool_input_chars += delta.delta.len();
            }
            LanguageModelStreamPart::ToolInputEnd(_) => part_counts.tool_input_end += 1,
            LanguageModelStreamPart::ReasoningStart(_) => part_counts.reasoning_start += 1,
            LanguageModelStreamPart::ReasoningDelta(delta) => {
                part_counts.reasoning_delta += 1;
                part_counts.reasoning_chars += delta.delta.len();
            }
            LanguageModelStreamPart::ReasoningEnd(_) => part_counts.reasoning_end += 1,
            LanguageModelStreamPart::Finish(finish) => {
                part_counts.finish += 1;
                part_counts.finish_reason = Some(format!("{:?}", finish.finish_reason));
                part_counts.input_tokens = finish.usage.input_tokens;
                part_counts.output_tokens = finish.usage.output_tokens;
            }
            LanguageModelStreamPart::Error(_) => part_counts.error += 1,
            LanguageModelStreamPart::Raw(_) => part_counts.raw += 1,
            _ => {} // Other variants (ToolCall, etc.)
        }
    }

    println!("📊 Stream Part Statistics:");
    println!("   Stream Start: {}", part_counts.stream_start);
    println!("   Text Start: {}", part_counts.text_start);
    println!(
        "   Text Delta: {} ({} chars)",
        part_counts.text_delta, part_counts.text_chars
    );
    println!("   Text End: {}", part_counts.text_end);
    println!("   Tool Input Start: {}", part_counts.tool_input_start);
    println!(
        "   Tool Input Delta: {} ({} chars)",
        part_counts.tool_input_delta, part_counts.tool_input_chars
    );
    println!("   Tool Input End: {}", part_counts.tool_input_end);
    println!("   Reasoning Start: {}", part_counts.reasoning_start);
    println!(
        "   Reasoning Delta: {} ({} chars)",
        part_counts.reasoning_delta, part_counts.reasoning_chars
    );
    println!("   Reasoning End: {}", part_counts.reasoning_end);
    println!("   Finish: {}", part_counts.finish);
    println!("   Error: {}", part_counts.error);
    println!("   Raw: {}", part_counts.raw);
    println!(
        "\n   Finish Reason: {}",
        part_counts.finish_reason.unwrap_or("N/A".to_string())
    );
    println!("   Input Tokens: {}", part_counts.input_tokens);
    println!("   Output Tokens: {}", part_counts.output_tokens);

    println!("\n✅ All examples completed successfully!");
    println!("\n💡 Key Features Demonstrated:");
    println!("   ✓ Using do_stream() with tools (provider-only)");
    println!("   ✓ Streaming tool calls");
    println!("   ✓ Processing different stream part types");
    println!("   ✓ Handling parallel tool calls");
    println!("   ✓ Analyzing stream structure");
    println!("\n⚠️  Note: This example shows tool call streaming only. For full tool execution,");
    println!("   use llm-kit-core's StreamText with tools.");

    Ok(())
}

/// Helper struct to count different stream part types
#[derive(Default)]
struct StreamPartCounts {
    stream_start: usize,
    text_start: usize,
    text_delta: usize,
    text_chars: usize,
    text_end: usize,
    tool_input_start: usize,
    tool_input_delta: usize,
    tool_input_chars: usize,
    tool_input_end: usize,
    reasoning_start: usize,
    reasoning_delta: usize,
    reasoning_chars: usize,
    reasoning_end: usize,
    finish: usize,
    error: usize,
    raw: usize,
    finish_reason: Option<String>,
    input_tokens: u64,
    output_tokens: u64,
}
