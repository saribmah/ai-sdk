use crate::generate_text::{PrepareStep, StopCondition};
use crate::output::Output;
use crate::prompt::Prompt;
use crate::prompt::call_settings::CallSettings;
use crate::tool::{ToolCallRepairFunction, ToolSet};
use ai_sdk_provider::LanguageModel;
use ai_sdk_provider::language_model::tool_choice::LanguageModelToolChoice;
use ai_sdk_provider::shared::provider_options::SharedProviderOptions;
use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use super::interface::AgentCallParameters;
use super::{AgentOnFinishCallback, AgentOnStepFinishCallback};

/// Configuration options for an agent.
///
/// This struct combines call settings with agent-specific configuration
/// like the model, tools, instructions, and callbacks.
///
/// # Type Parameters
///
/// * `CallOptions` - Optional type for provider-specific call options
///
/// # Examples
///
/// ```ignore
/// use ai_sdk_core::agent::AgentSettings;
///
/// let settings = AgentSettings::new(model)
///     .with_id("my-agent")
///     .with_instructions("You are a helpful assistant")
///     .with_tools(tools)
///     .with_temperature(0.7);
/// ```
pub struct AgentSettings<CallOptions = ()> {
    // Core agent settings
    /// The id of the agent.
    pub id: Option<String>,

    /// The instructions for the agent.
    pub instructions: Option<String>,

    /// The language model to use.
    pub model: Arc<dyn LanguageModel>,

    /// The tools that the model can call. The model needs to support calling tools.
    /// Wrapped in Arc to allow sharing without cloning the tools.
    pub tools: Option<Arc<ToolSet>>,

    /// The tool choice strategy. Default: 'auto'.
    pub tool_choice: Option<LanguageModelToolChoice>,

    /// Condition for stopping the generation when there are tool results in the last step.
    /// When the condition is a vector, any of the conditions can be met to stop the generation.
    ///
    /// Default: step_count_is(20)
    pub stop_when: Option<Vec<Arc<dyn StopCondition>>>,

    /// Limits the tools that are available for the model to call without
    /// changing the tool call and result types in the result.
    pub active_tools: Option<Vec<String>>,

    /// Optional specification for generating structured outputs.
    pub output: Option<Output>,

    /// Optional trait object that you can use to provide different settings for a step.
    pub prepare_step: Option<Arc<dyn PrepareStep>>,

    /// A function that attempts to repair a tool call that failed to parse.
    pub experimental_repair_tool_call: Option<ToolCallRepairFunction>,

    /// Callback that is called when each step (LLM call) is finished, including intermediate steps.
    pub on_step_finish: Option<AgentOnStepFinishCallback>,

    /// Callback that is called when all steps are finished and the response is complete.
    pub on_finish: Option<AgentOnFinishCallback>,

    /// Additional provider-specific options. They are passed through
    /// to the provider from the AI SDK and enable provider-specific
    /// functionality that can be fully encapsulated in the provider.
    pub provider_options: Option<SharedProviderOptions>,

    /// Context that is passed into tool calls.
    ///
    /// Experimental (can break in patch releases).
    pub experimental_context: Option<Value>,

    /// The schema for the call options (stored as JSON schema).
    pub call_options_schema: Option<Value>,

    /// Prepare the parameters for the generateText or streamText call.
    ///
    /// You can use this to have templates based on call options.
    pub prepare_call: Option<PrepareCallFunction<CallOptions>>,

    // Call settings (inherited from CallSettings)
    /// Maximum number of tokens to generate.
    pub max_output_tokens: Option<u32>,

    /// Temperature setting for sampling.
    pub temperature: Option<f64>,

    /// Nucleus sampling parameter.
    pub top_p: Option<f64>,

    /// Top-K sampling parameter.
    pub top_k: Option<u32>,

    /// Presence penalty for repetition.
    pub presence_penalty: Option<f64>,

    /// Frequency penalty for repetition.
    pub frequency_penalty: Option<f64>,

    /// Stop sequences for generation.
    pub stop_sequences: Option<Vec<String>>,

    /// Random seed for deterministic generation.
    pub seed: Option<u32>,

    /// Additional HTTP headers for the request.
    pub headers: Option<HashMap<String, String>>,
}

