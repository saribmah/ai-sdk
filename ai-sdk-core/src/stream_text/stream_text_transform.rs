use crate::generate_text::ToolSet;
use crate::stream_text::TextStreamPart;
use futures_util::stream::Stream;
use serde_json::Value;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Type alias for async iterable streams.
pub type AsyncIterableStream<T> = Pin<Box<dyn Stream<Item = T> + Send>>;

/// Options passed to a stream text transform.
///
/// These options provide context and control mechanisms for the transformation.
///
/// # Type Parameters
///
/// * `INPUT` - The input type for tools (defaults to `Value`)
/// * `OUTPUT` - The output type from tools (defaults to `Value`)
#[derive(Clone)]
pub struct StreamTextTransformOptions<INPUT = Value, OUTPUT = Value> {
    /// The tools that are accessible to and can be called by the model.
    ///
    /// The model needs to support calling tools. These are provided for type
    /// inference and context in the transformation.
    ///
    /// Wrapped in Arc to allow cloning even though Tool doesn't implement Clone.
    pub tools: Option<Arc<ToolSet>>,

    /// A function that stops the source stream.
    ///
    /// When called, this will signal the upstream stream to stop producing values.
    /// This is useful for implementing early termination or backpressure.
    pub stop_stream: Arc<dyn Fn() + Send + Sync>,

    /// Phantom data to bind the INPUT and OUTPUT types.
    _phantom: PhantomData<(INPUT, OUTPUT)>,
}

impl<INPUT, OUTPUT> StreamTextTransformOptions<INPUT, OUTPUT> {
    /// Creates new transform options.
    ///
    /// # Arguments
    ///
    /// * `tools` - Optional tool set for the transformation
    /// * `stop_stream` - Function to stop the source stream
    ///
    /// # Example
    ///
    /// ```ignore
    /// use ai_sdk_core::stream_text::StreamTextTransformOptions;
    /// use std::sync::Arc;
    ///
    /// let options = StreamTextTransformOptions::new(
    ///     None,
    ///     Arc::new(|| {
    ///         println!("Stopping stream");
    ///     }),
    /// );
    /// ```
    pub fn new(tools: Option<Arc<ToolSet>>, stop_stream: Arc<dyn Fn() + Send + Sync>) -> Self {
        Self {
            tools,
            stop_stream,
            _phantom: PhantomData,
        }
    }
}

/// A transformation function that is applied to a text stream.
///
/// This type represents a function that takes stream transform options and returns
/// a transformer function. The transformer function converts one stream of
/// `TextStreamPart` into another stream of `TextStreamPart`.
///
/// This is the Rust equivalent of TypeScript's `StreamTextTransform` type:
/// ```typescript
/// type StreamTextTransform<TOOLS> = (options: {
///   tools: TOOLS;
///   stopStream: () => void;
/// }) => TransformStream<TextStreamPart<TOOLS>, TextStreamPart<TOOLS>>;
/// ```
///
/// # Type Parameters
///
/// * `INPUT` - The input type for tools (defaults to `Value`)
/// * `OUTPUT` - The output type from tools (defaults to `Value`)
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::stream_text::{StreamTextTransform, StreamTextTransformOptions};
/// use futures_util::StreamExt;
///
/// // Create a simple pass-through transform
/// let transform: StreamTextTransform = Box::new(|options| {
///     Box::new(move |stream| {
///         Box::pin(stream)
///     })
/// });
///
/// // Create a filtering transform
/// let filter_transform: StreamTextTransform = Box::new(|options| {
///     Box::new(move |stream| {
///         Box::pin(stream.filter(|part| async move {
///             // Filter logic here
///             true
///         }))
///     })
/// });
/// ```
pub type StreamTextTransform<INPUT = Value, OUTPUT = Value> = Box<
    dyn Fn(
            StreamTextTransformOptions<INPUT, OUTPUT>,
        ) -> Box<
            dyn Fn(
                    AsyncIterableStream<TextStreamPart<INPUT, OUTPUT>>,
                ) -> AsyncIterableStream<TextStreamPart<INPUT, OUTPUT>>
                + Send
                + Sync,
        > + Send
        + Sync,
>;

/// Creates a simple pass-through transform that doesn't modify the stream.
///
/// This is useful as a default or for testing purposes.
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::stream_text::create_passthrough_transform;
///
/// let transform = create_passthrough_transform();
/// ```
pub fn create_passthrough_transform<INPUT, OUTPUT>() -> StreamTextTransform<INPUT, OUTPUT>
where
    INPUT: Clone + Send + Sync + 'static,
    OUTPUT: Clone + Send + Sync + 'static,
{
    Box::new(|_options| Box::new(|stream| stream))
}

