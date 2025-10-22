use std::collections::HashMap;
use async_trait::async_trait;
use futures::Stream;
use regex::Regex;
use serde_json::Value;
use crate::language_model::call_options::CallOptions;
use crate::language_model::call_warning::CallWarning;
use crate::language_model::content::Content;
use crate::language_model::finish_reason::FinishReason;
use crate::language_model::response_metadata::ResponseMetadata;
use crate::language_model::stream_part::StreamPart;
use crate::language_model::usage::Usage;
use crate::shared::headers::Headers;
use crate::shared::provider_metadata::ProviderMetadata;

pub mod call_options;
mod data_content;
mod file;
mod finish_reason;
pub mod prompt;
pub mod provider_defined_tool;
mod reasoning;
mod response_metadata;
mod source;
mod text;
mod tool_call;
pub mod tool_choice;
mod tool_result;
mod usage;
pub mod function_tool;
mod call_warning;
mod content;
mod stream_part;

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

    async fn do_generate(&self, options: CallOptions) -> Result<LanguageModelGenerateResponse, Box<dyn std::error::Error>>;

    async fn do_stream(&self, options: CallOptions) -> Result<LanguageModelStreamResponse, Box<dyn std::error::Error>>;
}

pub struct LanguageModelGenerateResponse {
    pub content: Vec<Content>,
    pub finish_reason: FinishReason,
    pub usage: Usage,
    pub provider_metadata: Option<ProviderMetadata>,
    pub request: Option<RequestMetadata>,
    pub response: Option<ResponseMetadata>,
    pub warnings: Vec<CallWarning>,
}

pub struct LanguageModelStreamResponse {
    pub stream: Box<dyn Stream<Item = StreamPart> + Unpin + Send>,
    pub request: Option<RequestMetadata>,
    pub response: Option<StreamResponseMetadata>,
}

pub struct RequestMetadata {
    pub body: Option<Value>,
}

pub struct StreamResponseMetadata {
    pub headers: Option<Headers>
}
