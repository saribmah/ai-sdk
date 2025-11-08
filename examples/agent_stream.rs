use ai_sdk_core::tool::definition::Tool;
/// Agent stream example demonstrating agent-based tool calling (streaming).
///
/// This example shows how to:
/// - Create an agent with tools and instructions
/// - Use the agent.stream() method for streaming responses
/// - Process streamed text chunks in real-time
/// - Handle multi-step tool execution with streaming
///
/// Run with:
/// ```bash
/// export OPENAI_API_KEY="your-api-key"
/// cargo run --example agent_stream
/// ```
use ai_sdk_core::{Agent, AgentCallParameters, AgentInterface, AgentSettings};
use ai_sdk_core::{ToolSet, step_count_is};
use ai_sdk_openai_compatible::OpenAICompatibleClient;
use futures_util::StreamExt;
use serde_json::{Value, json};
use std::env;
use std::sync::Arc;

/// Simulates getting stock price information
fn get_stock_price(symbol: &str) -> Value {
    println!("\n    📈 Executing: get_stock_price(symbol=\"{}\")", symbol);

    // Mock stock data
    let stock_data = match symbol.to_uppercase().as_str() {
        "AAPL" => json!({
            "symbol": "AAPL",
            "company": "Apple Inc.",
            "price": 182.52,
            "currency": "USD",
            "change": 2.34,
            "change_percent": 1.30,
            "volume": 52_341_200,
            "market_cap": "2.89T"
        }),
        "GOOGL" => json!({
            "symbol": "GOOGL",
            "company": "Alphabet Inc.",
            "price": 142.87,
            "currency": "USD",
            "change": -0.92,
            "change_percent": -0.64,
            "volume": 23_456_100,
            "market_cap": "1.78T"
        }),
        "MSFT" => json!({
            "symbol": "MSFT",
            "company": "Microsoft Corporation",
            "price": 378.91,
            "currency": "USD",
            "change": 5.67,
            "change_percent": 1.52,
            "volume": 28_934_500,
            "market_cap": "2.82T"
        }),
        "TSLA" => json!({
            "symbol": "TSLA",
            "company": "Tesla, Inc.",
            "price": 248.50,
            "currency": "USD",
            "change": -3.21,
            "change_percent": -1.27,
            "volume": 89_234_600,
            "market_cap": "789B"
        }),
        _ => json!({
            "symbol": symbol,
            "company": format!("{} Corp.", symbol),
            "price": 125.00,
            "currency": "USD",
            "change": 0.50,
            "change_percent": 0.40,
            "volume": 10_000_000,
            "market_cap": "100B"
        }),
    };

    println!("    ✓ Stock data retrieved for {}", symbol);
    stock_data
}

/// Simulates getting company news
fn get_company_news(company: &str) -> Value {
    println!(
        "\n    📰 Executing: get_company_news(company=\"{}\")",
        company
    );

    // Mock news data
    let news = match company.to_uppercase().as_str() {
        "APPLE" | "AAPL" => json!({
            "company": "Apple Inc.",
            "articles": [
                {
                    "title": "Apple Announces New AI Features for iPhone",
                    "summary": "Apple reveals groundbreaking AI capabilities in latest iOS update",
                    "sentiment": "positive",
                    "published": "2 hours ago"
                },
                {
                    "title": "Apple Vision Pro Sees Strong Sales",
                    "summary": "Demand exceeds expectations for spatial computing device",
                    "sentiment": "positive",
                    "published": "5 hours ago"
                }
            ]
        }),
        "GOOGLE" | "GOOGL" | "ALPHABET" => json!({
            "company": "Alphabet Inc.",
            "articles": [
                {
                    "title": "Google Cloud Revenue Surges",
                    "summary": "Cloud division shows significant growth in Q4 earnings",
                    "sentiment": "positive",
                    "published": "1 hour ago"
                }
            ]
        }),
        _ => json!({
            "company": company,
            "articles": [
                {
                    "title": format!("{} Reports Quarterly Results", company),
                    "summary": "Company announces financial performance",
                    "sentiment": "neutral",
                    "published": "3 hours ago"
                }
            ]
        }),
    };

    println!("    ✓ News retrieved for {}", company);
    news
}

