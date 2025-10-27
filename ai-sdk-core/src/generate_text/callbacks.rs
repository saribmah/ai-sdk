use super::step_result::StepResult;
use ai_sdk_provider::language_model::usage::Usage;
use async_trait::async_trait;
use std::future::Future;

/// Callback that is called when a step finishes.
///
/// This callback is invoked after each generation step completes,
/// allowing you to process or log intermediate results.
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::{OnStepFinish, StepResult};
/// use async_trait::async_trait;
///
/// struct MyStepCallback;
///
/// #[async_trait]
/// impl OnStepFinish for MyStepCallback {
///     async fn call(&self, step_result: StepResult) {
///         println!("Step finished with {} tokens", step_result.usage.total_tokens);
///     }
/// }
/// ```
#[async_trait]
pub trait OnStepFinish: Send + Sync {
    /// Called when a step finishes.
    ///
    /// # Arguments
    ///
    /// * `step_result` - The result of the completed step
    async fn call(&self, step_result: StepResult);
}

// Blanket implementation for async closures
#[async_trait]
impl<F, Fut> OnStepFinish for F
where
    F: Fn(StepResult) -> Fut + Send + Sync,
    Fut: Future<Output = ()> + Send,
{
    async fn call(&self, step_result: StepResult) {
        self(step_result).await
    }
}

/// Event passed to the `onFinish` callback.
///
/// This combines the final step result with aggregate information
/// about all steps in the generation process.
///
/// # Example
///
/// ```
/// use ai_sdk_core::{FinishEvent, StepResult};
/// use ai_sdk_provider::language_model::usage::Usage;
///
/// // Create a finish event
/// let event = FinishEvent {
///     text: "Hello".to_string(),
///     tool_calls: vec![],
///     tool_results: vec![],
///     finish_reason: ai_sdk_provider::language_model::finish_reason::FinishReason::Stop,
///     usage: Usage::new(10, 20),
///     warnings: None,
///     steps: vec![],
///     total_usage: Usage::new(50, 100),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct FinishEvent {
    /// The generated text from the final step.
    pub text: String,

    /// Tool calls from the final step.
    pub tool_calls: Vec<ai_sdk_provider::language_model::tool_call::ToolCall>,

    /// Tool results from the final step.
    pub tool_results: Vec<ai_sdk_provider::language_model::tool_result::ToolResult>,

    /// The reason why the generation finished.
    pub finish_reason: ai_sdk_provider::language_model::finish_reason::FinishReason,

    /// The token usage of the final step.
    pub usage: Usage,

    /// Warnings from the final step.
    pub warnings: Option<Vec<ai_sdk_provider::language_model::call_warning::CallWarning>>,

    /// Details for all steps.
    ///
    /// This includes all intermediate steps plus the final step.
    pub steps: Vec<StepResult>,

    /// Total usage for all steps.
    ///
    /// This is the sum of the usage of all steps.
    pub total_usage: Usage,
}

impl FinishEvent {
    /// Creates a new FinishEvent from a final step result and all steps.
    ///
    /// # Arguments
    ///
    /// * `final_step` - The final step result
    /// * `all_steps` - All steps including the final step
    ///
    /// # Example
    ///
    /// ```ignore
    /// use ai_sdk_core::FinishEvent;
    ///
    /// let event = FinishEvent::new(final_step, all_steps);
    /// println!("Total tokens used: {}", event.total_usage.total_tokens);
    /// ```
    pub fn new(final_step: &StepResult, all_steps: Vec<StepResult>) -> Self {
        // Calculate total usage across all steps
        let total_usage = all_steps.iter().fold(Usage::new(0, 0), |acc, step| {
            Usage {
                input_tokens: acc.input_tokens + step.usage.input_tokens,
                output_tokens: acc.output_tokens + step.usage.output_tokens,
                total_tokens: acc.total_tokens + step.usage.total_tokens,
                reasoning_tokens: acc.reasoning_tokens + step.usage.reasoning_tokens,
                cached_input_tokens: acc.cached_input_tokens + step.usage.cached_input_tokens,
            }
        });

        Self {
            text: final_step.text(),
            tool_calls: final_step.tool_calls().into_iter().cloned().collect(),
            tool_results: final_step.tool_results().into_iter().cloned().collect(),
            finish_reason: final_step.finish_reason.clone(),
            usage: final_step.usage,
            warnings: final_step.warnings.clone(),
            steps: all_steps,
            total_usage,
        }
    }
}

/// Callback that is called when generation finishes.
///
/// This callback is invoked once all steps complete, providing
/// aggregate information about the entire generation process.
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::{OnFinish, FinishEvent};
/// use async_trait::async_trait;
///
/// struct MyFinishCallback;
///
/// #[async_trait]
/// impl OnFinish for MyFinishCallback {
///     async fn call(&self, event: FinishEvent) {
///         println!("Generation finished!");
///         println!("Total steps: {}", event.steps.len());
///         println!("Total tokens: {}", event.total_usage.total_tokens);
///     }
/// }
/// ```
#[async_trait]
pub trait OnFinish: Send + Sync {
    /// Called when generation finishes.
    ///
    /// # Arguments
    ///
    /// * `event` - The finish event containing the final result and all steps
    async fn call(&self, event: FinishEvent);
}

