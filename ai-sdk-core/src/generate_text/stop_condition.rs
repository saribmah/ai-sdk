use super::step_result::StepResult;
use async_trait::async_trait;
use std::future::Future;

/// A condition that determines when to stop the generation process.
///
/// Stop conditions are evaluated after each generation step and can be used
/// to implement custom stopping logic beyond the model's finish reason.
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::StopCondition;
///
/// struct CustomCondition;
///
/// #[async_trait]
/// impl StopCondition for CustomCondition {
///     async fn check(&self, steps: &[StepResult]) -> bool {
///         // Custom logic here
///         steps.len() >= 5
///     }
/// }
/// ```
#[async_trait]
pub trait StopCondition: Send + Sync {
    /// Checks if the stop condition is met given the current steps.
    ///
    /// # Arguments
    ///
    /// * `steps` - The steps that have been completed so far
    ///
    /// # Returns
    ///
    /// `true` if the condition is met and generation should stop, `false` otherwise
    async fn check(&self, steps: &[StepResult]) -> bool;
}

/// A stop condition that checks if the step count equals a specific number.
///
/// # Example
///
/// ```
/// use ai_sdk_core::step_count_is;
///
/// // Stop after exactly 3 steps
/// let condition = step_count_is(3);
/// ```
#[derive(Debug, Clone)]
pub struct StepCountIs {
    step_count: usize,
}

#[async_trait]
impl StopCondition for StepCountIs {
    async fn check(&self, steps: &[StepResult]) -> bool {
        steps.len() == self.step_count
    }
}

/// Creates a stop condition that checks if the step count equals a specific number.
///
/// # Arguments
///
/// * `step_count` - The target step count
///
/// # Returns
///
/// A `StepCountIs` condition that can be used to stop generation
///
/// # Example
///
/// ```
/// use ai_sdk_core::step_count_is;
///
/// let condition = step_count_is(5);
/// // This will return true when exactly 5 steps have been completed
/// ```
pub fn step_count_is(step_count: usize) -> StepCountIs {
    StepCountIs { step_count }
}

/// A stop condition that checks if the last step contains a tool call with a specific name.
///
/// # Example
///
/// ```
/// use ai_sdk_core::has_tool_call;
///
/// // Stop when the model calls the "final_answer" tool
/// let condition = has_tool_call("final_answer");
/// ```
#[derive(Debug, Clone)]
pub struct HasToolCall {
    tool_name: String,
}

#[async_trait]
impl StopCondition for HasToolCall {
    async fn check(&self, steps: &[StepResult]) -> bool {
        if let Some(last_step) = steps.last() {
            last_step.tool_calls().iter().any(|tc| {
                use super::TypedToolCall;
                match tc {
                    TypedToolCall::Static(call) => call.tool_name == self.tool_name,
                    TypedToolCall::Dynamic(call) => call.tool_name == self.tool_name,
                }
            })
        } else {
            false
        }
    }
}

/// Creates a stop condition that checks if the last step has a tool call with a specific name.
///
/// # Arguments
///
/// * `tool_name` - The name of the tool to check for
///
/// # Returns
///
/// A `HasToolCall` condition that can be used to stop generation
///
/// # Example
///
/// ```
/// use ai_sdk_core::has_tool_call;
///
/// let condition = has_tool_call("get_final_answer");
/// // This will return true when the last step contains a tool call named "get_final_answer"
/// ```
pub fn has_tool_call(tool_name: impl Into<String>) -> HasToolCall {
    HasToolCall {
        tool_name: tool_name.into(),
    }
}

/// Checks if any of the provided stop conditions are met.
///
/// This function evaluates all stop conditions in parallel and returns `true`
/// if any of them are satisfied.
///
/// # Arguments
///
/// * `stop_conditions` - A slice of stop conditions to evaluate
/// * `steps` - The steps that have been completed so far
///
/// # Returns
///
/// `true` if any stop condition is met, `false` otherwise
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::{is_stop_condition_met, step_count_is, has_tool_call};
///
/// let conditions: Vec<Box<dyn StopCondition>> = vec![
///     Box::new(step_count_is(10)),
///     Box::new(has_tool_call("final_answer")),
/// ];
///
/// let should_stop = is_stop_condition_met(&conditions, &steps).await;
/// ```
pub async fn is_stop_condition_met(
    stop_conditions: &[Box<dyn StopCondition>],
    steps: &[StepResult],
) -> bool {
    // Evaluate all conditions concurrently
    let futures: Vec<_> = stop_conditions
        .iter()
        .map(|condition| condition.check(steps))
        .collect();

    // Wait for all results
    let results = futures::future::join_all(futures).await;

    // Return true if any condition is met
    results.into_iter().any(|result| result)
}

