use crate::error::AISDKError;
use crate::generate_text::{GenerateText, step_count_is};
use crate::prompt::{Prompt, PromptContent};
use crate::stream_text::StreamText;
use crate::tool::ToolSet;
use async_trait::async_trait;
use std::sync::Arc;

use super::agent_settings::AgentSettings;
use super::interface::{AgentCallParameters, AgentInterface};

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
/// # Examples
///
/// ```ignore
/// use ai_sdk_core::agent::Agent;
///
/// let agent = Agent::new(settings);
///
/// // Generate non-streaming - returns a builder
/// let result = agent.generate(AgentCallParameters::with_prompt("Hello"))?
///     .execute()
///     .await?;
///
/// // Stream - returns a builder
/// let result = agent.stream(AgentCallParameters::with_prompt("Hello"))?
///     .execute()
///     .await?;
/// ```
pub struct Agent {
    settings: AgentSettings,
}

impl Agent {
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
    pub fn new(settings: AgentSettings) -> Self {
        Self { settings }
    }

    /// Gets the agent settings.
    pub fn settings(&self) -> &AgentSettings {
        &self.settings
    }

    /// Builds a prompt from call parameters and agent instructions.
    fn build_prompt(&self, params: &AgentCallParameters) -> Result<Prompt, AISDKError> {
        // Convert PromptContent to Prompt
        let mut prompt = match &params.prompt {
            PromptContent::Text { text } => Prompt::text(text.clone()),
            PromptContent::Messages { messages } => Prompt::messages(messages.clone()),
        };

        // Add system instructions if provided
        if let Some(ref instructions) = self.settings.instructions {
            prompt = prompt.with_system(instructions.clone());
        }

        Ok(prompt)
    }
}

impl AgentInterface for Agent {
    type Output = ();

    fn id(&self) -> Option<&str> {
        self.settings.id.as_deref()
    }

    fn tools(&self) -> Option<&ToolSet> {
        self.settings.tools.as_ref()
    }

    fn generate(&self, params: AgentCallParameters) -> Result<GenerateText, AISDKError> {
        let prompt = self.build_prompt(&params)?;
        let mut builder = GenerateText::new(self.settings.model.clone(), prompt);

        // Apply tools from agent settings
        if let Some(tools) = &self.settings.tools {
            builder = builder.tools(tools.clone());
        }
        if let Some(tool_choice) = &self.settings.tool_choice {
            builder = builder.tool_choice(tool_choice.clone());
        }

        // Apply stop condition (default to step_count_is(20) if not provided)
        if let Some(stop_conditions) = &self.settings.stop_when {
            // Convert Arc<dyn StopCondition> to Box<dyn StopCondition>
            let boxed_conditions: Vec<Box<dyn crate::generate_text::StopCondition>> =
                stop_conditions
                    .iter()
                    .map(|arc| {
                        Box::new(ArcStopConditionWrapper(arc.clone()))
                            as Box<dyn crate::generate_text::StopCondition>
                    })
                    .collect();
            builder = builder.stop_when(boxed_conditions);
        } else {
            // Default stop condition
            builder = builder.stop_when(vec![Box::new(step_count_is(20))]);
        }

        // Apply other settings
        if let Some(prepare_step) = &self.settings.prepare_step {
            builder = builder.prepare_step(Box::new(ArcPrepareStepWrapper(prepare_step.clone())));
        }
        if let Some(provider_options) = &self.settings.provider_options {
            builder = builder.provider_options(provider_options.clone());
        }
        if let Some(max_tokens) = self.settings.max_output_tokens {
            builder = builder.max_output_tokens(max_tokens);
        }
        if let Some(temperature) = self.settings.temperature {
            builder = builder.temperature(temperature);
        }
        if let Some(top_p) = self.settings.top_p {
            builder = builder.top_p(top_p);
        }
        if let Some(top_k) = self.settings.top_k {
            builder = builder.top_k(top_k);
        }
        if let Some(presence_penalty) = self.settings.presence_penalty {
            builder = builder.presence_penalty(presence_penalty);
        }
        if let Some(frequency_penalty) = self.settings.frequency_penalty {
            builder = builder.frequency_penalty(frequency_penalty);
        }
        if let Some(stop_sequences) = &self.settings.stop_sequences {
            builder = builder.stop_sequences(stop_sequences.clone());
        }
        if let Some(seed) = self.settings.seed {
            builder = builder.seed(seed);
        }
        if let Some(headers) = &self.settings.headers {
            builder = builder.headers(headers.clone());
        }

        Ok(builder)
    }

    fn stream(&self, params: AgentCallParameters) -> Result<StreamText, AISDKError> {
        let prompt = self.build_prompt(&params)?;
        let mut builder = StreamText::new(self.settings.model.clone(), prompt);

        // Apply tools from agent settings
        if let Some(tools) = &self.settings.tools {
            builder = builder.tools(tools.clone());
        }
        if let Some(tool_choice) = &self.settings.tool_choice {
            builder = builder.tool_choice(tool_choice.clone());
        }

        // Apply stop condition (default to step_count_is(20) if not provided)
        if let Some(stop_conditions) = &self.settings.stop_when {
            // Convert Arc<dyn StopCondition> to Box<dyn StopCondition>
            let boxed_conditions: Vec<Box<dyn crate::generate_text::StopCondition>> =
                stop_conditions
                    .iter()
                    .map(|arc| {
                        Box::new(ArcStopConditionWrapper(arc.clone()))
                            as Box<dyn crate::generate_text::StopCondition>
                    })
                    .collect();
            builder = builder.stop_when(boxed_conditions);
        } else {
            // Default stop condition
            builder = builder.stop_when(vec![Box::new(step_count_is(20))]);
        }

        // Apply other settings
        if let Some(prepare_step) = &self.settings.prepare_step {
            builder = builder.prepare_step(Box::new(ArcPrepareStepWrapper(prepare_step.clone())));
        }
        if let Some(provider_options) = &self.settings.provider_options {
            builder = builder.provider_options(provider_options.clone());
        }
        if let Some(max_tokens) = self.settings.max_output_tokens {
            builder = builder.max_output_tokens(max_tokens);
        }
        if let Some(temperature) = self.settings.temperature {
            builder = builder.temperature(temperature);
        }
        if let Some(top_p) = self.settings.top_p {
            builder = builder.top_p(top_p);
        }
        if let Some(top_k) = self.settings.top_k {
            builder = builder.top_k(top_k);
        }
        if let Some(presence_penalty) = self.settings.presence_penalty {
            builder = builder.presence_penalty(presence_penalty);
        }
        if let Some(frequency_penalty) = self.settings.frequency_penalty {
            builder = builder.frequency_penalty(frequency_penalty);
        }
        if let Some(stop_sequences) = &self.settings.stop_sequences {
            builder = builder.stop_sequences(stop_sequences.clone());
        }
        if let Some(seed) = self.settings.seed {
            builder = builder.seed(seed);
        }
        if let Some(headers) = &self.settings.headers {
            builder = builder.headers(headers.clone());
        }

        Ok(builder)
    }
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
