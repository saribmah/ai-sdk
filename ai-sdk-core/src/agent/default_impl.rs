use crate::error::AISDKError;
use crate::generate_text::{GenerateText, GenerateTextResult, step_count_is};
use crate::prompt::Prompt;
use crate::stream_text::{StreamText, StreamTextResult};
use crate::tool::ToolSet;
use async_trait::async_trait;
use std::sync::Arc;

use super::agent_settings::{AgentSettings, PrepareCallInput};
use super::interface::{AgentCallParameters, AgentInterface, AgentPrompt};

/// An agent that runs tools in a loop.
///
/// In each step, it calls the LLM, and if there are tool calls, it executes the tools
/// and calls the LLM again in a new step with the tool results.
///
/// The loop continues until:
/// - A finish reason other than tool-calls is returned, or
/// - A tool that is invoked does not have an execute function, or
/// - A tool call needs approval, or
/// - A stop condition is met (default stop condition is step_count_is(20))
///
/// # Type Parameters
///
/// * `CallOptions` - Optional type for provider-specific call options
///
/// # Examples
///
/// ```ignore
/// use ai_sdk_core::agent::Agent;
///
/// let agent = Agent::new(settings);
///
/// // Generate non-streaming
/// let result = agent.generate(AgentCallParameters::with_prompt("Hello")).await?;
///
/// // Stream
/// let result = agent.stream(AgentCallParameters::with_prompt("Hello")).await?;
/// ```
pub struct Agent<CallOptions = ()> {
    settings: AgentSettings<CallOptions>,
    // We need to store an empty toolset Arc for the case where settings.tools is None
    empty_tools: Arc<ToolSet>,
}

impl<CallOptions> Agent<CallOptions> {
    /// Creates a new agent with the given settings.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let settings = AgentSettings::new(model)
    ///     .with_instructions("You are a helpful assistant")
    ///     .with_tools(tools);
    ///
    /// let agent = Agent::new(settings);
    /// ```
    pub fn new(settings: AgentSettings<CallOptions>) -> Self {
        Self {
            settings,
            empty_tools: Arc::new(ToolSet::new()),
        }
    }

    /// Gets the agent settings.
    pub fn settings(&self) -> &AgentSettings<CallOptions> {
        &self.settings
    }

