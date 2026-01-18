/// Reranking example using Together AI provider with only llm-kit-provider.
///
/// This example demonstrates:
/// - Using RerankingModel::do_rerank() directly (no llm-kit-core)
/// - Reranking documents for improved search results
/// - Working with RerankingModelCallOptions from llm-kit-provider
///
/// Run with:
/// ```bash
/// export TOGETHER_AI_API_KEY="your-api-key"
/// cargo run --example reranking -p llm-kit-togetherai
/// ```
use llm_kit_provider::reranking_model::call_options::{
    RerankingDocuments, RerankingModelCallOptions,
};
use llm_kit_togetherai::TogetherAIClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Together AI Reranking Example (Provider-Only)\n");

    // Get API key from environment
    let api_key = std::env::var("TOGETHER_AI_API_KEY").map_err(
        |_| "TOGETHER_AI_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create Together AI provider using client builder
    let provider = TogetherAIClient::new().api_key(api_key).build();

    // Create a reranking model
    let model = provider.reranking_model("Salesforce/Llama-Rank-v1");

    println!("✓ Model loaded: {}\n", model.model_id());

    // Example 1: Basic reranking with text documents
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 1: Basic Document Reranking");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let query1 = "What is the capital of France?".to_string();
    let documents1 = RerankingDocuments::from_text(vec![
        "The capital of France is Paris.".to_string(),
        "Python is a programming language.".to_string(),
        "Paris is known for the Eiffel Tower.".to_string(),
        "JavaScript runs in web browsers.".to_string(),
        "France is a country in Western Europe.".to_string(),
    ]);

    let options1 = RerankingModelCallOptions::new(documents1, query1.clone());

    println!("📝 Query: \"{}\"\n", query1);
    println!("📚 Original Documents:");
    for (i, doc) in [
        "The capital of France is Paris.",
        "Python is a programming language.",
        "Paris is known for the Eiffel Tower.",
        "JavaScript runs in web browsers.",
        "France is a country in Western Europe.",
    ]
    .iter()
    .enumerate()
    {
        println!("   {}. {}", i, doc);
    }
    println!();

    let result1 = model.do_rerank(options1).await?;

    println!("🎯 Reranked Results:");
    for (rank, ranked_doc) in result1.ranking.iter().enumerate() {
        let original_doc = [
            "The capital of France is Paris.",
            "Python is a programming language.",
            "Paris is known for the Eiffel Tower.",
            "JavaScript runs in web browsers.",
            "France is a country in Western Europe.",
        ][ranked_doc.index];
        println!(
            "   {}. [Score: {:.4}] (Index: {}) {}",
            rank + 1,
            ranked_doc.relevance_score,
            ranked_doc.index,
            original_doc
        );
    }
    println!();

    // Example 2: Reranking with top_n limit
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 2: Reranking with Top-N");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let query2 = "Rust programming language".to_string();
    let documents2 = RerankingDocuments::from_text(vec![
        "Rust is a systems programming language focused on safety.".to_string(),
        "Python is great for data science and machine learning.".to_string(),
        "Rust provides memory safety without garbage collection.".to_string(),
        "Go is designed for building scalable network services.".to_string(),
        "Rust has zero-cost abstractions and no runtime overhead.".to_string(),
        "Java is a popular object-oriented programming language.".to_string(),
    ]);

    let options2 = RerankingModelCallOptions::new(documents2, query2.clone()).with_top_n(3);

    println!("📝 Query: \"{}\"\n", query2);
    println!("📚 6 documents provided, requesting top 3\n");

    let result2 = model.do_rerank(options2).await?;

    println!("🎯 Top 3 Reranked Results:");
    for (rank, ranked_doc) in result2.ranking.iter().enumerate() {
        let original_doc = [
            "Rust is a systems programming language focused on safety.",
            "Python is great for data science and machine learning.",
            "Rust provides memory safety without garbage collection.",
            "Go is designed for building scalable network services.",
            "Rust has zero-cost abstractions and no runtime overhead.",
            "Java is a popular object-oriented programming language.",
        ][ranked_doc.index];
        println!(
            "   {}. [Score: {:.4}] {}",
            rank + 1,
            ranked_doc.relevance_score,
            original_doc
        );
    }
    println!();

    // Example 3: Semantic search use case
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 3: Semantic Search Scenario");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let query3 = "How do I deploy a web application?".to_string();
    let search_results = vec![
        "Docker containers make it easy to package and deploy applications.".to_string(),
        "REST APIs are a common way to build web services.".to_string(),
        "Kubernetes orchestrates containerized applications at scale.".to_string(),
        "GraphQL is an alternative to REST for APIs.".to_string(),
        "CI/CD pipelines automate the deployment process.".to_string(),
        "Version control is important for software development.".to_string(),
    ];

    let documents3 = RerankingDocuments::from_text(search_results.clone());
    let options3 = RerankingModelCallOptions::new(documents3, query3.clone()).with_top_n(3);

    println!("📝 Search Query: \"{}\"\n", query3);
    println!("📚 Initial Search Results (unranked):");
    for (i, doc) in search_results.iter().enumerate() {
        println!("   {}. {}", i + 1, doc);
    }
    println!();

    let result3 = model.do_rerank(options3).await?;

    println!("🎯 Reranked Top 3 (most relevant first):");
    for (rank, ranked_doc) in result3.ranking.iter().enumerate() {
        println!(
            "   {}. [Score: {:.4}] {}",
            rank + 1,
            ranked_doc.relevance_score,
            search_results[ranked_doc.index]
        );
    }

    // Show metadata if available
    if let Some(response) = &result3.response {
        println!("\n📊 Response Metadata:");
        if let Some(model_id) = &response.model_id {
            println!("   Model: {}", model_id);
        }
        if let Some(id) = &response.id {
            println!("   Request ID: {}", id);
        }
    }

    println!("\n✅ All examples completed successfully!");
    println!("\n💡 Key Features Demonstrated:");
    println!("   ✓ Using do_rerank() directly (provider-only)");
    println!("   ✓ Basic document reranking");
    println!("   ✓ Top-N filtering");
    println!("   ✓ Semantic search improvement");
    println!("   ✓ Relevance score inspection");
    println!("\n💡 Use Case: Reranking improves search results by scoring documents");
    println!("   based on semantic relevance to the query, not just keyword matching.");

    Ok(())
}
