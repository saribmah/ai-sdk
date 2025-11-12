/// Streaming tool calling example with Anthropic Claude.
///
/// This example demonstrates:
/// - Streaming text generation with tools
/// - Handling tool calls in streams
/// - Processing stream events (text, tool calls, tool results)
/// - Real-time output display
///
/// Run with:
/// ```bash
/// export ANTHROPIC_API_KEY="your-api-key"
/// cargo run --example streaming_tool_calling -p ai-sdk-anthropic
/// ```
use ai_sdk_anthropic::{AnthropicProviderSettings, create_anthropic};
use ai_sdk_core::prompt::Prompt;
use ai_sdk_core::stream_text::TextStreamPart;
use ai_sdk_core::{StreamText, ToolSet};
use ai_sdk_provider::language_model::LanguageModel;
use ai_sdk_provider_utils::tool::{Tool, ToolExecutionOutput};
use futures::StreamExt;
use serde_json::{Value, json};
use std::env;
use std::sync::Arc;

/// Simulates a search operation
async fn search_web(query: &str) -> Value {
    // Simulate API delay
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    json!({
        "query": query,
        "results": [
            {
                "title": format!("Result 1 for '{}'", query),
                "snippet": "This is the first search result with relevant information.",
                "url": "https://example.com/1"
            },
            {
                "title": format!("Result 2 for '{}'", query),
                "snippet": "Another relevant result with additional details.",
                "url": "https://example.com/2"
            },
            {
                "title": format!("Result 3 for '{}'", query),
                "snippet": "More information about the search query.",
                "url": "https://example.com/3"
            }
        ],
        "total_results": 3
    })
}

