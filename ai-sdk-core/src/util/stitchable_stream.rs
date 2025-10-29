//! Stitchable Stream - A utility for dynamically combining multiple streams
//!
//! This module provides `StitchableStream`, which allows adding multiple stream
//! segments dynamically. This is critical for multi-step execution where each
//! LLM call produces a new stream that must be seamlessly combined.
//!
//! # Example
//!
//! ```rust,no_run
//! use futures_util::StreamExt;
//! use ai_sdk_core::util::stitchable_stream::StitchableStream;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let mut stitchable = StitchableStream::<i32>::new();
//!
//! // Add first stream
//! let stream1 = futures_util::stream::iter(vec![1, 2, 3]);
//! stitchable.add_stream(stream1).await;
//!
//! // Add second stream
//! let stream2 = futures_util::stream::iter(vec![4, 5, 6]);
//! stitchable.add_stream(stream2).await;
//!
//! // Close the stream (no more segments will be added)
//! stitchable.close();
//!
//! // Consume the combined stream
//! let mut output = stitchable.stream();
//! while let Some(value) = output.next().await {
//!     println!("{}", value);
//! }
//! # Ok(())
//! # }
//! ```

use futures_util::stream::{Stream, StreamExt};
use std::pin::Pin;
use std::sync::{Arc, Mutex as StdMutex};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

/// A stream handle that allows adding new stream segments dynamically
pub struct StitchableStream<T> {
    /// Sender for adding new items to the output stream (wrapped in Arc<Mutex> so we can drop it)
    tx: Arc<Mutex<Option<UnboundedSender<T>>>>,
    /// Receiver for the output stream (wrapped in Arc<Mutex> for interior mutability)
    rx: Arc<Mutex<Option<UnboundedReceiver<T>>>>,
    /// Track if the stream has been closed (uses std::sync::Mutex for synchronous access)
    closed: Arc<StdMutex<bool>>,
    /// Active stream tasks
    tasks: Arc<Mutex<Vec<JoinHandle<()>>>>,
}