/// Function type for preparing call parameters.
///
/// This function receives the agent call parameters and current settings,
/// and returns updated settings along with a prompt (without system message).
pub type PrepareCallFunction<CallOptions> = Arc<
    dyn Fn(
            PrepareCallInput<CallOptions>,
        ) -> Pin<
            Box<dyn Future<Output = Result<PrepareCallOutput, crate::error::AISDKError>> + Send>,
        > + Send
        + Sync,
>;

/// Input to the prepare call function.
pub struct PrepareCallInput<CallOptions> {
    /// The agent call parameters (prompt/messages + options)
    pub call_params: AgentCallParameters<CallOptions>,

    /// Current model
    pub model: Arc<dyn LanguageModel>,

    /// Current tools (read-only, cannot be modified in prepare_call output)
    /// Use active_tools to limit which tools are available instead.
    pub tools: Option<Arc<ToolSet>>,

    /// Current max output tokens
    pub max_output_tokens: Option<u32>,

    /// Current temperature
    pub temperature: Option<f64>,

    /// Current top_p
    pub top_p: Option<f64>,

    /// Current top_k
    pub top_k: Option<u32>,

    /// Current presence penalty
    pub presence_penalty: Option<f64>,

    /// Current frequency penalty
    pub frequency_penalty: Option<f64>,

    /// Current stop sequences
    pub stop_sequences: Option<Vec<String>>,

    /// Current seed
    pub seed: Option<u32>,

    /// Current headers
    pub headers: Option<HashMap<String, String>>,

    /// Current instructions
    pub instructions: Option<String>,

    /// Current stop condition
    pub stop_when: Option<Vec<Arc<dyn StopCondition>>>,

    /// Current active tools
    pub active_tools: Option<Vec<String>>,

    /// Current provider options
    pub provider_options: Option<SharedProviderOptions>,

    /// Current experimental context
    pub experimental_context: Option<Value>,
}

/// Output from the prepare call function.
pub struct PrepareCallOutput {
    /// Updated model (if changed)
    pub model: Option<Arc<dyn LanguageModel>>,

    /// Updated max output tokens (if changed)
    pub max_output_tokens: Option<u32>,

    /// Updated temperature (if changed)
    pub temperature: Option<f64>,

    /// Updated top_p (if changed)
    pub top_p: Option<f64>,

    /// Updated top_k (if changed)
    pub top_k: Option<u32>,

    /// Updated presence penalty (if changed)
    pub presence_penalty: Option<f64>,

    /// Updated frequency penalty (if changed)
    pub frequency_penalty: Option<f64>,

    /// Updated stop sequences (if changed)
    pub stop_sequences: Option<Vec<String>>,

    /// Updated seed (if changed)
    pub seed: Option<u32>,

    /// Updated headers (if changed)
    pub headers: Option<HashMap<String, String>>,

    /// Updated instructions (if changed)
    pub instructions: Option<String>,

    /// Updated stop condition (if changed)
    pub stop_when: Option<Vec<Arc<dyn StopCondition>>>,

    /// Updated active tools (if changed)
    pub active_tools: Option<Vec<String>>,

    /// Updated provider options (if changed)
    pub provider_options: Option<SharedProviderOptions>,

    /// Updated experimental context (if changed)
    pub experimental_context: Option<Value>,

    /// The prompt to use (without system message, which comes from instructions)
    pub prompt: Prompt,
}

impl<CallOptions> AgentSettings<CallOptions> {
    /// Creates new agent settings with the specified model.
    pub fn new(model: Arc<dyn LanguageModel>) -> Self {
        Self {
            id: None,
            instructions: None,
            model,
            tools: None,
            tool_choice: None,
            stop_when: None,
            active_tools: None,
            output: None,
            prepare_step: None,
            experimental_repair_tool_call: None,
            on_step_finish: None,
            on_finish: None,
            provider_options: None,
            experimental_context: None,
            call_options_schema: None,
            prepare_call: None,
            max_output_tokens: None,
            temperature: None,
            top_p: None,
            top_k: None,
            presence_penalty: None,
            frequency_penalty: None,
            stop_sequences: None,
            seed: None,
            headers: None,
        }
    }

    /// Sets the agent ID.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the agent instructions.
    pub fn with_instructions(mut self, instructions: impl Into<String>) -> Self {
        self.instructions = Some(instructions.into());
        self
    }

