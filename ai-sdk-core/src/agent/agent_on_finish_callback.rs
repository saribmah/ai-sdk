use crate::generate_text::StepResult;
use ai_sdk_provider::language_model::usage::LanguageModelUsage;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Event passed to the `AgentOnFinishCallback`.
///
/// This combines the final step result with all steps and total usage.
#[derive(Debug, Clone)]
pub struct AgentFinishEvent {
    /// The final step result containing all content, tool calls, etc.
    pub step: StepResult,

    /// Details for all steps.
    ///
    /// You can use this to get information about intermediate steps,
    /// such as the tool calls or the response headers.
    pub steps: Vec<StepResult>,

    /// Total usage for all steps.
    ///
    /// This is the sum of the usage of all steps.
    pub total_usage: LanguageModelUsage,
}

impl AgentFinishEvent {
    /// Creates a new `AgentFinishEvent`.
    ///
    /// # Arguments
    ///
    /// * `steps` - All generation steps
    /// * `total_usage` - Total token usage across all steps
    ///
    /// # Panics
    ///
    /// Panics if `steps` is empty.
    pub fn new(steps: Vec<StepResult>, total_usage: LanguageModelUsage) -> Self {
        let step = steps.last().expect("steps cannot be empty").clone();

        Self {
            step,
            steps,
            total_usage,
        }
    }

    /// Creates a new `AgentFinishEvent` from components.
    pub fn from_components(
        step: StepResult,
        steps: Vec<StepResult>,
        total_usage: LanguageModelUsage,
    ) -> Self {
        Self {
            step,
            steps,
            total_usage,
        }
    }
}

/// Callback that is set using the `onFinish` option for Agent.
///
/// The callback receives an event containing the final step result,
/// all intermediate steps, and total usage across all steps.
///
/// # Examples
///
/// ```no_run
/// use ai_sdk_core::agent::AgentOnFinishCallback;
/// use std::sync::Arc;
///
/// let on_finish: AgentOnFinishCallback = Arc::new(|event| {
///     Box::pin(async move {
///         println!("Final text: {}", event.step.text());
///         println!("Total steps: {}", event.steps.len());
///         println!("Total tokens: {:?}", event.total_usage);
///     })
/// });
/// ```
pub type AgentOnFinishCallback =
    Arc<dyn Fn(AgentFinishEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Creates a callback that does nothing.
///
/// This is useful as a default value when no callback is provided.
pub fn noop_on_finish_callback() -> AgentOnFinishCallback {
    Arc::new(|_event| Box::pin(async move {}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate_text::{RequestMetadata, StepResponseMetadata};
    use crate::output::{Output, TextOutput};
    use ai_sdk_provider::language_model::finish_reason::LanguageModelFinishReason;

    fn create_test_step(usage: LanguageModelUsage) -> StepResult {
        StepResult::new(
            vec![Output::Text(TextOutput::new("Hello"))],
            LanguageModelFinishReason::Stop,
            usage,
            None,
            RequestMetadata::default(),
            StepResponseMetadata {
                id: Some("test".to_string()),
                model_id: Some("gpt-4".to_string()),
                timestamp: None,
                body: None,
            },
            None,
        )
    }

    #[test]
    fn test_agent_finish_event_new() {
        let steps = vec![
            create_test_step(LanguageModelUsage::new(10, 20)),
            create_test_step(LanguageModelUsage::new(15, 25)),
        ];
        let total_usage = LanguageModelUsage::new(25, 45);

        let event = AgentFinishEvent::new(steps.clone(), total_usage);

        assert_eq!(event.steps.len(), 2);
        assert_eq!(event.total_usage.input_tokens, 25);
        assert_eq!(event.total_usage.output_tokens, 45);
        assert_eq!(event.step.text(), "Hello");
    }

    #[test]
    fn test_agent_finish_event_from_components() {
        let step = create_test_step(LanguageModelUsage::new(15, 25));
        let steps = vec![
            create_test_step(LanguageModelUsage::new(10, 20)),
            step.clone(),
        ];
        let total_usage = LanguageModelUsage::new(25, 45);

        let event = AgentFinishEvent::from_components(step, steps.clone(), total_usage);

        assert_eq!(event.steps.len(), 2);
        assert_eq!(event.total_usage.input_tokens, 25);
        assert_eq!(event.total_usage.output_tokens, 45);
    }

    #[tokio::test]
    async fn test_noop_on_finish_callback() {
        let callback = noop_on_finish_callback();
        let steps = vec![create_test_step(LanguageModelUsage::new(10, 20))];
        let event = AgentFinishEvent::new(steps, LanguageModelUsage::new(10, 20));

        // Should not panic
        callback(event).await;
    }

    #[tokio::test]
    async fn test_custom_on_finish_callback() {
        use std::sync::atomic::{AtomicBool, Ordering};

        let called = Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();

        let callback: AgentOnFinishCallback = Arc::new(move |event| {
            let called = called_clone.clone();
            Box::pin(async move {
                assert_eq!(event.steps.len(), 2);
                assert_eq!(event.total_usage.input_tokens, 25);
                called.store(true, Ordering::SeqCst);
            })
        });

        let steps = vec![
            create_test_step(LanguageModelUsage::new(10, 20)),
            create_test_step(LanguageModelUsage::new(15, 25)),
        ];
        let event = AgentFinishEvent::new(steps, LanguageModelUsage::new(25, 45));

        callback(event).await;

        assert!(called.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_callback_with_step_access() {
        let callback: AgentOnFinishCallback = Arc::new(|event| {
            Box::pin(async move {
                // Access final step
                assert_eq!(event.step.text(), "Hello");
                assert_eq!(event.step.usage.input_tokens, 15);

                // Access all steps
                assert_eq!(event.steps.len(), 2);

                // Access total usage
                assert_eq!(event.total_usage.input_tokens, 25);
            })
        });

        let steps = vec![
            create_test_step(LanguageModelUsage::new(10, 20)),
            create_test_step(LanguageModelUsage::new(15, 25)),
        ];
        let event = AgentFinishEvent::new(steps, LanguageModelUsage::new(25, 45));

        callback(event).await;
    }

    #[test]
    #[should_panic(expected = "steps cannot be empty")]
    fn test_agent_finish_event_empty_steps() {
        let steps: Vec<StepResult> = vec![];
        let _event = AgentFinishEvent::new(steps, LanguageModelUsage::default());
    }

    #[test]
    fn test_event_clone() {
        let steps = vec![create_test_step(LanguageModelUsage::new(10, 20))];
        let event = AgentFinishEvent::new(steps, LanguageModelUsage::new(10, 20));

        let cloned = event.clone();

        assert_eq!(event.steps.len(), cloned.steps.len());
        assert_eq!(
            event.total_usage.input_tokens,
            cloned.total_usage.input_tokens
        );
        assert_eq!(event.step.text(), cloned.step.text());
    }
}