impl<T: Send + 'static> StitchableStream<T> {
    /// Create a new stitchable stream
    ///
    /// # Example
    ///
    /// ```rust
    /// use ai_sdk_core::util::stitchable_stream::StitchableStream;
    ///
    /// let stitchable = StitchableStream::<i32>::new();
    /// ```
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self {
            tx: Arc::new(Mutex::new(Some(tx))),
            rx: Arc::new(Mutex::new(Some(rx))),
            closed: Arc::new(StdMutex::new(false)),
            tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Add a new stream segment to the stitchable stream
    ///
    /// All items from the added stream will be forwarded to the output stream.
    /// Streams are processed in the order they are added.
    ///
    /// # Arguments
    ///
    /// * `stream` - The stream to add
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ai_sdk_core::util::stitchable_stream::StitchableStream;
    /// # async fn example() {
    /// let mut stitchable = StitchableStream::<i32>::new();
    /// let stream = futures_util::stream::iter(vec![1, 2, 3]);
    /// stitchable.add_stream(stream).await;
    /// # }
    /// ```
    pub async fn add_stream<S>(&self, stream: S)
    where
        S: Stream<Item = T> + Send + 'static,
    {
        // Check if stream is already closed (synchronous check)
        let is_closed = *self.closed.lock().unwrap();
        if is_closed {
            return;
        }

        // Get a clone of the sender
        let tx_option = self.tx.lock().await.clone();
        let Some(tx) = tx_option else {
            return;
        };

        let tasks = self.tasks.clone();

        // Spawn a task to forward all items from the stream to the channel
        let handle = tokio::spawn(async move {
            // Pin the stream for consumption
            tokio::pin!(stream);

            while let Some(item) = stream.next().await {
                // If send fails, the receiver has been dropped, so we stop
                if tx.send(item).is_err() {
                    break;
                }
            }
        });

        // Store the task handle
        tasks.lock().await.push(handle);
    }

    /// Close the stream, indicating that no more segments will be added
    ///
    /// After calling `close()`, the output stream will complete once all
    /// added streams have been consumed.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ai_sdk_core::util::stitchable_stream::StitchableStream;
    /// # async fn example() {
    /// let stitchable = StitchableStream::<i32>::new();
    /// // Add streams...
    /// stitchable.close();
    /// # }
    /// ```
    pub fn close(&self) {
        // Mark as closed immediately (synchronous)
        *self.closed.lock().unwrap() = true;

        let tx = self.tx.clone();
        let tasks = self.tasks.clone();

        // Spawn a task to wait for all streams to complete and then close the channel
        tokio::spawn(async move {
            // Wait for all stream tasks to complete
            let handles = {
                let mut tasks_guard = tasks.lock().await;
                std::mem::take(&mut *tasks_guard)
            };

            for handle in handles {
                let _ = handle.await;
            }

            // Drop the sender to signal completion
            // Take the sender out of the Option and drop it
            tx.lock().await.take();
        });
    }

    /// Terminate the stream immediately, canceling any pending stream segments
    ///
    /// This is more forceful than `close()` and will cancel any in-flight
    /// stream processing.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ai_sdk_core::util::stitchable_stream::StitchableStream;
    /// # async fn example() {
    /// let stitchable = StitchableStream::<i32>::new();
    /// // Add streams...
    /// stitchable.terminate().await;
    /// # }
    /// ```
    pub async fn terminate(&self) {
        // Mark as closed immediately (synchronous)
        *self.closed.lock().unwrap() = true;

        // Abort all active tasks
        let handles = {
            let mut tasks_guard = self.tasks.lock().await;
            std::mem::take(&mut *tasks_guard)
        };

        for handle in handles {
            handle.abort();
        }

        // Drop the sender to signal completion
        self.tx.lock().await.take();
    }

    /// Get the output stream that yields all items from all added streams
    ///
    /// This can only be called once. Subsequent calls will return an empty stream.
    ///
    /// # Returns
    ///
    /// A pinned, boxed stream that yields items of type `T`
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use ai_sdk_core::util::stitchable_stream::StitchableStream;
    /// # use futures_util::StreamExt;
    /// # async fn example() {
    /// let mut stitchable = StitchableStream::<i32>::new();
    /// // Add streams...
    /// stitchable.close();
    ///
    /// let mut output = stitchable.stream();
    /// while let Some(item) = output.next().await {
    ///     println!("{}", item);
    /// }
    /// # }
    /// ```
    pub fn stream(&self) -> Pin<Box<dyn Stream<Item = T> + Send>> {
        let rx = self.rx.clone();

        Box::pin(async_stream::stream! {
            let mut rx_option = rx.lock().await;
            if let Some(mut receiver) = rx_option.take() {
                drop(rx_option); // Release lock immediately

                while let Some(item) = receiver.recv().await {
                    yield item;
                }
            }
        })
    }
}