// Allow implementing StopCondition for function types
#[async_trait]
impl<F, Fut> StopCondition for F
where
    F: Fn(&[StepResult]) -> Fut + Send + Sync,
    Fut: Future<Output = bool> + Send,
{
    async fn check(&self, steps: &[StepResult]) -> bool {
        self(steps).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ai_sdk_provider::language_model::{
        content::Content, content::text::Text, content::tool_call::ToolCall,
        finish_reason::FinishReason, usage::Usage,
    };

    fn create_test_step(tool_calls: Vec<(&str, &str)>) -> StepResult {
        use super::super::content_part::ContentPart;
        use super::super::text_output::TextOutput;
        use super::super::tool_call::{DynamicToolCall, TypedToolCall};
        use serde_json::json;

        let mut content: Vec<ContentPart> =
            vec![ContentPart::Text(TextOutput::new("Test".to_string()))];

        for (id, name) in tool_calls {
            content.push(ContentPart::ToolCall(TypedToolCall::Dynamic(
                DynamicToolCall::new(id.to_string(), name.to_string(), json!({})),
            )));
        }

        StepResult::new(
            content,
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
    async fn test_step_count_is() {
        let condition = step_count_is(3);

        let steps = vec![
            create_test_step(vec![]),
            create_test_step(vec![]),
            create_test_step(vec![]),
        ];

        assert!(condition.check(&steps).await);
        assert!(!condition.check(&steps[..2]).await);
        assert!(!condition.check(&[]).await);
    }

    #[tokio::test]
    async fn test_has_tool_call() {
        let condition = has_tool_call("get_weather");

        let steps = vec![
            create_test_step(vec![("call_1", "other_tool")]),
            create_test_step(vec![("call_2", "get_weather")]),
        ];

        assert!(condition.check(&steps).await);
        assert!(!condition.check(&steps[..1]).await);
        assert!(!condition.check(&[]).await);
    }

    #[tokio::test]
    async fn test_has_tool_call_multiple_calls_in_step() {
        let condition = has_tool_call("target_tool");

        let steps = vec![create_test_step(vec![
            ("call_1", "tool_a"),
            ("call_2", "target_tool"),
            ("call_3", "tool_b"),
        ])];

        assert!(condition.check(&steps).await);
    }

    #[tokio::test]
    async fn test_has_tool_call_not_found() {
        let condition = has_tool_call("missing_tool");

        let steps = vec![
            create_test_step(vec![("call_1", "tool_a")]),
            create_test_step(vec![("call_2", "tool_b")]),
        ];

        assert!(!condition.check(&steps).await);
    }

    #[tokio::test]
    async fn test_is_stop_condition_met_none_met() {
        let conditions: Vec<Box<dyn StopCondition>> = vec![
            Box::new(step_count_is(5)),
            Box::new(has_tool_call("missing")),
        ];

        let steps = vec![
            create_test_step(vec![]),
            create_test_step(vec![("call_1", "other")]),
        ];

        assert!(!is_stop_condition_met(&conditions, &steps).await);
    }

    #[tokio::test]
    async fn test_is_stop_condition_met_one_met() {
        let conditions: Vec<Box<dyn StopCondition>> = vec![
            Box::new(step_count_is(2)),
            Box::new(has_tool_call("missing")),
        ];

        let steps = vec![create_test_step(vec![]), create_test_step(vec![])];

        assert!(is_stop_condition_met(&conditions, &steps).await);
    }

    #[tokio::test]
    async fn test_is_stop_condition_met_multiple_met() {
        let conditions: Vec<Box<dyn StopCondition>> = vec![
            Box::new(step_count_is(2)),
            Box::new(has_tool_call("get_weather")),
        ];

        let steps = vec![
            create_test_step(vec![]),
            create_test_step(vec![("call_1", "get_weather")]),
        ];

        assert!(is_stop_condition_met(&conditions, &steps).await);
    }

    #[tokio::test]
    async fn test_is_stop_condition_met_empty_conditions() {
        let conditions: Vec<Box<dyn StopCondition>> = vec![];
        let steps = vec![create_test_step(vec![])];

        assert!(!is_stop_condition_met(&conditions, &steps).await);
    }

    #[tokio::test]
    async fn test_custom_stop_condition_trait() {
        struct AlwaysStop;

        #[async_trait]
        impl StopCondition for AlwaysStop {
            async fn check(&self, _steps: &[StepResult]) -> bool {
                true
            }
        }

        let condition = AlwaysStop;
        let steps = vec![create_test_step(vec![])];

        assert!(condition.check(&steps).await);
    }

    #[tokio::test]
    async fn test_step_count_is_zero() {
        let condition = step_count_is(0);
        assert!(condition.check(&[]).await);
        assert!(!condition.check(&[create_test_step(vec![])]).await);
    }
}
