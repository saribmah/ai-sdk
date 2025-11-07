use crate::shared::provider_options::SharedProviderOptions;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio_util::sync;

/// Embedding model call options.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbeddingModelCallOptions<V> {
    /// List of values to embed.
    pub values: Vec<V>,

    /// Additional HTTP headers to be sent with the request.
    /// Only applicable for HTTP-based providers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,

    /// Additional provider-specific options. They are passed through
    /// to the provider from the AI SDK and enable provider-specific
    /// functionality that can be fully encapsulated in the provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<SharedProviderOptions>,

    /// Abort/cancellation signal (not serialized, used for runtime control).
    #[serde(skip)]
    pub abort_signal: Option<sync::CancellationToken>,
}

impl<V> EmbeddingModelCallOptions<V> {
    /// Create new call options with values to embed
    pub fn new(values: Vec<V>) -> Self {
        Self {
            values,
            headers: None,
            provider_options: None,
            abort_signal: None,
        }
    }

    // Builder methods
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }

    pub fn with_provider_options(mut self, options: SharedProviderOptions) -> Self {
        self.provider_options = Some(options);
        self
    }

    pub fn with_abort_signal(mut self, signal: sync::CancellationToken) -> Self {
        self.abort_signal = Some(signal);
        self
    }
}
