use super::step_result::StepResult;
use crate::message::ModelMessage;
use ai_sdk_provider::language_model::{
    tool_choice::ToolChoice,
};
use async_trait::async_trait;
use std::future::Future;

/// Options passed to a prepare step function.
///
/// These options provide context about the current generation state,
/// allowing you to make informed decisions about step-specific settings.
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::PrepareStepOptions;
///
/// fn my_prepare_step(options: &PrepareStepOptions) -> Option<PrepareStepResult> {
///     // Access current state
///     let step_count = options.steps.len();
///     let current_step = options.step_number;
///
///     // Return custom settings for this step
///     Some(PrepareStepResult {
///         tool_choice: Some(ToolChoice::Required),
///         ..Default::default()
///     })
/// }
/// ```
pub struct PrepareStepOptions<'a> {
    /// The steps that have been executed so far.
    pub steps: &'a [StepResult],

    /// The number of the step that is being executed (0-indexed).
    pub step_number: usize,

    /// The messages being sent to the model for this step.
    pub messages: &'a [ModelMessage],
}

/// Result returned by a prepare step function.
///
/// This allows you to override settings for a specific step in the generation process.
/// All fields are optional - if `None`, the setting from the outer level will be used.
///
/// # Example
///
/// ```
/// use ai_sdk_core::PrepareStepResult;
/// use ai_sdk_provider::language_model::tool_choice::ToolChoice;
///
/// // Force tool usage on step 3
/// let result = PrepareStepResult {
///     tool_choice: Some(ToolChoice::Required),
///     active_tools: Some(vec!["get_weather".to_string()]),
///     system: Some("Be concise".to_string()),
///     messages: None,
/// };
/// ```
#[derive(Debug, Clone, Default)]
pub struct PrepareStepResult {
    /// Optional tool choice override for this step.
    ///
    /// If provided, this will override the tool choice setting for this specific step.
    pub tool_choice: Option<ToolChoice>,

    /// Optional subset of tools to make available for this step.
    ///
    /// If provided, only these tools (by name) will be passed to the model.
    /// This allows you to restrict which tools are available based on the current state.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Only allow the "final_answer" tool on the last step
    /// PrepareStepResult {
    ///     active_tools: Some(vec!["final_answer".to_string()]),
    ///     ..Default::default()
    /// }
    /// ```
    pub active_tools: Option<Vec<String>>,

    /// Optional system message override for this step.
    ///
    /// If provided, this will replace or add a system message for this specific step.
    pub system: Option<String>,

    /// Optional messages override for this step.
    ///
    /// If provided, this will replace the messages for this specific step.
    pub messages: Option<Vec<ModelMessage>>,
}

