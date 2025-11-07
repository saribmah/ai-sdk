use crate::stream_text::TextStreamPart;
use futures_util::Stream;
use std::pin::Pin;

/// A stream transformation that can be applied to a TextStreamPart stream.
///
/// Stream transformations allow you to modify, filter, or augment the stream
/// of text parts as they flow through the system. Transformations are composable
/// and can be chained together.
///
/// # Examples
///
/// ```ignore
/// use ai_sdk_core::stream_text::transform::{StreamTransform, filter_transform};
///
/// // Create a transform that only allows text deltas
/// let text_only = filter_transform(|part| {
///     matches!(part, TextStreamPart::TextDelta { .. })
/// });
/// ```
pub trait StreamTransform: Send + Sync {
    /// Transform the input stream into an output stream.
    ///
    /// # Arguments
    ///
    /// * `stream` - The input stream of TextStreamParts
    /// * `options` - Additional options for the transformation
    ///
    /// # Returns
    ///
    /// A new stream with the transformation applied
    fn transform(
        &self,
        stream: Pin<Box<dyn Stream<Item = TextStreamPart> + Send>>,
        options: TransformOptions,
    ) -> Pin<Box<dyn Stream<Item = TextStreamPart> + Send>>;
}

/// Options passed to stream transformations.
#[derive(Clone)]
pub struct TransformOptions {
    /// A function that can be called to stop the stream early
    pub stop_stream: Option<StopStreamHandle>,
}

impl TransformOptions {
    /// Create new transform options
    pub fn new() -> Self {
        Self { stop_stream: None }
    }

    /// Set the stop stream handle
    pub fn with_stop_stream(mut self, handle: StopStreamHandle) -> Self {
        self.stop_stream = Some(handle);
        self
    }
}

impl Default for TransformOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// A handle that can be used to stop a stream early.
#[derive(Clone)]
pub struct StopStreamHandle {
    stop_fn: std::sync::Arc<dyn Fn() + Send + Sync>,
}

impl StopStreamHandle {
    /// Create a new stop stream handle
    pub fn new<F>(stop_fn: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        Self {
            stop_fn: std::sync::Arc::new(stop_fn),
        }
    }

    /// Stop the stream
    pub fn stop(&self) {
        (self.stop_fn)()
    }
}

/// A filter transformation that only passes through items matching a predicate.
pub struct FilterTransform<F>
where
    F: Fn(&TextStreamPart) -> bool + Send + Sync + Clone,
{
    predicate: F,
}

impl<F> FilterTransform<F>
where
    F: Fn(&TextStreamPart) -> bool + Send + Sync + Clone,
{
    /// Create a new filter transformation
    pub fn new(predicate: F) -> Self {
        Self { predicate }
    }
}

impl<F> StreamTransform for FilterTransform<F>
where
    F: Fn(&TextStreamPart) -> bool + Send + Sync + Clone + 'static,
{
    fn transform(
        &self,
        stream: Pin<Box<dyn Stream<Item = TextStreamPart> + Send>>,
        _options: TransformOptions,
    ) -> Pin<Box<dyn Stream<Item = TextStreamPart> + Send>> {
        use futures_util::StreamExt;

        let predicate = self.predicate.clone();
        Box::pin(stream.filter(move |part| {
            let result = predicate(part);
            async move { result }
        }))
    }
}

/// A map transformation that transforms each stream part.
///
/// # Examples
///
/// ```ignore
/// let uppercase_text = map_transform(|part| {
///     match part {
///         TextStreamPart::TextDelta { id, text, provider_metadata } => {
///             TextStreamPart::TextDelta {
///                 id,
///                 text: text.to_uppercase(),
///                 provider_metadata,
///             }
///         }
///         other => other,
///     }
/// });
/// ```
pub struct MapTransform<F>
where
    F: Fn(TextStreamPart) -> TextStreamPart + Send + Sync + Clone,
{
    mapper: F,
}

impl<F> MapTransform<F>
where
    F: Fn(TextStreamPart) -> TextStreamPart + Send + Sync + Clone,
{
    /// Create a new map transformation
    pub fn new(mapper: F) -> Self {
        Self { mapper }
    }
}

impl<F> StreamTransform for MapTransform<F>
where
    F: Fn(TextStreamPart) -> TextStreamPart + Send + Sync + Clone + 'static,
{
    fn transform(
        &self,
        stream: Pin<Box<dyn Stream<Item = TextStreamPart> + Send>>,
        _options: TransformOptions,
    ) -> Pin<Box<dyn Stream<Item = TextStreamPart> + Send>> {
        use futures_util::StreamExt;

        let mapper = self.mapper.clone();
        Box::pin(stream.map(mapper))
    }
}

