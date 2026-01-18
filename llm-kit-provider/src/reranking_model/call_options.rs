use crate::shared::provider_options::SharedProviderOptions;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tokio_util::sync;

/// Documents to rerank.
/// Either a list of texts or a list of JSON objects.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum RerankingDocuments {
    /// Text documents
    Text {
        /// List of text strings
        values: Vec<String>,
    },
    /// JSON object documents
    Object {
        /// List of JSON objects
        values: Vec<HashMap<String, Value>>,
    },
}

impl RerankingDocuments {
    /// Create documents from a list of text strings
    pub fn from_text(values: Vec<String>) -> Self {
        Self::Text { values }
    }

    /// Create documents from a single text string
    pub fn from_single_text(text: impl Into<String>) -> Self {
        Self::Text {
            values: vec![text.into()],
        }
    }

    /// Create documents from a list of JSON objects
    pub fn from_objects(values: Vec<HashMap<String, Value>>) -> Self {
        Self::Object { values }
    }

    /// Create documents from a single JSON object
    pub fn from_single_object(obj: HashMap<String, Value>) -> Self {
        Self::Object { values: vec![obj] }
    }

    /// Get the number of documents
    pub fn len(&self) -> usize {
        match self {
            Self::Text { values } => values.len(),
            Self::Object { values } => values.len(),
        }
    }

    /// Check if there are no documents
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Check if this contains text documents
    pub fn is_text(&self) -> bool {
        matches!(self, Self::Text { .. })
    }

    /// Check if this contains object documents
    pub fn is_object(&self) -> bool {
        matches!(self, Self::Object { .. })
    }

    /// Get the text values if this contains text documents
    pub fn as_text(&self) -> Option<&Vec<String>> {
        match self {
            Self::Text { values } => Some(values),
            Self::Object { .. } => None,
        }
    }

    /// Get the object values if this contains object documents
    pub fn as_objects(&self) -> Option<&Vec<HashMap<String, Value>>> {
        match self {
            Self::Text { .. } => None,
            Self::Object { values } => Some(values),
        }
    }
}

impl From<Vec<String>> for RerankingDocuments {
    fn from(values: Vec<String>) -> Self {
        Self::from_text(values)
    }
}

impl From<Vec<HashMap<String, Value>>> for RerankingDocuments {
    fn from(values: Vec<HashMap<String, Value>>) -> Self {
        Self::from_objects(values)
    }
}

/// Reranking model call options.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RerankingModelCallOptions {
    /// Documents to rerank.
    /// Either a list of texts or a list of JSON objects.
    pub documents: RerankingDocuments,

    /// The query is a string that represents the query to rerank the documents against.
    pub query: String,

    /// Optional limit returned documents to the top n documents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_n: Option<usize>,

    /// Additional provider-specific options. They are passed through
    /// to the provider from the LLM Kit and enable provider-specific
    /// functionality that can be fully encapsulated in the provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<SharedProviderOptions>,

    /// Additional HTTP headers to be sent with the request.
    /// Only applicable for HTTP-based providers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,

    /// Abort/cancellation signal (not serialized, used for runtime control).
    #[serde(skip)]
    pub abort_signal: Option<sync::CancellationToken>,
}

impl RerankingModelCallOptions {
    /// Create new call options with documents and a query.
    pub fn new(documents: impl Into<RerankingDocuments>, query: impl Into<String>) -> Self {
        Self {
            documents: documents.into(),
            query: query.into(),
            top_n: None,
            provider_options: None,
            headers: None,
            abort_signal: None,
        }
    }

    /// Create new call options with text documents and a query.
    pub fn with_text(texts: Vec<String>, query: impl Into<String>) -> Self {
        Self::new(RerankingDocuments::from_text(texts), query)
    }

    /// Create new call options with JSON object documents and a query.
    pub fn with_objects(objects: Vec<HashMap<String, Value>>, query: impl Into<String>) -> Self {
        Self::new(RerankingDocuments::from_objects(objects), query)
    }

    // Builder methods
    /// Set the maximum number of documents to return
    pub fn with_top_n(mut self, top_n: usize) -> Self {
        self.top_n = Some(top_n);
        self
    }

    /// Set provider-specific options
    pub fn with_provider_options(mut self, options: SharedProviderOptions) -> Self {
        self.provider_options = Some(options);
        self
    }

    /// Set HTTP headers for the request
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Set an abort signal to cancel the request
    pub fn with_abort_signal(mut self, signal: sync::CancellationToken) -> Self {
        self.abort_signal = Some(signal);
        self
    }
}
