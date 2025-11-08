use super::ProviderError;

/// Builder for constructing TooManyEmbeddingValuesForCall error with a fluent interface
///
/// # Examples
///
/// ```
/// use ai_sdk_provider::error::{TooManyEmbeddingValuesForCallErrorBuilder, ProviderError};
///
/// let error = TooManyEmbeddingValuesForCallErrorBuilder::new(
///     "openai",
///     "text-embedding-ada-002",
///     2048,
///     3000,
/// )
/// .build();
///
/// match error {
///     ProviderError::TooManyEmbeddingValuesForCall {
///         provider,
///         model_id,
///         max_embeddings_per_call,
///         values_count,
///     } => {
///         assert_eq!(provider, "openai");
///         assert_eq!(model_id, "text-embedding-ada-002");
///         assert_eq!(max_embeddings_per_call, 2048);
///         assert_eq!(values_count, 3000);
///     }
///     _ => panic!("Expected TooManyEmbeddingValuesForCall error"),
/// }
/// ```
#[derive(Debug, Default)]
pub struct TooManyEmbeddingValuesForCallErrorBuilder {
    provider: String,
    model_id: String,
    max_embeddings_per_call: usize,
    values_count: usize,
}

impl TooManyEmbeddingValuesForCallErrorBuilder {
    /// Create a new TooManyEmbeddingValuesForCallErrorBuilder with required fields
    pub fn new(
        provider: impl Into<String>,
        model_id: impl Into<String>,
        max_embeddings_per_call: usize,
        values_count: usize,
    ) -> Self {
        Self {
            provider: provider.into(),
            model_id: model_id.into(),
            max_embeddings_per_call,
            values_count,
        }
    }

    /// Build the ProviderError
    pub fn build(self) -> ProviderError {
        ProviderError::TooManyEmbeddingValuesForCall {
            provider: self.provider,
            model_id: self.model_id,
            max_embeddings_per_call: self.max_embeddings_per_call,
            values_count: self.values_count,
        }
    }
}

