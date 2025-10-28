use futures_util::Stream;
use super::definition::{ToolExecuteFunction, ToolExecutionOutput};
use super::options::ToolCallOptions;

/// Event emitted during tool execution.
#[derive(Debug, Clone)]
pub enum ToolExecutionEvent<OUTPUT>
where
    OUTPUT: Send + Clone + 'static,
{
    /// Preliminary output from a streaming tool (not the final output).
    Preliminary { output: OUTPUT },

    /// Final output from the tool execution.
    Final { output: OUTPUT },

    /// Error occurred during tool execution.
    Error { error: serde_json::Value },
}

impl<OUTPUT> PartialEq for ToolExecutionEvent<OUTPUT>
where
    OUTPUT: PartialEq + Send + Clone + 'static,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Preliminary { output: a }, Self::Preliminary { output: b }) => a == b,
            (Self::Final { output: a }, Self::Final { output: b }) => a == b,
            (Self::Error { error: a }, Self::Error { error: b }) => a == b,
            _ => false,
        }
    }
}

impl<OUTPUT> Eq for ToolExecutionEvent<OUTPUT>
where
    OUTPUT: Eq + Send + Clone + 'static,
{
}

/// Executes a tool with the given input and options.
///
/// Returns a stream of execution events. For streaming tools, this will yield
/// `Preliminary` events for each intermediate output, followed by a `Final` event
/// with the last output. For non-streaming tools, this yields a single `Final` event.
///
/// # Type Parameters
///
/// * `INPUT` - The type of input the tool accepts
/// * `OUTPUT` - The type of output the tool produces
///
/// # Arguments
///
/// * `execute` - The tool execution function
/// * `input` - The input to pass to the tool
/// * `options` - Additional options for the tool execution
///
/// # Returns
///
/// An async stream that yields `ToolExecutionEvent<OUTPUT>` items.
pub fn execute_tool<INPUT, OUTPUT>(
    execute: ToolExecuteFunction<INPUT, OUTPUT>,
    input: INPUT,
    options: ToolCallOptions,
) -> impl Stream<Item = ToolExecutionEvent<OUTPUT>>
where
    INPUT: Send + 'static,
    OUTPUT: Send + Clone + 'static,
{
    async_stream::stream! {
        use futures_util::StreamExt;

        let result = execute(input, options);

        match result {
            ToolExecutionOutput::Streaming(mut stream) => {
                let mut last_output: Option<OUTPUT> = None;

                // Yield preliminary events for each output
                while let Some(result) = stream.next().await {
                    match result {
                        Ok(output) => {
                            last_output = Some(output.clone());
                            yield ToolExecutionEvent::Preliminary { output };
                        }
                        Err(error) => {
                            // Error in stream - yield error event and stop
                            yield ToolExecutionEvent::Error { error };
                            return;
                        }
                    }
                }

                // Yield final event with the last output
                if let Some(output) = last_output {
                    yield ToolExecutionEvent::Final { output };
                }
            }
            ToolExecutionOutput::Single(future) => {
                // For single outputs, await and yield either final or error event
                match future.await {
                    Ok(output) => {
                        yield ToolExecutionEvent::Final { output };
                    }
                    Err(error) => {
                        yield ToolExecutionEvent::Error { error };
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};
    use std::pin::Pin;
    use futures_util::StreamExt;

    #[tokio::test]
    async fn test_execute_tool_single_output() {
        let execute: ToolExecuteFunction<Value, Value> = Box::new(|input, _options| {
            ToolExecutionOutput::Single(Box::pin(async move {
                Ok(json!({"result": input}))
            }))
        });

        let options = ToolCallOptions::new("call_123", vec![]);
        let mut stream = Box::pin(execute_tool(execute, json!({"city": "SF"}), options));

        // Should get exactly one Final event
        let event = stream.next().await.unwrap();
        assert_eq!(
            event,
            ToolExecutionEvent::Final {
                output: json!({"result": {"city": "SF"}})
            }
        );

        // No more events
        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn test_execute_tool_streaming_output() {
        let execute: ToolExecuteFunction<Value, Value> = Box::new(|_input, _options| {
            ToolExecutionOutput::Streaming(Box::pin(async_stream::stream! {
                yield Ok(json!({"step": 1}));
                yield Ok(json!({"step": 2}));
                yield Ok(json!({"step": 3}));
            }))
        });

        let options = ToolCallOptions::new("call_123", vec![]);
        let mut stream = Box::pin(execute_tool(execute, json!({}), options));

        // Should get 3 Preliminary events
        let event1 = stream.next().await.unwrap();
        assert_eq!(
            event1,
            ToolExecutionEvent::Preliminary {
                output: json!({"step": 1})
            }
        );

        let event2 = stream.next().await.unwrap();
        assert_eq!(
            event2,
            ToolExecutionEvent::Preliminary {
                output: json!({"step": 2})
            }
        );

        let event3 = stream.next().await.unwrap();
        assert_eq!(
            event3,
            ToolExecutionEvent::Preliminary {
                output: json!({"step": 3})
            }
        );

        // Should get 1 Final event with the last output
        let event4 = stream.next().await.unwrap();
        assert_eq!(
            event4,
            ToolExecutionEvent::Final {
                output: json!({"step": 3})
            }
        );

        // No more events
        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn test_execute_tool_empty_stream() {
        let execute: ToolExecuteFunction<Value, Value> = Box::new(|_input, _options| {
            ToolExecutionOutput::Streaming(Box::pin(async_stream::stream! {
                // Empty stream - no yields
                // We need to specify the type since there are no yields
                if false {
                    yield Ok::<Value, Value>(json!(null));
                }
            }))
        });

        let options = ToolCallOptions::new("call_123", vec![]);
        let mut stream = Box::pin(execute_tool(execute, json!({}), options));

        // Should get no events (not even a Final event since there's no output)
        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn test_execute_tool_single_error() {
        let execute: ToolExecuteFunction<Value, Value> = Box::new(|_input, _options| {
            ToolExecutionOutput::Single(Box::pin(async move {
                Err(json!("Tool execution failed"))
            }))
        });

        let options = ToolCallOptions::new("call_123", vec![]);
        let mut stream = Box::pin(execute_tool(execute, json!({}), options));

        // Should get exactly one Error event
        let event = stream.next().await.unwrap();
        assert_eq!(
            event,
            ToolExecutionEvent::Error {
                error: json!("Tool execution failed")
            }
        );

        // No more events
        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn test_execute_tool_streaming_error() {
        let execute: ToolExecuteFunction<Value, Value> = Box::new(|_input, _options| {
            ToolExecutionOutput::Streaming(Box::pin(async_stream::stream! {
                yield Ok(json!({"step": 1}));
                yield Err(json!("Error in stream"));
                yield Ok(json!({"step": 3})); // This should not be reached
            }))
        });

        let options = ToolCallOptions::new("call_123", vec![]);
        let mut stream = Box::pin(execute_tool(execute, json!({}), options));

        // Should get 1 Preliminary event
        let event1 = stream.next().await.unwrap();
        assert_eq!(
            event1,
            ToolExecutionEvent::Preliminary {
                output: json!({"step": 1})
            }
        );

        // Should get Error event and stream should stop
        let event2 = stream.next().await.unwrap();
        assert_eq!(
            event2,
            ToolExecutionEvent::Error {
                error: json!("Error in stream")
            }
        );

        // No more events
        assert!(stream.next().await.is_none());
    }
}
