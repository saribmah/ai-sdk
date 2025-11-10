use crate::generate_text::{PrepareStep, StopCondition};
use crate::output::Output;
use crate::prompt::call_settings::CallSettings;
use crate::tool::{ToolCallRepairFunction, ToolSet};
use ai_sdk_provider::LanguageModel;
use ai_sdk_provider::language_model::tool_choice::LanguageModelToolChoice;
use ai_sdk_provider::shared::provider_options::SharedProviderOptions;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

#[cfg(feature = "storage")]
use ai_sdk_storage::Storage;

use super::{AgentOnFinishCallback, AgentOnStepFinishCallback};

/// Configuration options for an agent.
///
/// This struct combines call settings with agent-specific configuration
/// like the model, tools, instructions, and callbacks.
///
/// # Examples
///
/// ```no_run
/// use ai_sdk_core::agent::AgentSettings;
/// # use std::sync::Arc;
/// # use ai_sdk_provider::LanguageModel;
/// # fn example(model: Arc<dyn LanguageModel>, tools: ai_sdk_core::tool::ToolSet) {
///
/// let settings = AgentSettings::new(model)
///     .with_id("my-agent")
///     .with_instructions("You are a helpful assistant")
///     .with_tools(tools)
///     .with_temperature(0.7);
/// # }
/// ```
pub struct AgentSettings {
    // Core agent settings
    /// The id of the agent.
    pub id: Option<String>,

    /// The instructions for the agent.
    pub instructions: Option<String>,

    /// The language model to use.
    pub model: Arc<dyn LanguageModel>,

    /// The tools that the model can call. The model needs to support calling tools.
    /// Tools can be cloned efficiently since they wrap closures in Arc.
    pub tools: Option<ToolSet>,

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

    // Storage settings
    /// Optional storage provider for conversation persistence.
    ///
    /// When set, all agent calls can store and retrieve conversation history.
    /// Requires the "storage" feature flag.
    #[cfg(feature = "storage")]
    pub storage: Option<Arc<dyn Storage>>,

    /// Optional session ID for persistent conversations (stateful mode).
    ///
    /// When set in AgentSettings, all calls from this agent use the same session (stateful mode).
    /// When not set, session_id can be specified per call via builder methods (stateless mode).
    /// Requires the "storage" feature flag.
    #[cfg(feature = "storage")]
    pub session_id: Option<String>,

    /// Storage error handling configuration.
    ///
    /// Controls how storage errors are handled, retry behavior, and telemetry hooks.
    /// Requires the "storage" feature flag.
    #[cfg(feature = "storage")]
    pub storage_config: crate::storage_config::StorageConfig,

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

impl AgentSettings {
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
            #[cfg(feature = "storage")]
            storage: None,
            #[cfg(feature = "storage")]
            session_id: None,
            #[cfg(feature = "storage")]
            storage_config: crate::storage_config::StorageConfig::default(),
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
        self.tools = Some(tools);
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

    /// Sets the storage provider for conversation persistence.
    ///
    /// When storage is configured, agent calls can automatically store and retrieve
    /// conversation history. Use with `with_session_id()` for stateful agents, or
    /// specify session per call for stateless agents.
    ///
    /// Requires the "storage" feature flag.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ai_sdk_core::agent::AgentSettings;
    /// # #[cfg(feature = "storage")]
    /// # use ai_sdk_storage_filesystem::FilesystemStorage;
    /// # use std::sync::Arc;
    /// # use ai_sdk_provider::LanguageModel;
    /// # #[cfg(feature = "storage")]
    /// # async fn example(model: Arc<dyn LanguageModel>) -> Result<(), Box<dyn std::error::Error>> {
    /// # let storage = Arc::new(FilesystemStorage::new("./storage")?);
    ///
    /// // Stateless mode - specify session per call
    /// let settings = AgentSettings::new(model)
    ///     .with_storage(storage);
    ///
    /// // Later, specify session_id when calling:
    /// // agent.generate(params)?.with_session_id("session-123").execute()
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "storage")]
    pub fn with_storage(mut self, storage: Arc<dyn Storage>) -> Self {
        self.storage = Some(storage);
        self
    }

    /// Sets the session ID for persistent conversations (stateful mode).
    ///
    /// When set, all calls from this agent use the same session, enabling
    /// automatic conversation history across multiple interactions.
    ///
    /// Requires the "storage" feature flag and storage to be configured.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ai_sdk_core::agent::AgentSettings;
    /// # #[cfg(feature = "storage")]
    /// # use ai_sdk_storage_filesystem::FilesystemStorage;
    /// # #[cfg(feature = "storage")]
    /// # use ai_sdk_storage::Storage;
    /// # use std::sync::Arc;
    /// # use ai_sdk_provider::LanguageModel;
    /// # #[cfg(feature = "storage")]
    /// # async fn example(model: Arc<dyn LanguageModel>) -> Result<(), Box<dyn std::error::Error>> {
    /// # let storage = Arc::new(FilesystemStorage::new("./storage")?);
    /// # let session_id = storage.generate_session_id();
    ///
    /// // Stateful mode - all calls use the same session
    /// let settings = AgentSettings::new(model)
    ///     .with_storage(storage)
    ///     .with_session_id(session_id);
    ///
    /// // All agent calls automatically load/store history for this session
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "storage")]
    pub fn with_session_id(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }

    /// Configure storage error handling and telemetry.
    ///
    /// Controls how storage errors are handled (log, return, retry) and
    /// provides hooks for monitoring storage operations.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ai_sdk_core::agent::AgentSettings;
    /// use ai_sdk_core::storage_config::{StorageConfig, StorageErrorBehavior};
    /// # use std::sync::Arc;
    /// # use ai_sdk_provider::LanguageModel;
    /// # fn example(model: Arc<dyn LanguageModel>) {
    ///
    /// let config = StorageConfig::new()
    ///     .with_error_behavior(StorageErrorBehavior::default_retry());
    ///
    /// let settings = AgentSettings::new(model)
    ///     .storage_config(config);
    /// # }
    /// ```
    #[cfg(feature = "storage")]
    pub fn storage_config(mut self, config: crate::storage_config::StorageConfig) -> Self {
        self.storage_config = config;
        self
    }

    /// Shortcut for configuring storage error behavior.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ai_sdk_core::agent::AgentSettings;
    /// use ai_sdk_core::storage_config::StorageErrorBehavior;
    /// # use std::sync::Arc;
    /// # use ai_sdk_provider::LanguageModel;
    /// # fn example(model: Arc<dyn LanguageModel>) {
    ///
    /// let settings = AgentSettings::new(model)
    ///     .storage_error_behavior(StorageErrorBehavior::ReturnError);
    /// # }
    /// ```
    #[cfg(feature = "storage")]
    pub fn storage_error_behavior(
        mut self,
        behavior: crate::storage_config::StorageErrorBehavior,
    ) -> Self {
        self.storage_config.error_behavior = behavior;
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