/// A throttling transformation that limits the rate of stream parts.
///
/// This is useful for rate limiting or for slowing down output for display purposes.
///
/// # Examples
///
/// ```ignore
/// use std::time::Duration;
///
/// // Limit to one item every 100ms
/// let throttled = throttle_transform(Duration::from_millis(100));
/// ```
pub struct ThrottleTransform {
    delay: std::time::Duration,
}

impl ThrottleTransform {
    /// Create a new throttle transformation
    pub fn new(delay: std::time::Duration) -> Self {
        Self { delay }
    }
}

impl StreamTransform for ThrottleTransform {
    fn transform(
        &self,
        stream: Pin<Box<dyn Stream<Item = TextStreamPart> + Send>>,
        _options: TransformOptions,
    ) -> Pin<Box<dyn Stream<Item = TextStreamPart> + Send>> {
        use futures_util::StreamExt;

        let delay = self.delay;
        Box::pin(stream.then(move |part| async move {
            tokio::time::sleep(delay).await;
            part
        }))
    }
}

/// A batching transformation that groups text deltas together.
///
/// This can reduce the number of events emitted by combining multiple
/// small text deltas into larger chunks.
///
/// # Examples
///
/// ```ignore
/// // Batch up to 10 text deltas or flush after 100ms
/// let batched = batch_text_transform(10, Duration::from_millis(100));
/// ```
pub struct BatchTextTransform {
    max_batch_size: usize,
    max_delay: std::time::Duration,
}

impl BatchTextTransform {
    /// Create a new batch text transformation
    pub fn new(max_batch_size: usize, max_delay: std::time::Duration) -> Self {
        Self {
            max_batch_size,
            max_delay,
        }
    }
}

impl StreamTransform for BatchTextTransform {
    fn transform(
        &self,
        stream: Pin<Box<dyn Stream<Item = TextStreamPart> + Send>>,
        _options: TransformOptions,
    ) -> Pin<Box<dyn Stream<Item = TextStreamPart> + Send>> {
        use futures_util::StreamExt;

        let max_batch_size = self.max_batch_size;
        let max_delay = self.max_delay;

        Box::pin(async_stream::stream! {
            let mut stream = stream;
            let mut batch = String::new();
            let mut batch_id = None;
            let mut batch_metadata = None;
            let mut last_emit = tokio::time::Instant::now();

            while let Some(part) = stream.next().await {
                match part {
                    TextStreamPart::TextDelta { id, text, provider_metadata } => {
                        // Initialize batch with first delta's metadata
                        if batch.is_empty() {
                            batch_id = Some(id.clone());
                            batch_metadata = provider_metadata.clone();
                        }

                        batch.push_str(&text);

                        // Flush if we hit the size limit or time limit
                        let should_flush = batch.len() >= max_batch_size
                            || last_emit.elapsed() >= max_delay;

                        if should_flush && !batch.is_empty() {
                            yield TextStreamPart::TextDelta {
                                id: batch_id.clone().unwrap_or_else(|| id.clone()),
                                text: batch.clone(),
                                provider_metadata: batch_metadata.clone(),
                            };
                            batch.clear();
                            batch_id = None;
                            batch_metadata = None;
                            last_emit = tokio::time::Instant::now();
                        }
                    }
                    other => {
                        // Flush any pending batch before emitting non-text part
                        if !batch.is_empty() {
                            yield TextStreamPart::TextDelta {
                                id: batch_id.take().unwrap_or_default(),
                                text: batch.clone(),
                                provider_metadata: batch_metadata.clone(),
                            };
                            batch.clear();
                            batch_metadata = None;
                            last_emit = tokio::time::Instant::now();
                        }
                        yield other;
                    }
                }
            }

            // Flush any remaining batch
            if !batch.is_empty() {
                yield TextStreamPart::TextDelta {
                    id: batch_id.unwrap_or_default(),
                    text: batch,
                    provider_metadata: batch_metadata,
                };
            }
        })
    }
}

/// Helper function to create a filter transformation.
///
/// # Examples
///
/// ```ignore
/// let text_only = filter_transform(|part| {
///     matches!(part, TextStreamPart::TextDelta { .. })
/// });
/// ```
pub fn filter_transform<F>(predicate: F) -> FilterTransform<F>
where
    F: Fn(&TextStreamPart) -> bool + Send + Sync + Clone,
{
    FilterTransform::new(predicate)
}