/// Simulates getting stock prices
async fn get_stock_price(symbol: &str) -> Value {
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

    let price = match symbol.to_uppercase().as_str() {
        "AAPL" => 178.50,
        "GOOGL" => 140.25,
        "MSFT" => 380.75,
        "TSLA" => 245.00,
        _ => 100.00,
    };

    json!({
        "symbol": symbol.to_uppercase(),
        "price": price,
        "currency": "USD",
        "change": "+2.5%",
        "volume": "25.3M",
        "market_cap": "2.8T"
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌊 Anthropic Streaming Tool Calling Example\n");

    // Get API key from environment
    let api_key = env::var("ANTHROPIC_API_KEY").map_err(
        |_| "ANTHROPIC_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create Anthropic provider
    let settings = AnthropicProviderSettings::new().with_api_key(api_key);
    let provider = create_anthropic(settings);
    let model = Arc::new(provider.language_model("claude-3-haiku-20240307".to_string()));

    println!("✓ Model loaded: {}\n", model.model_id());

    // Define tools
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Defining Tools");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Tool 1: Web Search
    let search_tool = Tool::function(json!({
        "type": "object",
        "properties": {
            "query": {
                "type": "string",
                "description": "The search query"
            }
        },
        "required": ["query"]
    }))
    .with_description("Search the web for information")
    .with_execute(Arc::new(|input: Value, _options| {
        let query = input
            .get("query")
            .and_then(|v| v.as_str())
            .unwrap_or("default query")
            .to_string();

        ToolExecutionOutput::Single(Box::pin(async move {
            println!("\n🔍 Searching for: '{}'...", query);
            let result = search_web(&query).await;
            println!("✓ Search complete\n");
            Ok(result)
        }))
    }));

    // Tool 2: Stock Price
    let stock_tool = Tool::function(json!({
        "type": "object",
        "properties": {
            "symbol": {
                "type": "string",
                "description": "Stock symbol (e.g., AAPL, GOOGL)"
            }
        },
        "required": ["symbol"]
    }))
    .with_description("Get current stock price for a given symbol")
    .with_execute(Arc::new(|input: Value, _options| {
        let symbol = input
            .get("symbol")
            .and_then(|v| v.as_str())
            .unwrap_or("UNKNOWN")
            .to_string();

        ToolExecutionOutput::Single(Box::pin(async move {
            println!("\n📈 Fetching stock price for: {}...", symbol);
            let result = get_stock_price(&symbol).await;
            println!("✓ Price fetched\n");
            Ok(result)
        }))
    }));

    // Create ToolSet
    let mut tools = ToolSet::new();
    tools.insert("search_web".to_string(), search_tool);
    tools.insert("get_stock_price".to_string(), stock_tool);

    println!("📋 Registered Tools:");
    println!("   1. search_web - Search the web");
    println!("   2. get_stock_price - Get stock prices\n");

    // Example 1: Basic streaming with tool calls
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 1: Streaming with Tool Calls");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt1 = Prompt::text("Search for 'Rust programming language' and tell me about it.");

    println!("💬 User: Search for 'Rust programming language' and tell me about it.\n");
    println!("🤖 Assistant (streaming): ");

    let result1 = StreamText::new(model.clone(), prompt1)
        .tools(tools.clone())
        .execute()
        .await?;

    let mut text_buffer = String::new();
    let mut stream1 = result1.full_stream();

    while let Some(part) = stream1.next().await {
        match part {
            TextStreamPart::TextDelta { text, .. } => {
                print!("{}", text);
                text_buffer.push_str(&text);
                std::io::Write::flush(&mut std::io::stdout()).ok();
            }
            TextStreamPart::ToolCall { tool_call } => {
                println!("\n\n🔧 Tool Call: {}", tool_call.tool_name);
                println!(
                    "   Arguments: {}",
                    serde_json::to_string_pretty(&tool_call.input).unwrap()
                );
            }
            TextStreamPart::ToolResult { tool_result } => {
                println!("\n📥 Tool Result: {}", tool_result.tool_name);
                println!(
                    "   Output: {}",
                    serde_json::to_string_pretty(&tool_result.output).unwrap()
                );
                println!();
            }
            TextStreamPart::Finish {
                finish_reason,
                total_usage,
            } => {
                println!("\n\n🏁 Stream finished: {:?}", finish_reason);
                println!(
                    "📊 Usage: {} input tokens, {} output tokens",
                    total_usage.input_tokens, total_usage.output_tokens
                );
            }
            _ => {} // Ignore other parts
        }
    }

    println!("\n");

    // Example 2: Multiple parallel tool calls
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 2: Parallel Tool Calls");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt2 = Prompt::text("Get the stock prices for AAPL, GOOGL, and MSFT.");

    println!("💬 User: Get the stock prices for AAPL, GOOGL, and MSFT.\n");
    println!("🤖 Assistant (streaming): ");

    let result2 = StreamText::new(model.clone(), prompt2)
        .tools(tools.clone())
        .execute()
        .await?;

    let mut tool_call_count = 0;
    let mut tool_result_count = 0;
    let mut stream2 = result2.full_stream();

    while let Some(part) = stream2.next().await {
        match part {
            TextStreamPart::TextDelta { text, .. } => {
                print!("{}", text);
                std::io::Write::flush(&mut std::io::stdout()).ok();
            }
            TextStreamPart::ToolCall { tool_call } => {
                tool_call_count += 1;
                println!(
                    "\n\n🔧 Tool Call #{}: {}",
                    tool_call_count, tool_call.tool_name
                );
                println!("   ID: {}", tool_call.tool_call_id);
                println!(
                    "   Arguments: {}",
                    serde_json::to_string_pretty(&tool_call.input).unwrap()
                );
            }
            TextStreamPart::ToolResult { tool_result } => {
                tool_result_count += 1;
                println!(
                    "\n📥 Tool Result #{}: {}",
                    tool_result_count, tool_result.tool_name
                );
                if let Ok(result) = serde_json::from_value::<Value>(tool_result.output.clone())
                    && let Some(symbol) = result.get("symbol")
                    && let Some(price) = result.get("price")
                {
                    println!("   {} = ${}", symbol, price);
                }
            }
            TextStreamPart::Finish {
                finish_reason,
                total_usage,
            } => {
                println!("\n\n🏁 Stream finished: {:?}", finish_reason);
                println!("   Total tool calls: {}", tool_call_count);
                println!("   Total tool results: {}", tool_result_count);
                println!(
                    "   📊 Usage: {} input tokens, {} output tokens",
                    total_usage.input_tokens, total_usage.output_tokens
                );
            }
            _ => {}
        }
    }

    println!("\n");

    // Example 3: Stream event counting
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 3: Stream Event Analysis");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt3 = Prompt::text("Search for 'AI advancements 2024' and summarize the findings.");

    println!("💬 User: Search for 'AI advancements 2024' and summarize the findings.\n");

    let result3 = StreamText::new(model, prompt3)
        .tools(tools)
        .execute()
        .await?;

    let mut event_counts = std::collections::HashMap::new();
    let mut full_text = String::new();
    let mut stream3 = result3.full_stream();

    while let Some(part) = stream3.next().await {
        let event_name = match &part {
            TextStreamPart::TextStart { .. } => "TextStart",
            TextStreamPart::TextDelta { .. } => "TextDelta",
            TextStreamPart::TextEnd { .. } => "TextEnd",
            TextStreamPart::ReasoningStart { .. } => "ReasoningStart",
            TextStreamPart::ReasoningDelta { .. } => "ReasoningDelta",
            TextStreamPart::ReasoningEnd { .. } => "ReasoningEnd",
            TextStreamPart::ToolInputStart { .. } => "ToolInputStart",
            TextStreamPart::ToolInputDelta { .. } => "ToolInputDelta",
            TextStreamPart::ToolInputEnd { .. } => "ToolInputEnd",
            TextStreamPart::ToolCall { .. } => "ToolCall",
            TextStreamPart::ToolResult { .. } => "ToolResult",
            TextStreamPart::ToolError { .. } => "ToolError",
            TextStreamPart::ToolOutputDenied { .. } => "ToolOutputDenied",
            TextStreamPart::ToolApprovalRequest { .. } => "ToolApprovalRequest",
            TextStreamPart::Source { .. } => "Source",
            TextStreamPart::File { .. } => "File",
            TextStreamPart::StartStep { .. } => "StartStep",
            TextStreamPart::FinishStep { .. } => "FinishStep",
            TextStreamPart::Finish { .. } => "Finish",
            TextStreamPart::Start => "Start",
            TextStreamPart::Abort => "Abort",
            TextStreamPart::Raw { .. } => "Raw",
            TextStreamPart::Error { .. } => "Error",
        };

        *event_counts.entry(event_name).or_insert(0) += 1;

        // Collect text
        if let TextStreamPart::TextDelta { text, .. } = &part {
            full_text.push_str(text);
            print!("{}", text);
            std::io::Write::flush(&mut std::io::stdout()).ok();
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
    println!("   ✓ Real-time streaming with tools");
    println!("   ✓ Tool call and result events in streams");
    println!("   ✓ Parallel tool execution");
    println!("   ✓ Stream event analysis");
    println!("   ✓ Progressive text output");

    Ok(())
}
