/// Together AI embedding model identifiers.
///
/// These are the supported embedding models available through Together AI.
pub type TogetherAIEmbeddingModelId = String;

/// Common Together AI embedding model IDs
#[allow(dead_code)]
pub mod models {
    /// WhereIsAI UAE Large V1
    pub const UAE_LARGE_V1: &str = "WhereIsAI/UAE-Large-V1";

    /// BAAI BGE Large EN v1.5
    pub const BGE_LARGE_EN_V1_5: &str = "BAAI/bge-large-en-v1.5";

    /// BAAI BGE Base EN v1.5
    pub const BGE_BASE_EN_V1_5: &str = "BAAI/bge-base-en-v1.5";

    /// Sentence Transformers BERT Base NLI Mean Tokens
    pub const BERT_BASE_NLI_MEAN_TOKENS: &str = "sentence-transformers/msmarco-bert-base-dot-v5";
}
