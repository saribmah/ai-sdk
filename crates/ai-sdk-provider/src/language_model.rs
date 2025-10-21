use std::collections::HashMap;
use async_trait::async_trait;
use futures::Stream;
use regex::Regex;
use serde_json::Value;

mod call_options;
mod data_content;
mod file;
mod finish_reason;
mod provider_defined_tool;


#[async_trait]
pub trait LanguageModel {
    fn specification_version(&self) -> &str {
        "v2"
    }

    /// Name of the provider for logging purposes.
    fn provider(&self) -> &str;

    /// Provider-specific model ID for logging purposes.
    fn model_id(&self) -> &str;

    async fn supported_urls(&self) -> HashMap<String, Vec<Regex>>;

    async fn do_generate(&self, options: LanguageModelCallOptions) -> Result<LanguageModelGenerateResponse, Box<dyn std::error::Error>>;

    async fn do_stream(&self, options: LanguageModelCallOptions) -> Result<LanguageModelStreamResponse, Box<dyn std::error::Error>>;
}

pub struct LanguageModelGenerateResponse {
    pub content: Vec<LanguageModelContent>,
    pub finish_reason: LanguageModelFinishReason,
    pub usage: LanguageModelUsage,
    pub provider_metadata: Option<ProviderMetadata>,
    pub request: Option<RequestMetadata>,
    pub response: Option<ResponseMetadata>,
    pub warnings: Vec<LanguageModelCallWarning>,
}

pub struct LanguageModelStreamResponse {
    pub stream: Box<dyn Stream<Item = LanguageModelStreamPart> + Unpin + Send>,
    pub request: Option<RequestMetadata>,
    pub response: Option<StreamResponseMetadata>,
}

pub struct RequestMetadata {
    pub body: Option<Value>,
}

pub struct ResponseMetadata {
    pub headers: Option<Headers>,
    pub body: Option<Value>,

    #[serde(flatten)]
    pub metadata: LanguageModelResponseMetadata,
}

pub struct StreamResponseMetadata {
    pub headers: Option<Headers>
}
