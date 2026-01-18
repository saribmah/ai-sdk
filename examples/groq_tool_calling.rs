use llm_kit_core::prompt::Prompt;
use llm_kit_core::{GenerateText, ToolSet};
use llm_kit_groq::GroqClient;
use llm_kit_provider_utils::tool::{Tool, ToolExecutionOutput};
use serde_json::{Value, json};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Groq provider
    let provider = GroqClient::new().load_api_key_from_env().build();

    // Use a more capable model for tool calling
    let model = provider.chat_model("llama-3.3-70b-versatile");

    // Define a weather tool
    let weather_tool = Tool::function(json!({
        "type": "object",
        "properties": {
            "location": {
                "type": "string",
                "description": "The city and state, e.g. San Francisco, CA"
            }
        },
        "required": ["location"]
    }))
    .with_description("Get the current weather for a location")
    .with_execute(Arc::new(|input: Value, _options| {
        let location = input
            .get("location")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown");
        let result = format!("The weather in {} is sunny, 72°F", location);
        ToolExecutionOutput::Single(Box::pin(async move { Ok(json!(result)) }))
    }));

    // Create a ToolSet
    let mut tools = ToolSet::new();
    tools.insert("get_weather".to_string(), weather_tool);

    // Generate a response with tool calling
    println!("Asking about weather...\n");

    let result = GenerateText::new(
        model,
        Prompt::text("What's the weather like in San Francisco?"),
    )
    .tools(tools)
    .execute()
    .await?;

    println!("Response: {}", result.text);

    // Print tool calls if any
    if !result.tool_calls.is_empty() {
        println!("\nTool Calls:");
        for tool_call in &result.tool_calls {
            println!("  - {}: {:?}", tool_call.tool_name, tool_call.input);
        }
    }

    println!("\nUsage:");
    println!("  Input tokens: {:?}", result.usage.input_tokens);
    println!("  Output tokens: {:?}", result.usage.output_tokens);

    Ok(())
}