    /// Prepares the call parameters by applying the prepare_call function if provided.
    ///
    /// This method:
    /// 1. Combines base settings with call options
    /// 2. Sets default stop condition (step_count_is(20)) if not provided
    /// 3. Calls the optional prepare_call function to customize parameters
    /// 4. Returns prepared settings and prompt
    async fn prepare_call(
        &self,
        options: AgentCallParameters<CallOptions>,
    ) -> Result<PreparedCall, AISDKError> {
        // Set default stop condition if not provided
        let stop_when = if self.settings.stop_when.is_none() {
            Some(vec![
                Arc::new(step_count_is(20)) as Arc<dyn crate::generate_text::StopCondition>
            ])
        } else {
            self.settings.stop_when.clone()
        };

        // If there's a prepare_call function, use it
        if let Some(prepare_call_fn) = &self.settings.prepare_call {
            let input = PrepareCallInput {
                call_params: options,
                model: self.settings.model.clone(),
                tools: self.settings.tools.clone(),
                max_output_tokens: self.settings.max_output_tokens,
                temperature: self.settings.temperature,
                top_p: self.settings.top_p,
                top_k: self.settings.top_k,
                presence_penalty: self.settings.presence_penalty,
                frequency_penalty: self.settings.frequency_penalty,
                stop_sequences: self.settings.stop_sequences.clone(),
                seed: self.settings.seed,
                headers: self.settings.headers.clone(),
                instructions: self.settings.instructions.clone(),
                stop_when: stop_when.clone(),
                active_tools: self.settings.active_tools.clone(),
                provider_options: self.settings.provider_options.clone(),
                experimental_context: self.settings.experimental_context.clone(),
            };

            let output = prepare_call_fn(input).await?;

            // Build the final prompt from prepare_call output
            let prompt = if let Some(ref instructions) = output.instructions {
                output.prompt.with_system(instructions.clone())
            } else if let Some(ref instructions) = self.settings.instructions {
                output.prompt.with_system(instructions.clone())
            } else {
                output.prompt
            };

            Ok(PreparedCall {
                model: output.model.unwrap_or_else(|| self.settings.model.clone()),
                tools: self.settings.tools.clone(),
                tool_choice: self.settings.tool_choice.clone(),
                stop_when: output.stop_when.or(stop_when),
                prepare_step: self.settings.prepare_step.clone(),
                provider_options: output
                    .provider_options
                    .or_else(|| self.settings.provider_options.clone()),
                max_output_tokens: output.max_output_tokens.or(self.settings.max_output_tokens),
                temperature: output.temperature.or(self.settings.temperature),
                top_p: output.top_p.or(self.settings.top_p),
                top_k: output.top_k.or(self.settings.top_k),
                presence_penalty: output.presence_penalty.or(self.settings.presence_penalty),
                frequency_penalty: output.frequency_penalty.or(self.settings.frequency_penalty),
                stop_sequences: output
                    .stop_sequences
                    .or_else(|| self.settings.stop_sequences.clone()),
                seed: output.seed.or(self.settings.seed),
                headers: output.headers.or_else(|| self.settings.headers.clone()),
                prompt,
            })
        } else {
            // No prepare_call function, use settings directly
            let prompt = self.build_prompt_from_options(&options)?;

            Ok(PreparedCall {
                model: self.settings.model.clone(),
                tools: self.settings.tools.clone(),
                tool_choice: self.settings.tool_choice.clone(),
                stop_when,
                prepare_step: self.settings.prepare_step.clone(),
                provider_options: self.settings.provider_options.clone(),
                max_output_tokens: self.settings.max_output_tokens,
                temperature: self.settings.temperature,
                top_p: self.settings.top_p,
                top_k: self.settings.top_k,
                presence_penalty: self.settings.presence_penalty,
                frequency_penalty: self.settings.frequency_penalty,
                stop_sequences: self.settings.stop_sequences.clone(),
                seed: self.settings.seed,
                headers: self.settings.headers.clone(),
                prompt,
            })
        }
    }

    /// Builds a prompt from call parameters and agent instructions.
    fn build_prompt_from_options(
        &self,
        options: &AgentCallParameters<CallOptions>,
    ) -> Result<Prompt, AISDKError> {
        // Validate the options
        options.validate().map_err(|e| {
            AISDKError::invalid_argument_builder("options")
                .message(e)
                .build()
        })?;

        // Convert AgentPrompt or messages to Prompt
        let mut prompt = if let Some(ref agent_prompt) = options.prompt {
            match agent_prompt {
                AgentPrompt::Text(text) => Prompt::text(text.clone()),
                AgentPrompt::Messages(msgs) => Prompt::messages(msgs.clone()),
            }
        } else if let Some(ref msgs) = options.messages {
            Prompt::messages(msgs.clone())
        } else {
            return Err(AISDKError::invalid_argument_builder("prompt")
                .message("Either prompt or messages must be provided")
                .build());
        };

        // Add system instructions if provided
        if let Some(ref instructions) = self.settings.instructions {
            prompt = prompt.with_system(instructions.clone());
        }

        Ok(prompt)
    }
}

impl<CallOptions: Send + Sync> AgentInterface for Agent<CallOptions> {
    type CallOptions = CallOptions;
    type Output = ();

    fn id(&self) -> Option<&str> {
        self.settings.id.as_deref()
    }

    fn tools(&self) -> &ToolSet {
        self.settings
            .tools
            .as_ref()
            .map(|arc| arc.as_ref())
            .unwrap_or(&self.empty_tools)
    }

