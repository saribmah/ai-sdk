use serde::{Deserialize, Serialize};

/// An embedding is a vector, i.e. an array of numbers.
/// It is e.g. used to represent a text as a vector of word embeddings.
pub type EmbeddingModelEmbedding = Vec<f64>;

/// Helper struct for serialization/deserialization if needed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Embedding {
    /// The embedding values as a vector of floating-point numbers.
    pub values: Vec<f64>,
}

impl Embedding {
    /// Create a new embedding from a vector of values
    pub fn new(values: Vec<f64>) -> Self {
        Self { values }
    }

    /// Get the dimensionality of the embedding
    pub fn dimension(&self) -> usize {
        self.values.len()
    }

    /// Get a reference to the underlying values
    pub fn as_slice(&self) -> &[f64] {
        &self.values
    }
}

impl From<Vec<f64>> for Embedding {
    fn from(values: Vec<f64>) -> Self {
        Self::new(values)
    }
}

impl From<Embedding> for Vec<f64> {
    fn from(embedding: Embedding) -> Self {
        embedding.values
    }
}
