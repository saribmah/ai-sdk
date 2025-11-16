use ai_sdk_baseten::BasetenClient;
use ai_sdk_core::EmbedMany;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Embeddings require a custom model URL
    // Replace {model-id} with your actual embedding model ID from Baseten
    let model_id = std::env::var("BASETEN_EMBEDDING_MODEL_ID")
        .expect("BASETEN_EMBEDDING_MODEL_ID environment variable not set");

    let model_url = format!(
        "https://model-{}.api.baseten.co/environments/production/sync",
        model_id
    );

    let provider = BasetenClient::new().model_url(model_url).build();

    // Create an embedding model
    let model = provider.text_embedding_model(None);

    // Generate embeddings
    let texts = vec![
        "The capital of France is Paris.".to_string(),
        "The capital of Germany is Berlin.".to_string(),
        "The capital of Spain is Madrid.".to_string(),
    ];

    let result = EmbedMany::new(model, texts).execute().await?;

    println!("Number of embeddings: {}", result.embeddings.len());
    if !result.embeddings.is_empty() {
        println!("Embedding dimension: {}", result.embeddings[0].len());
        println!(
            "First embedding (first 5 values): {:?}",
            &result.embeddings[0][..5.min(result.embeddings[0].len())]
        );
    }
    println!("Usage: {:?}", result.usage);

    Ok(())
}