impl ProviderError {
    /// Create a new TooManyEmbeddingValuesForCall error
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_provider::error::ProviderError;
    ///
    /// let error = ProviderError::too_many_embedding_values_for_call(
    ///     "openai",
    ///     "text-embedding-ada-002",
    ///     2048,
    ///     5000,
    /// );
    ///
    /// match error {
    ///     ProviderError::TooManyEmbeddingValuesForCall {
    ///         provider,
    ///         model_id,
    ///         max_embeddings_per_call,
    ///         values_count,
    ///     } => {
    ///         assert_eq!(provider, "openai");
    ///         assert_eq!(model_id, "text-embedding-ada-002");
    ///         assert_eq!(max_embeddings_per_call, 2048);
    ///         assert_eq!(values_count, 5000);
    ///     }
    ///     _ => panic!("Expected TooManyEmbeddingValuesForCall error"),
    /// }
    /// ```
    pub fn too_many_embedding_values_for_call(
        provider: impl Into<String>,
        model_id: impl Into<String>,
        max_embeddings_per_call: usize,
        values_count: usize,
    ) -> Self {
        Self::TooManyEmbeddingValuesForCall {
            provider: provider.into(),
            model_id: model_id.into(),
            max_embeddings_per_call,
            values_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_too_many_embedding_values_basic() {
        let error = ProviderError::too_many_embedding_values_for_call(
            "openai",
            "text-embedding-ada-002",
            2048,
            3000,
        );

        match error {
            ProviderError::TooManyEmbeddingValuesForCall {
                provider,
                model_id,
                max_embeddings_per_call,
                values_count,
            } => {
                assert_eq!(provider, "openai");
                assert_eq!(model_id, "text-embedding-ada-002");
                assert_eq!(max_embeddings_per_call, 2048);
                assert_eq!(values_count, 3000);
            }
            _ => panic!("Expected TooManyEmbeddingValuesForCall error"),
        }
    }

    #[test]
    fn test_too_many_embedding_values_builder() {
        let error = TooManyEmbeddingValuesForCallErrorBuilder::new(
            "anthropic",
            "claude-embed-1",
            1000,
            1500,
        )
        .build();

        match error {
            ProviderError::TooManyEmbeddingValuesForCall {
                provider,
                model_id,
                max_embeddings_per_call,
                values_count,
            } => {
                assert_eq!(provider, "anthropic");
                assert_eq!(model_id, "claude-embed-1");
                assert_eq!(max_embeddings_per_call, 1000);
                assert_eq!(values_count, 1500);
            }
            _ => panic!("Expected TooManyEmbeddingValuesForCall error"),
        }
    }

    #[test]
    fn test_too_many_embedding_values_not_retryable() {
        let error =
            ProviderError::too_many_embedding_values_for_call("provider", "model", 100, 200);
        assert!(!error.is_retryable());
        assert!(error.status_code().is_none());
        assert!(error.url().is_none());
    }

    #[test]
    fn test_error_display() {
        let error =
            ProviderError::too_many_embedding_values_for_call("openai", "ada-002", 2048, 3000);
        let display = format!("{}", error);
        assert!(display.contains("Too many embedding values"));
        assert!(display.contains("openai"));
        assert!(display.contains("ada-002"));
        assert!(display.contains("2048"));
        assert!(display.contains("3000"));
    }

    #[test]
    fn test_builder_creates_same_as_direct() {
        let error1 =
            ProviderError::too_many_embedding_values_for_call("provider", "model", 100, 150);
        let error2 =
            TooManyEmbeddingValuesForCallErrorBuilder::new("provider", "model", 100, 150).build();

        match (error1, error2) {
            (
                ProviderError::TooManyEmbeddingValuesForCall {
                    provider: p1,
                    model_id: m1,
                    max_embeddings_per_call: max1,
                    values_count: v1,
                },
                ProviderError::TooManyEmbeddingValuesForCall {
                    provider: p2,
                    model_id: m2,
                    max_embeddings_per_call: max2,
                    values_count: v2,
                },
            ) => {
                assert_eq!(p1, p2);
                assert_eq!(m1, m2);
                assert_eq!(max1, max2);
                assert_eq!(v1, v2);
            }
            _ => panic!("Expected TooManyEmbeddingValuesForCall errors"),
        }
    }

    #[test]
    fn test_different_providers() {
        let providers = vec![
            ("openai", "text-embedding-ada-002", 2048),
            ("anthropic", "claude-embed", 1000),
            ("cohere", "embed-english-v3.0", 96),
            ("google", "textembedding-gecko", 5),
        ];

        for (provider, model, max) in providers {
            let error =
                ProviderError::too_many_embedding_values_for_call(provider, model, max, max + 100);
            match error {
                ProviderError::TooManyEmbeddingValuesForCall {
                    provider: p,
                    model_id: m,
                    max_embeddings_per_call: max_emb,
                    values_count: v,
                } => {
                    assert_eq!(p, provider);
                    assert_eq!(m, model);
                    assert_eq!(max_emb, max);
                    assert_eq!(v, max + 100);
                }
                _ => panic!("Expected TooManyEmbeddingValuesForCall error"),
            }
        }
    }

    #[test]
    fn test_exact_limit_boundary() {
        // Test when values_count equals max (should still create error, as it's "too many")
        let error =
            ProviderError::too_many_embedding_values_for_call("provider", "model", 100, 100);
        match error {
            ProviderError::TooManyEmbeddingValuesForCall {
                max_embeddings_per_call,
                values_count,
                ..
            } => {
                assert_eq!(max_embeddings_per_call, 100);
                assert_eq!(values_count, 100);
            }
            _ => panic!("Expected TooManyEmbeddingValuesForCall error"),
        }
    }

    #[test]
    fn test_large_numbers() {
        let error = ProviderError::too_many_embedding_values_for_call(
            "provider", "model", 10_000, 1_000_000,
        );
        match error {
            ProviderError::TooManyEmbeddingValuesForCall {
                max_embeddings_per_call,
                values_count,
                ..
            } => {
                assert_eq!(max_embeddings_per_call, 10_000);
                assert_eq!(values_count, 1_000_000);
            }
            _ => panic!("Expected TooManyEmbeddingValuesForCall error"),
        }
    }

    #[test]
    fn test_small_limits() {
        let error = ProviderError::too_many_embedding_values_for_call("provider", "model", 1, 2);
        match error {
            ProviderError::TooManyEmbeddingValuesForCall {
                max_embeddings_per_call,
                values_count,
                ..
            } => {
                assert_eq!(max_embeddings_per_call, 1);
                assert_eq!(values_count, 2);
            }
            _ => panic!("Expected TooManyEmbeddingValuesForCall error"),
        }
    }

    #[test]
    fn test_zero_max() {
        // Edge case: max is 0 (shouldn't happen in practice, but should handle)
        let error = ProviderError::too_many_embedding_values_for_call("provider", "model", 0, 1);
        match error {
            ProviderError::TooManyEmbeddingValuesForCall {
                max_embeddings_per_call,
                values_count,
                ..
            } => {
                assert_eq!(max_embeddings_per_call, 0);
                assert_eq!(values_count, 1);
            }
            _ => panic!("Expected TooManyEmbeddingValuesForCall error"),
        }
    }

    #[test]
    fn test_realistic_scenarios() {
        let scenarios = vec![
            (
                "openai",
                "text-embedding-ada-002",
                2048,
                3000,
                "OpenAI batch limit exceeded",
            ),
            (
                "cohere",
                "embed-english-v3.0",
                96,
                200,
                "Cohere per-request limit exceeded",
            ),
            (
                "custom",
                "my-embedding-model",
                500,
                501,
                "Just over the limit by 1",
            ),
        ];

        for (provider, model, max, count, description) in scenarios {
            let error =
                ProviderError::too_many_embedding_values_for_call(provider, model, max, count);
            assert!(
                matches!(error, ProviderError::TooManyEmbeddingValuesForCall { .. }),
                "Failed for scenario: {}",
                description
            );
        }
    }
}
