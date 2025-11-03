pub mod error;
pub mod language_model;
pub mod provider;
pub mod shared;

// Re-export commonly used types
pub use error::ProviderError;
pub use language_model::{
    LanguageModel, LanguageModelGenerateResponse, LanguageModelStreamResponse, LanguageModelRequestMetadata,
};
pub use provider::Provider;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
