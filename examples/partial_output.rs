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
use ai_sdk_core::prompt::{Prompt, call_settings::CallSettings};
use ai_sdk_core::stream_text;
use ai_sdk_openai_compatible::{OpenAICompatibleProviderSettings, create_openai_compatible};
use futures_util::StreamExt;
use std::env;
use std::sync::Arc;

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
    println!("🤖 AI SDK Rust - Partial Output Parsing Example\n");

    // Get API key from environment
    let api_key = env::var("OPENAI_API_KEY").map_err(
        |_| "OPENAI_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create OpenAI provider
    let provider = create_openai_compatible(
        OpenAICompatibleProviderSettings::new("https://openrouter.ai/api/v1", "openai")
            .with_api_key(api_key),
    );

    println!("✓ Provider created: {}", provider.name());
    println!("✓ Base URL: {}\n", provider.base_url());

    // Get a language model
    let model: Arc<dyn ai_sdk_provider::language_model::LanguageModel> =
        Arc::from(provider.chat_model("gpt-4o-mini"));
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
    let settings = CallSettings::default()
        .with_temperature(0.7)
        .with_max_output_tokens(200);

    let result = stream_text(
        Arc::clone(&model),
        prompt,
        settings.clone(),
        None,  // tools
        None,  // tool_choice
        None,  // stop_when
        None,  // provider_options
        None,  // prepare_step
        false, // include_raw_chunks
        None,  // transforms
        None,  // on_chunk
        None,  // on_error
        None,  // on_step_finish
        None,  // on_finish
    )
    .await?;

    println!("📝 Streaming partial JSON (will parse to Person):\n");

    // Stream partial outputs as JSON values
    let mut partial_stream = result.partial_output_stream();
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

    let result = stream_text::stream_text(
        Arc::clone(&model),
        prompt,
        settings.clone(),
        None,
        None,
        None,
        None,
        None,
        false,
        None,
        None,
        None,
        None,
        None,
    )
    .await?;

    println!("📝 Streaming partial JSON (will parse to Recipe):\n");

    let mut partial_stream = result.partial_output_stream();
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

    let result = stream_text::stream_text(
        Arc::clone(&model),
        prompt,
        settings,
        None,
        None,
        None,
        None,
        None,
        false,
        None,
        None,
        None,
        None,
        None,
    )
    .await?;

    println!("📝 Streaming partial JSON values:\n");

    let mut partial_stream = result.partial_output_stream();
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