    async fn generate(
        &self,
        params: AgentCallParameters<Self::CallOptions>,
    ) -> Result<GenerateTextResult, AISDKError> {
        let prepared = self.prepare_call(params).await?;

        let mut builder = GenerateText::new(prepared.model, prepared.prompt);

        // Apply all settings to the builder
        if let Some(tools_arc) = prepared.tools {
            // Try to unwrap the Arc if we have the only reference
            // Otherwise, we can't pass tools since Tool doesn't implement Clone
            match Arc::try_unwrap(tools_arc) {
                Ok(tools) => {
                    builder = builder.tools(tools);
                }
                Err(_arc) => {
                    // Multiple references exist - this shouldn't happen in normal usage
                    // since PreparedCall owns its tools Arc
                    return Err(AISDKError::invalid_argument_builder("tools")
                        .message("Cannot use tools: Arc has multiple references")
                        .build());
                }
            }
        }
        if let Some(tool_choice) = prepared.tool_choice {
            builder = builder.tool_choice(tool_choice);
        }
        if let Some(stop_when) = prepared.stop_when {
            // Convert Arc<dyn StopCondition> to Box<dyn StopCondition>
            let boxed_conditions: Vec<Box<dyn crate::generate_text::StopCondition>> = stop_when
                .into_iter()
                .map(|arc| {
                    Box::new(ArcStopConditionWrapper(arc))
                        as Box<dyn crate::generate_text::StopCondition>
                })
                .collect();
            builder = builder.stop_when(boxed_conditions);
        }
        if let Some(prepare_step) = prepared.prepare_step {
            // Convert Arc<dyn PrepareStep> to Box<dyn PrepareStep>
            builder = builder.prepare_step(Box::new(ArcPrepareStepWrapper(prepare_step)));
        }
        if let Some(provider_options) = prepared.provider_options {
            builder = builder.provider_options(provider_options);
        }
        if let Some(max_tokens) = prepared.max_output_tokens {
            builder = builder.max_output_tokens(max_tokens);
        }
        if let Some(temperature) = prepared.temperature {
            builder = builder.temperature(temperature);
        }
        if let Some(top_p) = prepared.top_p {
            builder = builder.top_p(top_p);
        }
        if let Some(top_k) = prepared.top_k {
            builder = builder.top_k(top_k);
        }
        if let Some(presence_penalty) = prepared.presence_penalty {
            builder = builder.presence_penalty(presence_penalty);
        }
        if let Some(frequency_penalty) = prepared.frequency_penalty {
            builder = builder.frequency_penalty(frequency_penalty);
        }
        if let Some(stop_sequences) = prepared.stop_sequences {
            builder = builder.stop_sequences(stop_sequences);
        }
        if let Some(seed) = prepared.seed {
            builder = builder.seed(seed);
        }
        if let Some(headers) = prepared.headers {
            builder = builder.headers(headers);
        }

        builder.execute().await
    }

