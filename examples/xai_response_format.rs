use llm_kit_core::GenerateText;
///
/// This example demonstrates response format (JSON mode) with the xAI provider.
///
/// This example shows how to:
/// - Use simple JSON mode (no schema)
/// - Use structured outputs with JSON schema
/// - Validate the JSON response
///
/// Run with:
/// ```bash
/// export XAI_API_KEY="your-xai-api-key"
/// cargo run --example xai_response_format
/// ```
use llm_kit_core::prompt::Prompt;
use llm_kit_provider::language_model::call_options::LanguageModelResponseFormat;
use llm_kit_xai::XaiClient;
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment
    let api_key = env::var("XAI_API_KEY").expect("XAI_API_KEY environment variable must be set");

    // Create xAI provider
    let provider = XaiClient::new().api_key(&api_key).build();

    // Example 1: Simple JSON mode (no schema)
    println!("=== Example 1: Simple JSON Mode ===\n");
    simple_json_mode(&provider).await?;

    println!("\n{}\n", "=".repeat(80));

    // Example 2: Structured outputs with JSON schema
    println!("=== Example 2: Structured Outputs (JSON Schema) ===\n");
    structured_output(&provider).await?;

    println!("\n{}\n", "=".repeat(80));

    // Example 3: Complex structured output
    println!("=== Example 3: Complex Structured Output ===\n");
    complex_structured_output(&provider).await?;

    Ok(())
}

/// Example 1: Simple JSON mode without schema
async fn simple_json_mode(
    provider: &llm_kit_xai::XaiProvider,
) -> Result<(), Box<dyn std::error::Error>> {
    let model = provider.chat_model("grok-2-1212");

    println!("Requesting weather information in JSON format...\n");

    let result = GenerateText::new(
        model,
        Prompt::text(
            "Get the weather for San Francisco. Return a JSON object with city, temperature, and conditions."
        ),
    )
    .with_response_format(LanguageModelResponseFormat::Json {
        schema: None,
        name: None,
        description: None,
    })
    .temperature(0.7)
    .execute()
    .await?;

    println!("Response:");
    println!("{}", result.text);

    // Validate JSON
    if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&result.text) {
        println!("\n✓ Valid JSON received");
        println!(
            "Parsed JSON:\n{}",
            serde_json::to_string_pretty(&json_value)?
        );
    } else {
        println!("\n✗ Invalid JSON received");
    }

    Ok(())
}

/// Example 2: Structured output with JSON schema
async fn structured_output(
    provider: &llm_kit_xai::XaiProvider,
) -> Result<(), Box<dyn std::error::Error>> {
    let model = provider.chat_model("grok-2-1212");

    // Define a strict JSON schema
    let schema = json!({
        "type": "object",
        "properties": {
            "city": {
                "type": "string",
                "description": "The name of the city"
            },
            "temperature": {
                "type": "number",
                "description": "Temperature in Celsius"
            },
            "conditions": {
                "type": "string",
                "enum": ["Sunny", "Cloudy", "Rainy", "Snowy"],
                "description": "Weather conditions"
            },
            "humidity": {
                "type": "integer",
                "description": "Humidity percentage (0-100)"
            }
        },
        "required": ["city", "temperature", "conditions", "humidity"],
        "additionalProperties": false
    });

    println!("Requesting weather with strict JSON schema...\n");
    println!("Schema:");
    println!("{}\n", serde_json::to_string_pretty(&schema)?);

    let result = GenerateText::new(model, Prompt::text("Get the weather for Tokyo."))
        .with_response_format(LanguageModelResponseFormat::Json {
            schema: Some(schema.clone()),
            name: Some("WeatherData".to_string()),
            description: Some("Weather information for a city".to_string()),
        })
        .temperature(0.7)
        .execute()
        .await?;

    println!("Response:");
    println!("{}", result.text);

    // Validate against schema
    if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&result.text) {
        println!("\n✓ Valid JSON received");
        println!(
            "Parsed JSON:\n{}",
            serde_json::to_string_pretty(&json_value)?
        );

        // Check required fields
        if json_value.get("city").is_some()
            && json_value.get("temperature").is_some()
            && json_value.get("conditions").is_some()
            && json_value.get("humidity").is_some()
        {
            println!("\n✓ All required fields present");
        } else {
            println!("\n✗ Missing required fields");
        }
    } else {
        println!("\n✗ Invalid JSON received");
    }

    Ok(())
}

/// Example 3: Complex structured output (user profile)
async fn complex_structured_output(
    provider: &llm_kit_xai::XaiProvider,
) -> Result<(), Box<dyn std::error::Error>> {
    let model = provider.chat_model("grok-2-1212");

    // Define a complex schema for a user profile
    let schema = json!({
        "type": "object",
        "properties": {
            "user": {
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string"
                    },
                    "age": {
                        "type": "integer",
                        "minimum": 0,
                        "maximum": 150
                    },
                    "email": {
                        "type": "string",
                        "format": "email"
                    },
                    "interests": {
                        "type": "array",
                        "items": {
                            "type": "string"
                        },
                        "minItems": 1
                    },
                    "location": {
                        "type": "object",
                        "properties": {
                            "city": {
                                "type": "string"
                            },
                            "country": {
                                "type": "string"
                            }
                        },
                        "required": ["city", "country"]
                    }
                },
                "required": ["name", "age", "email", "interests", "location"]
            }
        },
        "required": ["user"],
        "additionalProperties": false
    });

    println!("Requesting user profile with complex schema...\n");

    let result = GenerateText::new(
        model,
        Prompt::text(
            "Generate a fictional user profile for a software engineer named Alice who lives in London."
        ),
    )
    .with_response_format(LanguageModelResponseFormat::Json {
        schema: Some(schema.clone()),
        name: Some("UserProfile".to_string()),
        description: Some("A detailed user profile".to_string()),
    })
    .temperature(0.8)
    .execute()
    .await?;

    println!("Response:");
    println!("{}", result.text);

    // Validate and pretty print
    if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&result.text) {
        println!("\n✓ Valid JSON received");
        println!(
            "Parsed JSON:\n{}",
            serde_json::to_string_pretty(&json_value)?
        );
    } else {
        println!("\n✗ Invalid JSON received");
    }

    Ok(())
}
