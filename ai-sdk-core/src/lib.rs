pub mod embed;
pub mod error;
pub mod generate_image;
pub mod generate_speech;
pub mod generate_text;
pub mod output;
pub mod prompt;
pub mod rerank;
pub mod stream_text;
pub mod tool;
pub mod transcribe;

pub use embed::{
    EmbedManyResult, EmbedManyResultResponseData, EmbedResult, EmbedResultResponseData, embed,
    embed_many,
};
pub use error::AISDKError;
pub use generate_image::{GenerateImageResult, ImageModelResponseMetadata, generate_image};
pub use generate_speech::{
    GenerateSpeechResult, GeneratedAudioFile, GeneratedAudioFileWithType, generate_speech,
};
pub use generate_text::{
    FinishEvent, GenerateTextBuilder, GenerateTextResult, GeneratedFile, HasToolCall, OnFinish,
    OnStepFinish, PrepareStep, PrepareStepOptions, PrepareStepResult, RequestMetadata,
    ResponseMessage, ResponseMetadata, StepCountIs, StepResponseMetadata, StepResult,
    StopCondition, as_output, generate_text, has_tool_call, is_stop_condition_met, step_count_is,
    to_response_messages,
};
pub use output::{reasoning::ReasoningOutput, source::SourceOutput, text::TextOutput};
pub use rerank::{RankedDocumentWithValue, RerankResponseMetadata, RerankResult, rerank};
pub use stream_text::{
    AbortEvent, AsyncIterableStream, ChunkEvent, ChunkStreamPart, ConsumeStreamOptions, ErrorEvent,
    ErrorHandler, OnAbortCallback, OnChunkCallback, OnErrorCallback, OnFinishCallback,
    OnStepFinishCallback, StreamFinishEvent, StreamGeneratedFile, StreamTextBuilder,
    StreamTextResult, TextStreamPart, stream_text,
};
pub use tool::{
    OnPreliminaryToolResult, Tool, ToolApprovalRequest, ToolApprovalRequestOutput,
    ToolApprovalResponse, ToolCallRepairFunction, ToolCallRepairOptions, ToolExecuteFunction,
    ToolExecuteOptions, ToolExecutionOutput, ToolNeedsApprovalFunction, ToolSet, ToolType,
    execute_tool_call, is_approval_needed, no_repair, parse_provider_executed_dynamic_tool_call,
    parse_tool_call, prepare_tools_and_tool_choice,
};
pub use transcribe::TranscriptionResult;
