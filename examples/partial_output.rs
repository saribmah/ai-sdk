use futures_util::StreamExt;
use llm_kit_core::StreamText;
/// Partial output parsing example demonstrating incremental structured data extraction.
///
/// This example shows how to:
/// - Request JSON output from a language model
/// - Stream partial JSON objects as they're being generated
/// - Parse structured data incrementally during streaming
/// - Handle complete JSON objects when streaming finishes
///
/// Run with:
/// ```bash
/// export OPENAI_API_KEY="your-api-key"
/// cargo run --example partial_output
/// ```
use llm_kit_core::prompt::Prompt;
use llm_kit_openai_compatible::OpenAICompatibleClient;
use std::env;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct Person {
    name: String,
    age: u32,
    occupation: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct Recipe {
    name: String,
    ingredients: Vec<String>,
    steps: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 LLM Kit - Partial Output Parsing Example\n");

    // Get API key from environment
    let api_key = env::var("OPENAI_API_KEY").map_err(
        |_| "OPENAI_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create OpenAI provider using the client builder
    let provider = OpenAICompatibleClient::new()
        .base_url("https://openrouter.ai/api/v1")
        .api_key(api_key)
        .build();

    println!("✓ Provider created: {}", provider.name());
    println!("✓ Base URL: {}\n", provider.base_url());

    // Get a language model
    let model = provider.chat_model("gpt-4o-mini");
    println!("✓ Model loaded: {}", model.model_id());
    println!("✓ Provider: {}\n", model.provider());

    // Example 1: Partial JSON parsing for a person object
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📋 Example 1: Streaming Person Object");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt = Prompt::text(
        "Generate a JSON object describing a person with name, age, and occupation. \
         Return ONLY valid JSON, no explanations. \
         Example: {\"name\": \"Alice\", \"age\": 30, \"occupation\": \"Engineer\"}",
    );

    let result = StreamText::new(model.clone(), prompt)
        .temperature(0.7)
        .max_output_tokens(200)
        .execute()
        .await?;

    println!("📝 Streaming partial JSON (will parse to Person):\n");

    // Stream partial outputs as JSON values
    let mut partial_stream = result.partial_output_stream::<serde_json::Value>();
    let mut last_value: Option<serde_json::Value> = None;

    while let Some(value) = partial_stream.next().await {
        // Only print if different from last
        if last_value.as_ref() != Some(&value) {
            // Try to parse as Person
            if let Ok(person) = serde_json::from_value::<Person>(value.clone()) {
                println!("  → Parsed Person: {:?}", person);
            } else {
                println!("  → Partial JSON: {}", serde_json::to_string(&value)?);
            }
            last_value = Some(value);
        }
    }

    println!("\n");

    // Example 2: Streaming a recipe with arrays
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📋 Example 2: Streaming Recipe with Arrays");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt = Prompt::text(
        "Generate a JSON recipe for chocolate chip cookies with name, ingredients array, and steps array. \
         Return ONLY valid JSON. Example: \
         {\"name\": \"Chocolate Chip Cookies\", \
         \"ingredients\": [\"flour\", \"sugar\", \"chocolate chips\"], \
         \"steps\": [\"Mix ingredients\", \"Bake at 350F\"]}",
    );

    let result = StreamText::new(model.clone(), prompt)
        .temperature(0.7)
        .max_output_tokens(200)
        .execute()
        .await?;

    println!("📝 Streaming partial JSON (will parse to Recipe):\n");

    let mut partial_stream = result.partial_output_stream::<serde_json::Value>();
    let mut last_value: Option<serde_json::Value> = None;

    while let Some(value) = partial_stream.next().await {
        if last_value.as_ref() != Some(&value) {
            // Try to parse as Recipe
            if let Ok(recipe) = serde_json::from_value::<Recipe>(value.clone()) {
                println!("  → Name: {}", recipe.name);
                println!(
                    "    Ingredients ({} so far): {}",
                    recipe.ingredients.len(),
                    recipe.ingredients.join(", ")
                );
                println!("    Steps ({} so far):", recipe.steps.len());
                for (i, step) in recipe.steps.iter().enumerate() {
                    println!("      {}. {}", i + 1, step);
                }
            } else {
                println!("  → Partial JSON: {}", serde_json::to_string(&value)?);
            }
            println!();
            last_value = Some(value);
        }
    }

    // Example 3: Using serde_json::Value for untyped JSON
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📋 Example 3: Streaming Generic JSON Value");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt = Prompt::text(
        "Generate a JSON object describing a car with make, model, year, and features array. \
         Return ONLY valid JSON.",
    );

    let result = StreamText::new(model, prompt)
        .temperature(0.7)
        .max_output_tokens(200)
        .execute()
        .await?;

    println!("📝 Streaming partial JSON values:\n");

    let mut partial_stream = result.partial_output_stream::<serde_json::Value>();
    let mut update_count = 0;

    while let Some(value) = partial_stream.next().await {
        update_count += 1;
        println!(
            "  Update #{}: {}",
            update_count,
            serde_json::to_string_pretty(&value)?
        );
        println!();
    }

    println!("✅ All examples completed successfully!");
    println!("   Total partial updates in last example: {}", update_count);

    Ok(())
}