    async fn stream(
        &self,
        params: AgentCallParameters<Self::CallOptions>,
    ) -> Result<StreamTextResult, AISDKError> {
        let prepared = self.prepare_call(params).await?;

        let mut builder = StreamText::new(prepared.model, prepared.prompt);

        // Apply all settings to the builder
        if let Some(tools_arc) = prepared.tools {
            // Try to unwrap the Arc if we have the only reference
            // Otherwise, we can't pass tools since Tool doesn't implement Clone
            match Arc::try_unwrap(tools_arc) {
                Ok(tools) => {
                    builder = builder.tools(tools);
                }
                Err(_arc) => {
                    // Multiple references exist - this shouldn't happen in normal usage
                    // since PreparedCall owns its tools Arc
                    return Err(AISDKError::invalid_argument_builder("tools")
                        .message("Cannot use tools: Arc has multiple references")
                        .build());
                }
            }
        }
        if let Some(tool_choice) = prepared.tool_choice {
            builder = builder.tool_choice(tool_choice);
        }
        if let Some(stop_when) = prepared.stop_when {
            // Convert Arc<dyn StopCondition> to Box<dyn StopCondition>
            let boxed_conditions: Vec<Box<dyn crate::generate_text::StopCondition>> = stop_when
                .into_iter()
                .map(|arc| {
                    Box::new(ArcStopConditionWrapper(arc))
                        as Box<dyn crate::generate_text::StopCondition>
                })
                .collect();
            builder = builder.stop_when(boxed_conditions);
        }
        if let Some(prepare_step) = prepared.prepare_step {
            // Convert Arc<dyn PrepareStep> to Box<dyn PrepareStep>
            builder = builder.prepare_step(Box::new(ArcPrepareStepWrapper(prepare_step)));
        }
        if let Some(provider_options) = prepared.provider_options {
            builder = builder.provider_options(provider_options);
        }
        if let Some(max_tokens) = prepared.max_output_tokens {
            builder = builder.max_output_tokens(max_tokens);
        }
        if let Some(temperature) = prepared.temperature {
            builder = builder.temperature(temperature);
        }
        if let Some(top_p) = prepared.top_p {
            builder = builder.top_p(top_p);
        }
        if let Some(top_k) = prepared.top_k {
            builder = builder.top_k(top_k);
        }
        if let Some(presence_penalty) = prepared.presence_penalty {
            builder = builder.presence_penalty(presence_penalty);
        }
        if let Some(frequency_penalty) = prepared.frequency_penalty {
            builder = builder.frequency_penalty(frequency_penalty);
        }
        if let Some(stop_sequences) = prepared.stop_sequences {
            builder = builder.stop_sequences(stop_sequences);
        }
        if let Some(seed) = prepared.seed {
            builder = builder.seed(seed);
        }
        if let Some(headers) = prepared.headers {
            builder = builder.headers(headers);
        }

        builder.execute().await
    }
}

/// Internal struct holding prepared call parameters.
struct PreparedCall {
    model: Arc<dyn ai_sdk_provider::LanguageModel>,
    tools: Option<Arc<ToolSet>>,
    tool_choice: Option<ai_sdk_provider::language_model::tool_choice::LanguageModelToolChoice>,
    stop_when: Option<Vec<Arc<dyn crate::generate_text::StopCondition>>>,
    prepare_step: Option<Arc<dyn crate::generate_text::PrepareStep>>,
    provider_options: Option<ai_sdk_provider::shared::provider_options::SharedProviderOptions>,
    max_output_tokens: Option<u32>,
    temperature: Option<f64>,
    top_p: Option<f64>,
    top_k: Option<u32>,
    presence_penalty: Option<f64>,
    frequency_penalty: Option<f64>,
    stop_sequences: Option<Vec<String>>,
    seed: Option<u32>,
    headers: Option<std::collections::HashMap<String, String>>,
    prompt: Prompt,
}

/// Wrapper that converts Arc<dyn StopCondition> to a type that implements StopCondition.
struct ArcStopConditionWrapper(Arc<dyn crate::generate_text::StopCondition>);

#[async_trait]
impl crate::generate_text::StopCondition for ArcStopConditionWrapper {
    async fn check(&self, steps: &[crate::generate_text::StepResult]) -> bool {
        self.0.check(steps).await
    }
}

/// Wrapper that converts Arc<dyn PrepareStep> to a type that implements PrepareStep.
struct ArcPrepareStepWrapper(Arc<dyn crate::generate_text::PrepareStep>);

#[async_trait]
impl crate::generate_text::PrepareStep for ArcPrepareStepWrapper {
    async fn prepare(
        &self,
        options: &crate::generate_text::PrepareStepOptions<'_>,
    ) -> Option<crate::generate_text::PrepareStepResult> {
        self.0.prepare(options).await
    }
}

#[cfg(test)]
mod tests {
    // Note: Full integration tests will require actual LanguageModel implementations.
    // These are basic structural tests.

    #[test]
    fn test_agent_can_be_created() {
        // This test will be expanded when we have mock models available
    }
}