    /// Sets the tools.
    pub fn with_tools(mut self, tools: ToolSet) -> Self {
        self.tools = Some(Arc::new(tools));
        self
    }

    /// Sets the tool choice strategy.
    pub fn with_tool_choice(mut self, tool_choice: LanguageModelToolChoice) -> Self {
        self.tool_choice = Some(tool_choice);
        self
    }

    /// Sets the stop condition.
    pub fn with_stop_when(mut self, stop_when: Vec<Arc<dyn StopCondition>>) -> Self {
        self.stop_when = Some(stop_when);
        self
    }

    /// Sets the active tools.
    pub fn with_active_tools(mut self, active_tools: Vec<String>) -> Self {
        self.active_tools = Some(active_tools);
        self
    }

    /// Sets the output specification.
    pub fn with_output(mut self, output: Output) -> Self {
        self.output = Some(output);
        self
    }

    /// Sets the prepare step trait object.
    pub fn with_prepare_step(mut self, prepare_step: Arc<dyn PrepareStep>) -> Self {
        self.prepare_step = Some(prepare_step);
        self
    }

    /// Sets the tool call repair function.
    pub fn with_experimental_repair_tool_call(mut self, repair_fn: ToolCallRepairFunction) -> Self {
        self.experimental_repair_tool_call = Some(repair_fn);
        self
    }

    /// Sets the on step finish callback.
    pub fn with_on_step_finish(mut self, callback: AgentOnStepFinishCallback) -> Self {
        self.on_step_finish = Some(callback);
        self
    }

    /// Sets the on finish callback.
    pub fn with_on_finish(mut self, callback: AgentOnFinishCallback) -> Self {
        self.on_finish = Some(callback);
        self
    }

    /// Sets the provider options.
    pub fn with_provider_options(mut self, options: SharedProviderOptions) -> Self {
        self.provider_options = Some(options);
        self
    }

    /// Sets the experimental context.
    pub fn with_experimental_context(mut self, context: Value) -> Self {
        self.experimental_context = Some(context);
        self
    }

    /// Sets the prepare call function.
    pub fn with_prepare_call(mut self, prepare_call: PrepareCallFunction<CallOptions>) -> Self {
        self.prepare_call = Some(prepare_call);
        self
    }

    /// Sets the maximum number of output tokens.
    pub fn with_max_output_tokens(mut self, max_output_tokens: u32) -> Self {
        self.max_output_tokens = Some(max_output_tokens);
        self
    }

    /// Sets the temperature.
    pub fn with_temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Sets the top_p parameter.
    pub fn with_top_p(mut self, top_p: f64) -> Self {
        self.top_p = Some(top_p);
        self
    }

    /// Sets the top_k parameter.
    pub fn with_top_k(mut self, top_k: u32) -> Self {
        self.top_k = Some(top_k);
        self
    }

    /// Sets the presence penalty.
    pub fn with_presence_penalty(mut self, presence_penalty: f64) -> Self {
        self.presence_penalty = Some(presence_penalty);
        self
    }

    /// Sets the frequency penalty.
    pub fn with_frequency_penalty(mut self, frequency_penalty: f64) -> Self {
        self.frequency_penalty = Some(frequency_penalty);
        self
    }

    /// Sets the stop sequences.
    pub fn with_stop_sequences(mut self, stop_sequences: Vec<String>) -> Self {
        self.stop_sequences = Some(stop_sequences);
        self
    }

    /// Sets the random seed.
    pub fn with_seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Sets the HTTP headers.
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Converts the agent settings to call settings.
    pub fn to_call_settings(&self) -> CallSettings {
        CallSettings {
            max_output_tokens: self.max_output_tokens,
            temperature: self.temperature,
            top_p: self.top_p,
            top_k: self.top_k,
            presence_penalty: self.presence_penalty,
            frequency_penalty: self.frequency_penalty,
            stop_sequences: self.stop_sequences.clone(),
            seed: self.seed,
            max_retries: None,
            abort_signal: None,
            headers: self.headers.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    // Note: We can't easily test with actual LanguageModel instances in unit tests,
    // so we'll test basic functionality when we have a concrete implementation

    #[test]
    fn test_agent_settings_to_call_settings() {
        // Test that we can extract call settings from agent settings
        // This will be tested more thoroughly with actual model instances
    }
}