/// Simulates analyzing market sentiment
fn analyze_sentiment(text: &str) -> Value {
    println!(
        "\n    🧠 Executing: analyze_sentiment(text_length={})",
        text.len()
    );

    // Simple mock sentiment analysis
    let sentiment_score = if text.to_lowercase().contains("growth")
        || text.to_lowercase().contains("increase")
        || text.to_lowercase().contains("positive")
    {
        0.75
    } else if text.to_lowercase().contains("decline")
        || text.to_lowercase().contains("decrease")
        || text.to_lowercase().contains("negative")
    {
        -0.65
    } else {
        0.15
    };

    let sentiment_label = if sentiment_score > 0.3 {
        "bullish"
    } else if sentiment_score < -0.3 {
        "bearish"
    } else {
        "neutral"
    };

    let result = json!({
        "sentiment": sentiment_label,
        "score": sentiment_score,
        "confidence": 0.82,
        "analysis": format!("Market sentiment appears {} based on the provided text", sentiment_label)
    });

    println!("    ✓ Sentiment analysis complete: {}", sentiment_label);
    result
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 AI SDK Rust - Agent Stream Example (Streaming)\n");

    // Get API key from environment
    let api_key = env::var("OPENAI_API_KEY").map_err(
        |_| "OPENAI_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create OpenAI provider
    let provider = OpenAICompatibleClient::new()
        .base_url("https://openrouter.ai/api/v1")
        .api_key(api_key)
        .build();

    let model = provider.chat_model("openai/gpt-4o-mini");
    println!("✓ Model loaded: {}\n", model.model_id());

    // Define tools
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Defining Tools");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    use ai_sdk_core::tool::definition::ToolExecutionOutput;

    // Stock price tool
    let stock_tool = Tool::function(json!({
        "type": "object",
        "properties": {
            "symbol": {
                "type": "string",
                "description": "The stock ticker symbol (e.g., AAPL, GOOGL, MSFT)"
            }
        },
        "required": ["symbol"]
    }))
    .with_description("Get the current stock price and information for a company")
    .with_execute(Arc::new(|input: Value, _options| {
        let symbol = input
            .get("symbol")
            .and_then(|v| v.as_str())
            .unwrap_or("UNKNOWN");

        let stock_data = get_stock_price(symbol);
        ToolExecutionOutput::Single(Box::pin(async move { Ok(stock_data) }))
    }));

    // Company news tool
    let news_tool = Tool::function(json!({
        "type": "object",
        "properties": {
            "company": {
                "type": "string",
                "description": "The company name or ticker symbol"
            }
        },
        "required": ["company"]
    }))
    .with_description("Get recent news articles about a company")
    .with_execute(Arc::new(|input: Value, _options| {
        let company = input
            .get("company")
            .and_then(|v| v.as_str())
            .unwrap_or("UNKNOWN");

        let news_data = get_company_news(company);
        ToolExecutionOutput::Single(Box::pin(async move { Ok(news_data) }))
    }));

    // Sentiment analysis tool
    let sentiment_tool = Tool::function(json!({
        "type": "object",
        "properties": {
            "text": {
                "type": "string",
                "description": "The text to analyze for market sentiment"
            }
        },
        "required": ["text"]
    }))
    .with_description("Analyze the sentiment of market-related text")
    .with_execute(Arc::new(|input: Value, _options| {
        let text = input.get("text").and_then(|v| v.as_str()).unwrap_or("");

        let sentiment_data = analyze_sentiment(text);
        ToolExecutionOutput::Single(Box::pin(async move { Ok(sentiment_data) }))
    }));

    // Create a ToolSet
    let mut tools = ToolSet::new();
    tools.insert("get_stock_price".to_string(), stock_tool);
    tools.insert("get_company_news".to_string(), news_tool);
    tools.insert("analyze_sentiment".to_string(), sentiment_tool);
    println!("📋 Available Tools:");
    println!("   1. get_stock_price - Get current stock price information");
    println!("   2. get_company_news - Get recent news about a company");
    println!("   3. analyze_sentiment - Analyze market sentiment from text\n");

    // Create agent settings
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Creating Agent");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let settings = AgentSettings::new(model)
        .with_id("financial-analyst-agent")
        .with_instructions(
            "You are a knowledgeable financial analyst assistant. \
             When users ask about stocks or companies, use the available tools to get current information. \
             Provide clear, concise analysis with relevant data. \
             Always mention the current price and recent news when discussing stocks. \
             Be professional and data-driven in your responses."
        )
        .with_temperature(0.5)
        .with_max_output_tokens(1500)
        .with_stop_when(vec![Arc::new(step_count_is(10))])
        .with_tools(tools);

    let agent = Agent::new(settings);

    println!("✓ Agent created with ID: {:?}", agent.settings().id);
    println!(
        "✓ Agent has {} tools available",
        agent.tools().map_or(0, |t| t.len())
    );
    println!("✓ Temperature: {:?}", agent.settings().temperature);
    println!(
        "✓ Max output tokens: {:?}\n",
        agent.settings().max_output_tokens
    );

    // Example 1: Single stock query with streaming
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 1: Single Stock Analysis (Streaming)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let params1 = AgentCallParameters::from_text(
        "Tell me about Apple stock (AAPL). What's the current price and what's the latest news?",
    );

    println!(
        "📝 User: Tell me about Apple stock (AAPL). What's the current price and what's the latest news?\n"
    );
    println!("🤖 Agent (streaming): ");

    let result1 = agent.stream(params1)?.execute().await?;
    let mut text_stream = result1.text_stream();

    // Stream the response in real-time
    while let Some(chunk) = text_stream.next().await {
        print!("{}", chunk);
        use std::io::Write;
        std::io::stdout().flush()?;
    }

    println!("\n");

    // Get stats from the result
    println!("📊 Stats:");
    println!("   • Steps taken: {}", result1.steps().await?.len());
    println!("   • Finish reason: {:?}", result1.finish_reason().await?);
    println!("   • Total tokens: {}", result1.usage().await?.total_tokens);

    // Example 2: Multi-stock comparison with streaming
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 2: Multi-Stock Comparison (Streaming)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let params2 = AgentCallParameters::from_text(
        "Compare Apple (AAPL) and Microsoft (MSFT) stocks. Which one is performing better today?",
    );

    println!(
        "📝 User: Compare Apple (AAPL) and Microsoft (MSFT) stocks. Which one is performing better today?\n"
    );
    println!("🤖 Agent (streaming): ");

    let result2 = agent.stream(params2)?.execute().await?;
    let mut text_stream2 = result2.text_stream();

    // Stream the response
    while let Some(chunk) = text_stream2.next().await {
        print!("{}", chunk);
        use std::io::Write;
        std::io::stdout().flush()?;
    }

    println!("\n");

    println!("📊 Stats:");
    println!("   • Steps taken: {}", result2.steps().await?.len());
    println!("   • Finish reason: {:?}", result2.finish_reason().await?);
    println!("   • Total tokens: {}", result2.usage().await?.total_tokens);

    // Example 3: Complex analysis with sentiment
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 3: Complex Analysis with Sentiment (Streaming)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let params3 = AgentCallParameters::from_text(
        "Give me a comprehensive analysis of Tesla (TSLA). Include the stock price, recent news, \
         and analyze the overall market sentiment around the company.",
    );

    println!(
        "📝 User: Give me a comprehensive analysis of Tesla (TSLA). Include the stock price, recent news, and analyze the overall market sentiment around the company.\n"
    );
    println!("🤖 Agent (streaming): ");

    let result3 = agent.stream(params3)?.execute().await?;
    let mut text_stream3 = result3.text_stream();

    // Stream the response
    while let Some(chunk) = text_stream3.next().await {
        print!("{}", chunk);
        use std::io::Write;
        std::io::stdout().flush()?;
    }

    println!("\n");

    println!("📊 Stats:");
    println!("   • Steps taken: {}", result3.steps().await?.len());
    println!("   • Finish reason: {:?}", result3.finish_reason().await?);
    println!("   • Total tokens: {}", result3.usage().await?.total_tokens);

    // Example 4: Using text stream parts for more control
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 4: Stream with Full Control (TextStreamPart)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let params4 = AgentCallParameters::from_text("What's the latest on Google (GOOGL)?");

    println!("📝 User: What's the latest on Google (GOOGL)?\n");
    println!("🤖 Agent (detailed streaming): ");

    let result4 = agent.stream(params4)?.execute().await?;
    let mut full_stream = result4.full_stream();

    let mut tool_call_count = 0;

    // Process stream parts
    while let Some(part) = full_stream.next().await {
        use ai_sdk_core::TextStreamPart;
        match part {
            TextStreamPart::TextDelta { text, .. } => {
                print!("{}", text);
                use std::io::Write;
                std::io::stdout().flush()?;
            }
            TextStreamPart::ToolInputStart { tool_name, .. } => {
                tool_call_count += 1;
                println!("\n   [Tool call #{}: {}]", tool_call_count, tool_name);
            }
            TextStreamPart::ToolInputEnd { .. } => {
                println!("   [Tool input completed]");
            }
            TextStreamPart::Finish { finish_reason, .. } => {
                println!("\n   [Stream finished: {:?}]", finish_reason);
            }
            _ => {}
        }
    }

    println!("\n");
    println!("📊 Stats:");
    println!("   • Tool calls made: {}", tool_call_count);

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ Agent Stream Example Complete!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    Ok(())
}