// Blanket implementation for async closures
#[async_trait]
impl<F, Fut> OnFinish for F
where
    F: Fn(FinishEvent) -> Fut + Send + Sync,
    Fut: Future<Output = ()> + Send,
{
    async fn call(&self, event: FinishEvent) {
        self(event).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ai_sdk_provider::language_model::{
        content::Content, finish_reason::FinishReason, text::Text, tool_call::ToolCall,
        usage::Usage,
    };

    fn create_test_step(input_tokens: u64, output_tokens: u64) -> StepResult {
        StepResult::new(
            vec![Content::Text(Text::new("Test response"))],
            FinishReason::Stop,
            Usage::new(input_tokens, output_tokens),
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
    async fn test_on_step_finish_trait() {
        struct TestCallback;

        #[async_trait]
        impl OnStepFinish for TestCallback {
            async fn call(&self, _step_result: StepResult) {
                // Test implementation
            }
        }

        let callback = TestCallback;
        let step = create_test_step(10, 20);
        callback.call(step).await;
    }

    #[tokio::test]
    async fn test_finish_event_new() {
        let step1 = create_test_step(10, 20);
        let step2 = create_test_step(15, 25);
        let step3 = create_test_step(20, 30);

        let all_steps = vec![step1.clone(), step2.clone(), step3.clone()];
        let event = FinishEvent::new(&step3, all_steps.clone());

        // Check final step data
        assert_eq!(event.finish_reason, FinishReason::Stop);
        assert_eq!(event.usage.input_tokens, 20);
        assert_eq!(event.usage.output_tokens, 30);

        // Check aggregate data
        assert_eq!(event.steps.len(), 3);
        assert_eq!(event.total_usage.input_tokens, 45); // 10 + 15 + 20
        assert_eq!(event.total_usage.output_tokens, 75); // 20 + 25 + 30
        assert_eq!(event.total_usage.total_tokens, 120); // 30 + 40 + 50
    }

    #[tokio::test]
    async fn test_finish_event_single_step() {
        let step = create_test_step(10, 20);
        let all_steps = vec![step.clone()];
        let event = FinishEvent::new(&step, all_steps);

        assert_eq!(event.steps.len(), 1);
        assert_eq!(event.total_usage.input_tokens, 10);
        assert_eq!(event.total_usage.output_tokens, 20);
        assert_eq!(event.total_usage.total_tokens, 30);
    }

    #[tokio::test]
    async fn test_finish_event_text_extraction() {
        let step = create_test_step(10, 20);
        let all_steps = vec![step.clone()];
        let event = FinishEvent::new(&step, all_steps);

        assert_eq!(event.text, "Test response");
    }

    #[tokio::test]
    async fn test_finish_event_with_tool_calls() {
        let content = vec![
            Content::Text(Text::new("Calling tool")),
            Content::ToolCall(ToolCall::new("call_1", "get_weather", "{}")),
        ];

        let step = StepResult::new(
            content,
            FinishReason::ToolCalls,
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
        );

        let all_steps = vec![step.clone()];
        let event = FinishEvent::new(&step, all_steps);

        assert_eq!(event.tool_calls.len(), 1);
        assert_eq!(event.tool_calls[0].tool_name, "get_weather");
        assert_eq!(event.finish_reason, FinishReason::ToolCalls);
    }

    #[tokio::test]
    async fn test_on_finish_trait() {
        struct TestCallback;

        #[async_trait]
        impl OnFinish for TestCallback {
            async fn call(&self, _event: FinishEvent) {
                // Test implementation
            }
        }

        let callback = TestCallback;
        let step = create_test_step(10, 20);
        let event = FinishEvent::new(&step, vec![step.clone()]);
        callback.call(event).await;
    }

    #[tokio::test]
    async fn test_finish_event_with_reasoning_tokens() {
        let step = StepResult::new(
            vec![Content::Text(Text::new("Test"))],
            FinishReason::Stop,
            Usage {
                input_tokens: 10,
                output_tokens: 20,
                total_tokens: 30,
                reasoning_tokens: 5,
                cached_input_tokens: 2,
            },
            None,
            super::super::step_result::RequestMetadata { body: None },
            super::super::step_result::StepResponseMetadata {
                id: None,
                timestamp: None,
                model_id: None,
                body: None,
            },
            None,
        );

        let all_steps = vec![step.clone()];
        let event = FinishEvent::new(&step, all_steps);

        assert_eq!(event.total_usage.reasoning_tokens, 5);
        assert_eq!(event.total_usage.cached_input_tokens, 2);
    }

    #[tokio::test]
    async fn test_finish_event_aggregates_all_usage_fields() {
        let step1 = StepResult::new(
            vec![Content::Text(Text::new("Step 1"))],
            FinishReason::Stop,
            Usage {
                input_tokens: 10,
                output_tokens: 20,
                total_tokens: 30,
                reasoning_tokens: 3,
                cached_input_tokens: 1,
            },
            None,
            super::super::step_result::RequestMetadata { body: None },
            super::super::step_result::StepResponseMetadata {
                id: None,
                timestamp: None,
                model_id: None,
                body: None,
            },
            None,
        );

        let step2 = StepResult::new(
            vec![Content::Text(Text::new("Step 2"))],
            FinishReason::Stop,
            Usage {
                input_tokens: 15,
                output_tokens: 25,
                total_tokens: 40,
                reasoning_tokens: 5,
                cached_input_tokens: 2,
            },
            None,
            super::super::step_result::RequestMetadata { body: None },
            super::super::step_result::StepResponseMetadata {
                id: None,
                timestamp: None,
                model_id: None,
                body: None,
            },
            None,
        );

        let all_steps = vec![step1, step2.clone()];
        let event = FinishEvent::new(&step2, all_steps);

        assert_eq!(event.total_usage.input_tokens, 25); // 10 + 15
        assert_eq!(event.total_usage.output_tokens, 45); // 20 + 25
        assert_eq!(event.total_usage.total_tokens, 70); // 30 + 40
        assert_eq!(event.total_usage.reasoning_tokens, 8); // 3 + 5
        assert_eq!(event.total_usage.cached_input_tokens, 3); // 1 + 2
    }
}
