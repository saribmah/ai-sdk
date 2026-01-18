use crate::generate_text::StepResult;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Callback that is set using the `onStepFinish` option for Agent.
///
/// The callback receives the result of each step as it completes. This is useful
/// for tracking progress, logging intermediate results, or handling step-by-step
/// processing in multi-step agent executions.
///
/// # Examples
///
/// ```no_run
/// use llm_kit_core::agent::AgentOnStepFinishCallback;
/// use std::sync::Arc;
///
/// let on_step_finish: AgentOnStepFinishCallback = Arc::new(|step_result| {
///     Box::pin(async move {
///         println!("Step completed: {}", step_result.text());
///         println!("Step usage: {:?}", step_result.usage);
///         println!("Tool calls in this step: {}", step_result.tool_calls().len());
///     })
/// });
/// ```
pub type AgentOnStepFinishCallback =
    Arc<dyn Fn(StepResult) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Creates a callback that does nothing.
///
/// This is useful as a default value when no callback is provided.
///
/// # Examples
///
/// ```no_run
/// use llm_kit_core::agent::noop_agent_on_step_finish_callback;
///
/// let callback = noop_agent_on_step_finish_callback();
/// // The callback can be safely called but will do nothing
/// ```
pub fn noop_on_step_finish_callback() -> AgentOnStepFinishCallback {
    Arc::new(|_step_result| Box::pin(async move {}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate_text::{RequestMetadata, StepResponseMetadata};
    use crate::output::{Output, TextOutput};
    use llm_kit_provider::language_model::finish_reason::LanguageModelFinishReason;
    use llm_kit_provider::language_model::usage::LanguageModelUsage;
    use llm_kit_provider_utils::tool::{ToolCall, ToolResult};
    use serde_json::json;

    fn create_test_step(text: &str, usage: LanguageModelUsage) -> StepResult {
        StepResult::new(
            vec![Output::Text(TextOutput::new(text))],
            LanguageModelFinishReason::Stop,
            usage,
            None,
            RequestMetadata::default(),
            StepResponseMetadata {
                id: Some("test_step".to_string()),
                model_id: Some("gpt-4".to_string()),
                timestamp: None,
                body: None,
            },
            None,
        )
    }

    fn create_step_with_tools() -> StepResult {
        let content = vec![
            Output::Text(TextOutput::new("I'll check the weather")),
            Output::ToolCall(ToolCall::new(
                "call_1".to_string(),
                "get_weather".to_string(),
                json!({"city": "San Francisco"}),
            )),
            Output::ToolResult(ToolResult::new(
                "call_1".to_string(),
                "get_weather".to_string(),
                json!({"city": "San Francisco"}),
                json!({"temperature": 72, "condition": "sunny"}),
            )),
        ];

        StepResult::new(
            content,
            LanguageModelFinishReason::ToolCalls,
            LanguageModelUsage::new(20, 30),
            None,
            RequestMetadata::default(),
            StepResponseMetadata {
                id: Some("step_with_tools".to_string()),
                model_id: Some("gpt-4".to_string()),
                timestamp: None,
                body: None,
            },
            None,
        )
    }

    #[tokio::test]
    async fn test_noop_on_step_finish_callback() {
        let callback = noop_on_step_finish_callback();
        let step = create_test_step("Hello", LanguageModelUsage::new(10, 20));

        // Should not panic
        callback(step).await;
    }

    #[tokio::test]
    async fn test_custom_on_step_finish_callback() {
        use std::sync::atomic::{AtomicBool, Ordering};

        let called = Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();

        let callback: AgentOnStepFinishCallback = Arc::new(move |step_result| {
            let called = called_clone.clone();
            Box::pin(async move {
                assert_eq!(step_result.text(), "Test step");
                assert_eq!(step_result.usage.input_tokens, 15);
                assert_eq!(step_result.usage.output_tokens, 25);
                called.store(true, Ordering::SeqCst);
            })
        });

        let step = create_test_step("Test step", LanguageModelUsage::new(15, 25));
        callback(step).await;

        assert!(called.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_callback_with_step_properties() {
        let callback: AgentOnStepFinishCallback = Arc::new(|step_result| {
            Box::pin(async move {
                // Access step text
                assert_eq!(step_result.text(), "Hello world");

                // Access step usage
                assert_eq!(step_result.usage.input_tokens, 10);
                assert_eq!(step_result.usage.output_tokens, 20);

                // Access finish reason
                assert_eq!(step_result.finish_reason, LanguageModelFinishReason::Stop);

                // Access response metadata
                assert_eq!(step_result.response.id, Some("test_step".to_string()));
                assert_eq!(step_result.response.model_id, Some("gpt-4".to_string()));
            })
        });

        let step = create_test_step("Hello world", LanguageModelUsage::new(10, 20));
        callback(step).await;
    }

    #[tokio::test]
    async fn test_callback_with_tool_calls() {
        let callback: AgentOnStepFinishCallback = Arc::new(|step_result| {
            Box::pin(async move {
                // Access tool calls
                let tool_calls = step_result.tool_calls();
                assert_eq!(tool_calls.len(), 1);
                assert_eq!(tool_calls[0].tool_name, "get_weather");
                assert_eq!(tool_calls[0].tool_call_id, "call_1");

                // Access tool results
                let tool_results = step_result.tool_results();
                assert_eq!(tool_results.len(), 1);
                assert_eq!(tool_results[0].tool_name, "get_weather");

                // Check finish reason
                assert_eq!(
                    step_result.finish_reason,
                    LanguageModelFinishReason::ToolCalls
                );
            })
        });

        let step = create_step_with_tools();
        callback(step).await;
    }

    #[tokio::test]
    async fn test_callback_invoked_multiple_times() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let callback: AgentOnStepFinishCallback = Arc::new(move |_step_result| {
            let counter = counter_clone.clone();
            Box::pin(async move {
                counter.fetch_add(1, Ordering::SeqCst);
            })
        });

        // Simulate multiple step finishes
        for i in 0..3 {
            let step = create_test_step(
                &format!("Step {}", i),
                LanguageModelUsage::new(10 + i as u64, 20 + i as u64),
            );
            callback(step).await;
        }

        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_callback_with_async_operations() {
        use std::sync::atomic::{AtomicBool, Ordering};

        let processed = Arc::new(AtomicBool::new(false));
        let processed_clone = processed.clone();

        let callback: AgentOnStepFinishCallback = Arc::new(move |step_result| {
            let processed = processed_clone.clone();
            Box::pin(async move {
                // Simulate async processing
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

                // Process the step
                let _text = step_result.text();
                processed.store(true, Ordering::SeqCst);
            })
        });

        let step = create_test_step("Async test", LanguageModelUsage::new(5, 10));
        callback(step).await;

        assert!(processed.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_callback_can_be_cloned() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let callback: AgentOnStepFinishCallback = Arc::new(move |_step_result| {
            let counter = counter_clone.clone();
            Box::pin(async move {
                counter.fetch_add(1, Ordering::SeqCst);
            })
        });

        // Clone the callback
        let callback_clone = callback.clone();

        // Call both callbacks
        let step1 = create_test_step("First", LanguageModelUsage::new(10, 20));
        let step2 = create_test_step("Second", LanguageModelUsage::new(15, 25));

        callback(step1).await;
        callback_clone(step2).await;

        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn test_callback_accessing_content() {
        let callback: AgentOnStepFinishCallback = Arc::new(|step_result| {
            Box::pin(async move {
                // Access content
                let content = &step_result.content;
                assert_eq!(content.len(), 1);

                match &content[0] {
                    Output::Text(text_output) => {
                        assert_eq!(text_output.text, "Content test");
                    }
                    _ => panic!("Expected text output"),
                }
            })
        });

        let step = create_test_step("Content test", LanguageModelUsage::new(10, 20));
        callback(step).await;
    }
}
