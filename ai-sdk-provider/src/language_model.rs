use crate::language_model::call_options::LanguageModelCallOptions;
use crate::language_model::call_warning::LanguageModelCallWarning;
use crate::language_model::content::LanguageModelContent;
use crate::language_model::finish_reason::LanguageModelFinishReason;
use crate::language_model::response_metadata::LanguageModelResponseMetadata;
use crate::language_model::stream_part::StreamPart;
use crate::language_model::usage::LanguageModelUsage;
use crate::shared::headers::Headers;
use crate::shared::provider_metadata::ProviderMetadata;
use async_trait::async_trait;
use futures::Stream;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

pub mod call_options;
pub mod call_warning;
pub mod content;
mod data_content;
pub mod finish_reason;
pub mod prompt;
pub mod response_metadata;
pub mod stream_part;
pub mod tool;
pub mod tool_choice;
pub mod usage;

#[async_trait]
pub trait LanguageModel: Send + Sync {
    fn specification_version(&self) -> &str {
        "v2"
    }

    /// Name of the provider for logging purposes.
    fn provider(&self) -> &str;

    /// Provider-specific model ID for logging purposes.
    fn model_id(&self) -> &str;

    async fn supported_urls(&self) -> HashMap<String, Vec<Regex>>;

    async fn do_generate(
        &self,
        options: LanguageModelCallOptions,
    ) -> Result<LanguageModelGenerateResponse, Box<dyn std::error::Error>>;

    async fn do_stream(
        &self,
        options: LanguageModelCallOptions,
    ) -> Result<LanguageModelStreamResponse, Box<dyn std::error::Error>>;
}

#[derive(Debug)]
pub struct LanguageModelGenerateResponse {
    pub content: Vec<LanguageModelContent>,
    pub finish_reason: LanguageModelFinishReason,
    pub usage: LanguageModelUsage,
    pub provider_metadata: Option<ProviderMetadata>,
    pub request: Option<LanguageModelRequestMetadata>,
    pub response: Option<LanguageModelResponseMetadata>,
    pub warnings: Vec<LanguageModelCallWarning>,
}

pub struct LanguageModelStreamResponse {
    pub stream: Box<dyn Stream<Item = StreamPart> + Unpin + Send>,
    pub request: Option<LanguageModelRequestMetadata>,
    pub response: Option<StreamResponseMetadata>,
}

#[derive(Debug)]
pub struct LanguageModelRequestMetadata {
    pub body: Option<Value>,
}

pub struct StreamResponseMetadata {
    pub headers: Option<Headers>,
}
