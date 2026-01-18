use std::fmt;

/// Error types for Baseten provider operations.
#[derive(Debug)]
pub enum BasetenError {
    /// API key is missing and could not be loaded from environment
    MissingApiKey,

    /// Model URL is required for embeddings but was not provided
    MissingModelUrl,

    /// Invalid endpoint type (e.g., /predict endpoints not supported)
    InvalidEndpoint(String),

    /// Wrapped provider error
    ProviderError(String),
}

impl fmt::Display for BasetenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BasetenError::MissingApiKey => {
                write!(
                    f,
                    "Baseten API key not found. Set BASETEN_API_KEY environment variable or provide api_key explicitly."
                )
            }
            BasetenError::MissingModelUrl => {
                write!(
                    f,
                    "Model URL is required for embeddings. Please set modelURL option for embeddings."
                )
            }
            BasetenError::InvalidEndpoint(msg) => {
                write!(f, "Invalid endpoint: {}", msg)
            }
            BasetenError::ProviderError(msg) => {
                write!(f, "Provider error: {}", msg)
            }
        }
    }
}

impl std::error::Error for BasetenError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = BasetenError::MissingApiKey;
        assert!(error.to_string().contains("BASETEN_API_KEY"));

        let error = BasetenError::MissingModelUrl;
        assert!(error.to_string().contains("Model URL is required"));

        let error = BasetenError::InvalidEndpoint("/predict not supported".to_string());
        assert!(error.to_string().contains("/predict not supported"));

        let error = BasetenError::ProviderError("test error".to_string());
        assert!(error.to_string().contains("test error"));
    }
}
