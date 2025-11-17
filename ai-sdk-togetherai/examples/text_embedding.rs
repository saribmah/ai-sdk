/// Text embedding example using Together AI provider with only ai-sdk-provider.
///
/// This example demonstrates:
/// - Using EmbeddingModel::do_embed() directly (no ai-sdk-core)
/// - Generating text embeddings for semantic search
/// - Working with EmbeddingModelCallOptions from ai-sdk-provider
///
/// Run with:
/// ```bash
/// export TOGETHER_AI_API_KEY="your-api-key"
/// cargo run --example text_embedding -p ai-sdk-togetherai
/// ```
use ai_sdk_provider::embedding_model::call_options::EmbeddingModelCallOptions;
use ai_sdk_togetherai::TogetherAIClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Together AI Text Embedding Example (Provider-Only)\n");

    // Get API key from environment
    let api_key = std::env::var("TOGETHER_AI_API_KEY").map_err(
        |_| "TOGETHER_AI_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create Together AI provider using client builder
    let provider = TogetherAIClient::new().api_key(api_key).build();

    // Create an embedding model
    let model = provider.text_embedding_model("WhereIsAI/UAE-Large-V1");

    println!("✓ Model loaded: {}\n", model.model_id());

    // Example 1: Single text embedding
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 1: Single Text Embedding");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let text1 = "Rust is a systems programming language focused on safety and performance.";
    let options1 = EmbeddingModelCallOptions::new(vec![text1.to_string()]);

    println!("📝 Text: \"{}\"\n", text1);

    let result1 = model.do_embed(options1).await?;

    println!("✅ Embedding generated successfully!");
    println!("   Embedding dimensions: {}", result1.embeddings[0].len());
    println!("   First 5 values: {:?}", &result1.embeddings[0][..5]);
    if let Some(usage) = &result1.usage {
        println!("   Tokens used: {}", usage.tokens);
    }
    println!();

    // Example 2: Batch text embeddings
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 2: Batch Text Embeddings");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let texts = vec![
        "The quick brown fox jumps over the lazy dog.".to_string(),
        "Machine learning is a subset of artificial intelligence.".to_string(),
        "Rust provides memory safety without garbage collection.".to_string(),
        "Together AI offers powerful open-source language models.".to_string(),
    ];

    let options2 = EmbeddingModelCallOptions::new(texts.clone());

    println!("📝 Generating embeddings for {} texts...\n", texts.len());

    let result2 = model.do_embed(options2).await?;

    println!("✅ Batch embeddings generated successfully!");
    println!("   Number of embeddings: {}", result2.embeddings.len());
    println!("   Embedding dimensions: {}", result2.embeddings[0].len());
    if let Some(usage) = &result2.usage {
        println!("   Total tokens used: {}", usage.tokens);
    }
    println!();

    // Example 3: Computing cosine similarity
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 3: Semantic Similarity");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let query = "Programming languages with memory safety".to_string();
    let documents = vec![
        "Rust ensures memory safety at compile time.".to_string(),
        "Python is a dynamically typed language.".to_string(),
        "JavaScript runs in web browsers.".to_string(),
        "C++ provides manual memory management.".to_string(),
    ];

    // Get embedding for query
    let query_options = EmbeddingModelCallOptions::new(vec![query.clone()]);
    let query_result = model.do_embed(query_options).await?;
    let query_embedding = &query_result.embeddings[0];

    // Get embeddings for documents
    let doc_options = EmbeddingModelCallOptions::new(documents.clone());
    let doc_result = model.do_embed(doc_options).await?;

    println!("📝 Query: \"{}\"\n", query);
    println!("📚 Documents:");
    for (i, doc) in documents.iter().enumerate() {
        println!("   {}. {}", i + 1, doc);
    }
    println!();

    // Compute cosine similarities
    println!("🔍 Similarity Scores:");
    let mut similarities: Vec<(usize, f64)> = doc_result
        .embeddings
        .iter()
        .enumerate()
        .map(|(i, doc_embedding)| {
            let similarity = cosine_similarity(query_embedding, doc_embedding);
            (i, similarity)
        })
        .collect();

    // Sort by similarity (descending)
    similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for (rank, (doc_idx, similarity)) in similarities.iter().enumerate() {
        println!(
            "   {}. [{:.4}] {}",
            rank + 1,
            similarity,
            documents[*doc_idx]
        );
    }

    if let Some(usage) = &query_result.usage {
        print!("\n   Query tokens: {}", usage.tokens);
    }
    if let Some(usage) = &doc_result.usage {
        println!(" | Document tokens: {}", usage.tokens);
    }

    println!("\n✅ All examples completed successfully!");
    println!("\n💡 Key Features Demonstrated:");
    println!("   ✓ Using do_embed() directly (provider-only)");
    println!("   ✓ Single text embedding");
    println!("   ✓ Batch text embeddings");
    println!("   ✓ Semantic similarity computation");
    println!("   ✓ Usage tracking");

    Ok(())
}

/// Compute cosine similarity between two vectors
fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    let dot_product: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let magnitude_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let magnitude_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();

    if magnitude_a == 0.0 || magnitude_b == 0.0 {
        0.0
    } else {
        dot_product / (magnitude_a * magnitude_b)
    }
}
