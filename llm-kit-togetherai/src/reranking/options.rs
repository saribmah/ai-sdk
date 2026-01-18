/// Together AI reranking model identifiers.
///
/// These are the supported reranking models available through Together AI.
/// For details, see: <https://docs.together.ai/docs/serverless-models#rerank-models>
pub type TogetherAIRerankingModelId = String;

/// Common Together AI reranking model IDs
#[allow(dead_code)]
pub mod models {
    /// Salesforce Llama Rank v1
    pub const LLAMA_RANK_V1: &str = "Salesforce/Llama-Rank-v1";

    /// Mixedbread AI Mxbai Rerank Large V2
    pub const MXBAI_RERANK_LARGE_V2: &str = "mixedbread-ai/Mxbai-Rerank-Large-V2";
}

/// Options for Together AI reranking requests.
#[derive(Debug, Clone, Default)]
pub struct TogetherAIRerankingOptions {
    /// List of keys in the JSON Object document to rank by.
    ///
    /// Defaults to use all supplied keys for ranking.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_togetherai::TogetherAIRerankingOptions;
    ///
    /// let options = TogetherAIRerankingOptions {
    ///     rank_fields: Some(vec!["title".to_string(), "text".to_string()]),
    /// };
    /// ```
    pub rank_fields: Option<Vec<String>>,
}

impl TogetherAIRerankingOptions {
    /// Creates a new `TogetherAIRerankingOptions` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the rank fields.
    ///
    /// # Arguments
    ///
    /// * `rank_fields` - List of document field names to use for ranking
    pub fn with_rank_fields(mut self, rank_fields: Vec<String>) -> Self {
        self.rank_fields = Some(rank_fields);
        self
    }
}