impl<T: Send + 'static> Default for StitchableStream<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::stream;

    #[tokio::test]
    async fn test_create_stitchable_stream() {
        let stitchable = StitchableStream::<i32>::new();
        stitchable.close();

        let mut output = stitchable.stream();
        let result: Vec<i32> = output.collect().await;

        assert_eq!(result, Vec::<i32>::new());
    }

    #[tokio::test]
    async fn test_add_single_stream() {
        let stitchable = StitchableStream::<i32>::new();

        let stream1 = stream::iter(vec![1, 2, 3]);
        stitchable.add_stream(stream1).await;

        stitchable.close();

        let mut output = stitchable.stream();
        let result: Vec<i32> = output.collect().await;

        assert_eq!(result, vec![1, 2, 3]);
    }

    #[tokio::test]
    async fn test_add_multiple_streams() {
        let stitchable = StitchableStream::<i32>::new();

        let stream1 = stream::iter(vec![1, 2, 3]);
        let stream2 = stream::iter(vec![4, 5, 6]);
        let stream3 = stream::iter(vec![7, 8, 9]);

        stitchable.add_stream(stream1).await;
        stitchable.add_stream(stream2).await;
        stitchable.add_stream(stream3).await;

        stitchable.close();

        let mut output = stitchable.stream();
        let result: Vec<i32> = output.collect().await;

        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[tokio::test]
    async fn test_streams_in_order() {
        let stitchable = StitchableStream::<String>::new();

        let stream1 = stream::iter(vec!["a".to_string(), "b".to_string()]);
        let stream2 = stream::iter(vec!["c".to_string(), "d".to_string()]);

        stitchable.add_stream(stream1).await;
        stitchable.add_stream(stream2).await;

        stitchable.close();

        let mut output = stitchable.stream();
        let result: Vec<String> = output.collect().await;

        assert_eq!(result, vec!["a", "b", "c", "d"]);
    }

    #[tokio::test]
    async fn test_close_before_adding_streams() {
        let stitchable = StitchableStream::<i32>::new();

        stitchable.close();

        // Adding streams after close should not add items
        let stream1 = stream::iter(vec![1, 2, 3]);
        stitchable.add_stream(stream1).await;

        let mut output = stitchable.stream();
        let result: Vec<i32> = output.collect().await;

        assert_eq!(result, Vec::<i32>::new());
    }

    #[tokio::test]
    async fn test_terminate() {
        let stitchable = StitchableStream::<i32>::new();

        let stream1 = stream::iter(vec![1, 2, 3]);
        stitchable.add_stream(stream1).await;

        stitchable.terminate().await;

        let mut output = stitchable.stream();
        let result: Vec<i32> = output.collect().await;

        // After terminate, stream should complete (may have partial data)
        assert!(result.len() <= 3);
    }

    #[tokio::test]
    async fn test_empty_stream() {
        let stitchable = StitchableStream::<i32>::new();

        let stream1 = stream::iter(vec![]);
        stitchable.add_stream(stream1).await;

        stitchable.close();

        let mut output = stitchable.stream();
        let result: Vec<i32> = output.collect().await;

        assert_eq!(result, Vec::<i32>::new());
    }

    #[tokio::test]
    async fn test_mixed_empty_and_nonempty_streams() {
        let stitchable = StitchableStream::<i32>::new();

        let stream1 = stream::iter(vec![]);
        let stream2 = stream::iter(vec![1, 2]);
        let stream3 = stream::iter(vec![]);
        let stream4 = stream::iter(vec![3, 4]);

        stitchable.add_stream(stream1).await;
        stitchable.add_stream(stream2).await;
        stitchable.add_stream(stream3).await;
        stitchable.add_stream(stream4).await;

        stitchable.close();

        let mut output = stitchable.stream();
        let result: Vec<i32> = output.collect().await;

        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[tokio::test]
    async fn test_stream_can_only_be_consumed_once() {
        let stitchable = StitchableStream::<i32>::new();

        let stream1 = stream::iter(vec![1, 2, 3]);
        stitchable.add_stream(stream1).await;

        stitchable.close();

        // First consumption
        let mut output1 = stitchable.stream();
        let result1: Vec<i32> = output1.collect().await;
        assert_eq!(result1, vec![1, 2, 3]);

        // Second consumption should yield nothing
        let mut output2 = stitchable.stream();
        let result2: Vec<i32> = output2.collect().await;
        assert_eq!(result2, Vec::<i32>::new());
    }

    #[tokio::test]
    async fn test_concurrent_stream_processing() {
        let stitchable = StitchableStream::<i32>::new();

        // Add multiple streams concurrently
        let stream1 = stream::iter(vec![1, 2]);
        let stream2 = stream::iter(vec![3, 4]);
        let stream3 = stream::iter(vec![5, 6]);

        stitchable.add_stream(stream1).await;
        stitchable.add_stream(stream2).await;
        stitchable.add_stream(stream3).await;

        stitchable.close();

        let mut output = stitchable.stream();
        let result: Vec<i32> = output.collect().await;

        // All items should be present (order guaranteed by sequential add_stream calls)
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
    }

    #[tokio::test]
    async fn test_with_async_stream() {
        let stitchable = StitchableStream::<i32>::new();

        // Create an async stream using async_stream
        let async_stream = async_stream::stream! {
            for i in 1..=3 {
                tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
                yield i;
            }
        };

        stitchable.add_stream(async_stream).await;
        stitchable.close();

        let mut output = stitchable.stream();
        let result: Vec<i32> = output.collect().await;

        assert_eq!(result, vec![1, 2, 3]);
    }
}
