//! AI SDK Core - Unified interface for building AI-powered applications
//!
//! This crate provides the core functionality of the AI SDK, including builder APIs
//! for text generation, streaming, embeddings, image generation, speech synthesis,
//! transcription, and reranking. It also includes a powerful agent system and
//! type-safe tool integration.
//!
//! # Architecture
//!
//! The SDK follows a layered architecture:
//!
//! - **Builder APIs**: Ergonomic interfaces for AI operations (`GenerateText`, `StreamText`, `Embed`, etc.)
//! - **Agent System**: Reusable AI agents with persistent configuration
//! - **Tool System**: Dynamic and type-safe tool integration for function calling
//! - **Prompt Management**: Standardized message types and prompt handling
//! - **Output Types**: Unified output representations (text, reasoning, sources)
//!
//! # Examples
//!
//! ## Text Generation
//!
//! ```no_run
//! use ai_sdk_core::GenerateText;
//! use ai_sdk_core::prompt::Prompt;
//! use ai_sdk_provider::Provider;
//! # use std::sync::Arc;
//! # async fn example(provider: Arc<dyn Provider>) -> Result<(), Box<dyn std::error::Error>> {
//!
//! let model = provider.language_model("gpt-4")?;
//!
//! let result = GenerateText::new(model, Prompt::text("What is the capital of France?"))
//!     .temperature(0.7)
//!     .max_output_tokens(100)
//!     .execute()
//!     .await?;
//!
//! println!("Response: {}", result.text);
//! # Ok(())
//! # }
//! ```
//!
//! ## Text Streaming
//!
//! ```no_run
//! use ai_sdk_core::StreamText;
//! use ai_sdk_core::prompt::Prompt;
//! use ai_sdk_core::stream_text::TextStreamPart;
//! # use ai_sdk_provider::Provider;
//! # use std::sync::Arc;
//! # use futures::StreamExt;
//! # async fn example(provider: Arc<dyn Provider>) -> Result<(), Box<dyn std::error::Error>> {
//!
//! let model = provider.language_model("gpt-4")?;
//!
//! let result = StreamText::new(model, Prompt::text("Write a poem"))
//!     .temperature(0.8)
//!     .execute()
//!     .await?;
//!
//! let mut stream = result.text_stream();
//! while let Some(text) = stream.next().await {
//!     print!("{}", text);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Embeddings
//!
//! ```no_run
//! use ai_sdk_core::Embed;
//! # use ai_sdk_provider::Provider;
//! # use std::sync::Arc;
//! # async fn example(provider: Arc<dyn Provider>) -> Result<(), Box<dyn std::error::Error>> {
//!
//! let embedding_model = provider.text_embedding_model("text-embedding-3-small")?;
//!
//! let result = Embed::new(embedding_model, "Hello world".to_string())
//!     .execute()
//!     .await?;
//!
//! println!("Embedding dimension: {}", result.embedding.len());
//! # Ok(())
//! # }
//! ```
//!
//! ## Agent Pattern
//!
//! ```no_run
//! use ai_sdk_core::{Agent, AgentSettings, AgentCallParameters};
//! use ai_sdk_core::agent::AgentInterface;
//! # use std::sync::Arc;
//! # use ai_sdk_provider::LanguageModel;
//! # use ai_sdk_core::tool::ToolSet;
//! # async fn example(model: Arc<dyn LanguageModel>, tools: ToolSet) -> Result<(), Box<dyn std::error::Error>> {
//!
//! // Configure agent with persistent settings
//! let settings = AgentSettings::new(model)
//!     .with_tools(tools)
//!     .with_temperature(0.7)
//!     .with_max_output_tokens(500);
//!
//! let agent = Agent::new(settings);
//!
//! // Use agent multiple times with consistent settings
//! let result = agent.generate(AgentCallParameters::from_text("Hello"))?
//!     .execute()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Tool Calling
//!
//! ```no_run
//! use ai_sdk_core::GenerateText;
//! use ai_sdk_core::prompt::Prompt;
//! use ai_sdk_core::tool::{Tool, ToolSet};
//! use ai_sdk_core::tool::definition::ToolExecutionOutput;
//! use serde_json::json;
//! # use std::sync::Arc;
//! # use ai_sdk_provider::LanguageModel;
//! # async fn example(model: Arc<dyn LanguageModel>) -> Result<(), Box<dyn std::error::Error>> {
//!
//! // Create a tool
//! let weather_tool = Tool::function(json!({
//!     "type": "object",
//!     "properties": {
//!         "location": {"type": "string", "description": "City name"}
//!     },
//!     "required": ["location"]
//! }))
//!     .with_description("Get the current weather for a location")
//!     .with_execute(Arc::new(|_input, _opts| {
//!         ToolExecutionOutput::Single(Box::pin(async move {
//!             Ok(json!("Sunny, 72°F"))
//!         }))
//!     }));
//!
//! let mut tools = ToolSet::new();
//! tools.insert("get_weather".to_string(), weather_tool);
//!
//! let result = GenerateText::new(model, Prompt::text("What's the weather in Paris?"))
//!     .tools(tools)
//!     .execute()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Features
//!
//! - **Multiple AI Operations**: Text generation, streaming, embeddings, images, speech, transcription, reranking
//! - **Agent System**: Reusable agents with persistent configuration
//! - **Tool Integration**: Dynamic and type-safe tools for function calling
//! - **Multi-step Execution**: Automatic tool execution with multiple reasoning steps
//! - **Streaming Support**: Real-time streaming with transforms and callbacks
//! - **Type Safety**: Comprehensive type system with compile-time guarantees
//! - **Error Handling**: Rich error types with detailed information
//!
//! # Module Organization
//!
//! - [`agent`]: Agent system for reusable AI agents
//! - [`embed`]: Embedding generation (single and batch)
//! - [`error`]: Error types for the SDK
//! - [`generate_image`]: Image generation
//! - [`generate_speech`]: Speech synthesis
//! - [`generate_text`]: Text generation with tool calling
//! - [`output`]: Unified output types (text, reasoning, sources)
//! - [`prompt`]: Message types and prompt management
//! - [`rerank`]: Document reranking
//! - [`stream_text`]: Text streaming with callbacks
//! - [`tool`]: Tool system for function calling
//! - [`transcribe`]: Audio transcription