/// Helper function to create a map transformation.
///
/// # Examples
///
/// ```ignore
/// let uppercase_text = map_transform(|part| {
///     // Transform the part
///     part
/// });
/// ```
pub fn map_transform<F>(mapper: F) -> MapTransform<F>
where
    F: Fn(TextStreamPart) -> TextStreamPart + Send + Sync + Clone,
{
    MapTransform::new(mapper)
}

/// Helper function to create a throttle transformation.
///
/// # Examples
///
/// ```ignore
/// use std::time::Duration;
///
/// let throttled = throttle_transform(Duration::from_millis(100));
/// ```
pub fn throttle_transform(delay: std::time::Duration) -> ThrottleTransform {
    ThrottleTransform::new(delay)
}

/// Helper function to create a batch text transformation.
///
/// # Examples
///
/// ```ignore
/// use std::time::Duration;
///
/// let batched = batch_text_transform(10, Duration::from_millis(100));
/// ```
pub fn batch_text_transform(
    max_batch_size: usize,
    max_delay: std::time::Duration,
) -> BatchTextTransform {
    BatchTextTransform::new(max_batch_size, max_delay)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream_text::TextStreamPart;
    use futures_util::StreamExt;

    #[tokio::test]
    async fn test_filter_transform() {
        let items = vec![
            TextStreamPart::Start,
            TextStreamPart::TextDelta {
                id: "1".to_string(),
                text: "hello".to_string(),
                provider_metadata: None,
            },
            TextStreamPart::TextDelta {
                id: "2".to_string(),
                text: " world".to_string(),
                provider_metadata: None,
            },
            TextStreamPart::Finish {
                finish_reason:
                    ai_sdk_provider::language_model::finish_reason::LanguageModelFinishReason::Stop,
                total_usage: ai_sdk_provider::language_model::usage::LanguageModelUsage::default(),
            },
        ];

        let stream = Box::pin(futures_util::stream::iter(items));
        let filter = filter_transform(|part| matches!(part, TextStreamPart::TextDelta { .. }));

        let options = TransformOptions::new();
        let mut transformed = filter.transform(stream, options);

        let mut count = 0;
        while let Some(part) = transformed.next().await {
            assert!(matches!(part, TextStreamPart::TextDelta { .. }));
            count += 1;
        }
        assert_eq!(count, 2);
    }

    #[tokio::test]
    async fn test_map_transform() {
        let items = vec![TextStreamPart::TextDelta {
            id: "1".to_string(),
            text: "hello".to_string(),
            provider_metadata: None,
        }];

        let stream = Box::pin(futures_util::stream::iter(items));
        let mapper = map_transform(|part| match part {
            TextStreamPart::TextDelta {
                id,
                text,
                provider_metadata,
            } => TextStreamPart::TextDelta {
                id,
                text: text.to_uppercase(),
                provider_metadata,
            },
            other => other,
        });

        let options = TransformOptions::new();
        let mut transformed = mapper.transform(stream, options);

        if let Some(TextStreamPart::TextDelta { text, .. }) = transformed.next().await {
            assert_eq!(text, "HELLO");
        } else {
            panic!("Expected TextDelta");
        }
    }

    #[tokio::test]
    async fn test_batch_text_transform() {
        let items = vec![
            TextStreamPart::TextDelta {
                id: "1".to_string(),
                text: "h".to_string(),
                provider_metadata: None,
            },
            TextStreamPart::TextDelta {
                id: "1".to_string(),
                text: "e".to_string(),
                provider_metadata: None,
            },
            TextStreamPart::TextDelta {
                id: "1".to_string(),
                text: "l".to_string(),
                provider_metadata: None,
            },
            TextStreamPart::TextDelta {
                id: "1".to_string(),
                text: "l".to_string(),
                provider_metadata: None,
            },
            TextStreamPart::TextDelta {
                id: "1".to_string(),
                text: "o".to_string(),
                provider_metadata: None,
            },
        ];

        let stream = Box::pin(futures_util::stream::iter(items));
        let batcher = batch_text_transform(5, std::time::Duration::from_secs(1));

        let options = TransformOptions::new();
        let mut transformed = batcher.transform(stream, options);

        // Should batch all 5 single-character deltas into one
        if let Some(TextStreamPart::TextDelta { text, .. }) = transformed.next().await {
            assert_eq!(text, "hello");
        } else {
            panic!("Expected batched TextDelta");
        }

        // Should be no more items (or maybe just the final flush)
        let remaining: Vec<_> = transformed.collect().await;
        assert!(remaining.is_empty() || remaining.len() <= 1);
    }
}