/// A trait for implementing step preparation logic.
///
/// Step preparation allows you to customize settings for each step in a multi-step
/// generation process. This is useful for:
/// - Changing tool availability based on progress
/// - Modifying the system prompt for different steps
/// - Adjusting tool choice strategy dynamically
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::{PrepareStep, PrepareStepOptions, PrepareStepResult};
/// use async_trait::async_trait;
///
/// struct MyPrepareStep;
///
/// #[async_trait]
/// impl PrepareStep for MyPrepareStep {
///     async fn prepare(&self, options: &PrepareStepOptions<'_>) -> Option<PrepareStepResult> {
///         // On the final step, force a final_answer tool call
///         if options.step_number == 9 {
///             Some(PrepareStepResult {
///                 active_tools: Some(vec!["final_answer".to_string()]),
///                 tool_choice: Some(ToolChoice::Required),
///                 ..Default::default()
///             })
///         } else {
///             None // Use default settings
///         }
///     }
/// }
/// ```
#[async_trait]
pub trait PrepareStep: Send + Sync {
    /// Prepares settings for a specific step.
    ///
    /// # Arguments
    ///
    /// * `options` - Context about the current generation state
    ///
    /// # Returns
    ///
    /// - `Some(PrepareStepResult)` to override settings for this step
    /// - `None` to use the default settings from the outer level
    async fn prepare(&self, options: &PrepareStepOptions<'_>) -> Option<PrepareStepResult>;
}

// Blanket implementation for async closures
#[async_trait]
impl<F, Fut> PrepareStep for F
where
    F: Fn(&PrepareStepOptions<'_>) -> Fut + Send + Sync,
    Fut: Future<Output = Option<PrepareStepResult>> + Send,
{
    async fn prepare(&self, options: &PrepareStepOptions<'_>) -> Option<PrepareStepResult> {
        self(options).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ai_sdk_provider::language_model::{
        finish_reason::FinishReason, usage::Usage,
    };
    use super::super::content_part::ContentPart;
    use super::super::text_output::TextOutput;

    fn create_test_step() -> StepResult {
        StepResult::new(
            vec![ContentPart::Text(TextOutput::new("Test".to_string()))],
            FinishReason::Stop,
            Usage::new(10, 20),
            None,
            super::super::step_result::RequestMetadata { body: None },
            super::super::step_result::StepResponseMetadata {
                id: None,
                timestamp: None,
                model_id: None,
                body: None,
            },
            None,
        )
    }

    #[tokio::test]
    async fn test_prepare_step_options() {
        let steps = vec![create_test_step()];
        let messages: Vec<ModelMessage> = vec![];

        let options = PrepareStepOptions {
            steps: &steps,
            step_number: 1,
            messages: &messages,
        };

        assert_eq!(options.steps.len(), 1);
        assert_eq!(options.step_number, 1);
    }

    #[tokio::test]
    async fn test_prepare_step_result_default() {
        let result = PrepareStepResult::default();

        assert!(result.tool_choice.is_none());
        assert!(result.active_tools.is_none());
        assert!(result.system.is_none());
        assert!(result.messages.is_none());
    }

    #[tokio::test]
    async fn test_prepare_step_result_with_values() {
        let result = PrepareStepResult {
            tool_choice: Some(ToolChoice::Required),
            active_tools: Some(vec!["tool1".to_string(), "tool2".to_string()]),
            system: Some("Test system".to_string()),
            messages: None,
        };

        assert!(matches!(result.tool_choice, Some(ToolChoice::Required)));
        assert_eq!(result.active_tools.as_ref().unwrap().len(), 2);
        assert_eq!(result.system.as_ref().unwrap(), "Test system");
        assert!(result.messages.is_none());
    }

    #[tokio::test]
    async fn test_prepare_step_trait_implementation() {
        struct CustomPrepare;

        #[async_trait]
        impl PrepareStep for CustomPrepare {
            async fn prepare(
                &self,
                options: &PrepareStepOptions<'_>,
            ) -> Option<PrepareStepResult> {
                if options.step_number > 5 {
                    Some(PrepareStepResult {
                        tool_choice: Some(ToolChoice::Required),
                        ..Default::default()
                    })
                } else {
                    None
                }
            }
        }

        let prepare = CustomPrepare;
        let steps = vec![create_test_step()];
        let messages = vec![];

        let options = PrepareStepOptions {
            steps: &steps,
            step_number: 3,
            messages: &messages,
        };

        let result = prepare.prepare(&options).await;
        assert!(result.is_none());

        let options_later = PrepareStepOptions {
            steps: &steps,
            step_number: 7,
            messages: &messages,
        };

        let result_later = prepare.prepare(&options_later).await;
        assert!(result_later.is_some());
        assert!(matches!(
            result_later.unwrap().tool_choice,
            Some(ToolChoice::Required)
        ));
    }

    #[tokio::test]
    async fn test_prepare_step_with_active_tools() {
        struct ToolFilterPrepare;

        #[async_trait]
        impl PrepareStep for ToolFilterPrepare {
            async fn prepare(
                &self,
                options: &PrepareStepOptions<'_>,
            ) -> Option<PrepareStepResult> {
                // On step 0, allow all tools; on step 1+, only allow final_answer
                if options.step_number > 0 {
                    Some(PrepareStepResult {
                        active_tools: Some(vec!["final_answer".to_string()]),
                        ..Default::default()
                    })
                } else {
                    None
                }
            }
        }

        let prepare = ToolFilterPrepare;
        let steps = vec![];
        let messages = vec![];

        let options = PrepareStepOptions {
            steps: &steps,
            step_number: 0,
            messages: &messages,
        };

        let result = prepare.prepare(&options).await;
        assert!(result.is_none());

        let options_next = PrepareStepOptions {
            steps: &steps,
            step_number: 1,
            messages: &messages,
        };

        let result_next = prepare.prepare(&options_next).await;
        assert!(result_next.is_some());
        let active_tools = result_next.unwrap().active_tools.unwrap();
        assert_eq!(active_tools.len(), 1);
        assert_eq!(active_tools[0], "final_answer");
    }
}