#![warn(missing_docs)]

/// Agent system for reusable AI agents with persistent configuration.
pub mod agent;
/// Embedding generation (single and batch operations).
pub mod embed;
/// Error types for the AI SDK.
pub mod error;
/// Image generation functionality.
pub mod generate_image;
/// Speech synthesis functionality.
pub mod generate_speech;
/// Text generation with tool calling support.
pub mod generate_text;
/// Unified output types for text, reasoning, and sources.
pub mod output;
/// Message types and prompt management.
pub mod prompt;
/// Document reranking functionality.
pub mod rerank;
/// Storage configuration and error handling (requires storage feature).
#[cfg(feature = "storage")]
pub mod storage_config;
/// Storage conversion utilities (requires storage feature).
#[cfg(feature = "storage")]
pub mod storage_conversion;
/// Text streaming with callbacks and transforms.
pub mod stream_text;
/// Tool system for function calling (dynamic and type-safe).
pub mod tool;
/// Audio transcription functionality.
pub mod transcribe;

pub use agent::{
    Agent, AgentCallParameters, AgentFinishEvent, AgentInterface, AgentOnFinishCallback,
    AgentOnStepFinishCallback, AgentSettings, noop_agent_on_finish_callback,
    noop_agent_on_step_finish_callback,
};
pub use embed::{
    Embed, EmbedMany, EmbedManyResult, EmbedManyResultResponseData, EmbedResult,
    EmbedResultResponseData,
};
pub use error::AISDKError;
pub use generate_image::{GenerateImage, GenerateImageResult, ImageModelResponseMetadata};
pub use generate_speech::{
    GenerateSpeech, GenerateSpeechResult, GeneratedAudioFile, GeneratedAudioFileWithType,
};
pub use generate_text::{
    FinishEvent, GenerateText, GenerateTextResult, GeneratedFile, HasToolCall, OnFinish,
    OnStepFinish, PrepareStep, PrepareStepOptions, PrepareStepResult, RequestMetadata,
    ResponseMessage, ResponseMetadata, StepCountIs, StepResponseMetadata, StepResult,
    StopCondition, as_output, has_tool_call, is_stop_condition_met, step_count_is,
    to_response_messages,
};
pub use output::{Output, reasoning::ReasoningOutput, source::SourceOutput, text::TextOutput};
pub use rerank::{RankedDocumentWithValue, Rerank, RerankResponseMetadata, RerankResult};
#[cfg(feature = "storage")]
pub use storage_config::{
    StorageConfig, StorageErrorBehavior, StorageTelemetry, retry_with_backoff,
};
pub use stream_text::{
    AbortEvent, AsyncIterableStream, ChunkEvent, ChunkStreamPart, ConsumeStreamOptions, ErrorEvent,
    ErrorHandler, OnAbortCallback, OnChunkCallback, OnErrorCallback, OnFinishCallback,
    OnStepFinishCallback, StreamFinishEvent, StreamGeneratedFile, StreamText, StreamTextResult,
    TextStreamPart,
};
pub use tool::{
    OnPreliminaryToolResult, Tool, ToolApprovalRequest, ToolApprovalRequestOutput,
    ToolApprovalResponse, ToolCallRepairFunction, ToolCallRepairOptions, ToolExecuteFunction,
    ToolExecutionOutput, ToolNeedsApprovalFunction, ToolSet, ToolType, execute_tool_call,
    is_approval_needed, no_repair, parse_provider_executed_dynamic_tool_call, parse_tool_call,
    prepare_tools_and_tool_choice,
};
pub use transcribe::{AudioInput, Transcribe, TranscriptionResult};