/// Creates a transform that logs each stream part.
///
/// This is useful for debugging streaming behavior.
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::stream_text::create_logging_transform;
///
/// let transform = create_logging_transform();
/// ```
pub fn create_logging_transform<INPUT, OUTPUT>() -> StreamTextTransform<INPUT, OUTPUT>
where
    INPUT: Clone + Send + Sync + std::fmt::Debug + 'static,
    OUTPUT: Clone + Send + Sync + std::fmt::Debug + 'static,
{
    Box::new(|_options| {
        Box::new(|stream| {
            Box::pin(futures_util::stream::unfold(
                (stream, 0usize),
                |(mut stream, count)| async move {
                    use futures_util::StreamExt;
                    match stream.next().await {
                        Some(part) => {
                            eprintln!("[Stream Part #{}] {:?}", count + 1, part);
                            Some((part, (stream, count + 1)))
                        }
                        None => {
                            eprintln!("[Stream Ended] Total parts: {}", count);
                            None
                        }
                    }
                },
            ))
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream_text::TextStreamPart;
    use futures_util::StreamExt;
    use futures_util::stream;

    #[tokio::test]
    async fn test_passthrough_transform() {
        let parts: Vec<TextStreamPart<Value, Value>> = vec![
            TextStreamPart::TextDelta {
                id: "text1".to_string(),
                provider_metadata: None,
                text: "Hello".to_string(),
            },
            TextStreamPart::TextDelta {
                id: "text1".to_string(),
                provider_metadata: None,
                text: " World".to_string(),
            },
        ];

        let input_stream: AsyncIterableStream<TextStreamPart<Value, Value>> =
            Box::pin(stream::iter(parts.clone()));

        let transform = create_passthrough_transform();
        let options = StreamTextTransformOptions::new(None, Arc::new(|| {}));

        let transformer = transform(options);
        let output_stream = transformer(input_stream);

        let output: Vec<_> = output_stream.collect().await;

        assert_eq!(output.len(), 2);
        assert_eq!(output, parts);
    }

    #[tokio::test]
    async fn test_transform_options() {
        let stop_called = Arc::new(Mutex::new(false));
        let stop_called_clone = Arc::clone(&stop_called);

        let options = StreamTextTransformOptions::<Value, Value>::new(
            None,
            Arc::new(move || {
                let stop_called = Arc::clone(&stop_called_clone);
                tokio::spawn(async move {
                    let mut guard = stop_called.lock().await;
                    *guard = true;
                });
            }),
        );

        // Call stop_stream
        (options.stop_stream)();

        // Give it a moment to process
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        let guard = stop_called.lock().await;
        assert!(*guard);
    }

    #[tokio::test]
    async fn test_custom_transform() {
        let parts: Vec<TextStreamPart<Value, Value>> = vec![
            TextStreamPart::TextDelta {
                id: "text1".to_string(),
                provider_metadata: None,
                text: "Hello".to_string(),
            },
            TextStreamPart::TextDelta {
                id: "text2".to_string(),
                provider_metadata: None,
                text: "World".to_string(),
            },
            TextStreamPart::TextDelta {
                id: "text3".to_string(),
                provider_metadata: None,
                text: "Test".to_string(),
            },
        ];

        let input_stream: AsyncIterableStream<TextStreamPart<Value, Value>> =
            Box::pin(stream::iter(parts));

        // Create a transform that filters out parts containing "World"
        let custom_transform: StreamTextTransform<Value, Value> = Box::new(|_options| {
            Box::new(|stream| {
                Box::pin(stream.filter(|part| {
                    let should_keep = match part {
                        TextStreamPart::TextDelta { text, .. } => !text.contains("World"),
                        _ => true,
                    };
                    async move { should_keep }
                }))
            })
        });

        let options = StreamTextTransformOptions::new(None, Arc::new(|| {}));
        let transformer = custom_transform(options);
        let output_stream = transformer(input_stream);

        let output: Vec<_> = output_stream.collect().await;

        // Should only have "Hello" and "Test", not "World"
        assert_eq!(output.len(), 2);
        if let TextStreamPart::TextDelta { text, .. } = &output[0] {
            assert_eq!(text, "Hello");
        }
        if let TextStreamPart::TextDelta { text, .. } = &output[1] {
            assert_eq!(text, "Test");
        }
    }
}
