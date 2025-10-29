use crate::stream_text::TextStreamPart;
use serde_json::Value;

/// An enriched stream part that combines a text stream part with an optional partial output.
///
/// This structure is used during streaming to track both the individual stream parts
/// (text deltas, tool calls, etc.) and the incrementally parsed partial output.
///
/// # Type Parameters
///
/// * `INPUT` - The input type for tools (defaults to `Value`)
/// * `OUTPUT` - The output type from tools and partial outputs (defaults to `Value`)
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::stream_text::{EnrichedStreamPart, TextStreamPart};
/// use serde_json::Value;
///
/// let enriched = EnrichedStreamPart {
///     part: TextStreamPart::TextDelta {
///         id: "text1".to_string(),
///         provider_metadata: None,
///         text: "Hello".to_string(),
///     },
///     partial_output: Some(Value::String("Hello".to_string())),
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct EnrichedStreamPart<INPUT = Value, OUTPUT = Value> {
    /// The underlying text stream part.
    pub part: TextStreamPart<INPUT, OUTPUT>,

    /// The incrementally parsed partial output, if available.
    ///
    /// This field is populated when the output specification is used and
    /// contains the partially parsed output as streaming progresses.
    /// It will be `None` if no output specification is provided or if
    /// the current part cannot contribute to partial output parsing.
    pub partial_output: Option<OUTPUT>,
}

impl<INPUT, OUTPUT> EnrichedStreamPart<INPUT, OUTPUT> {
    /// Creates a new `EnrichedStreamPart` with the given part and no partial output.
    ///
    /// # Arguments
    ///
    /// * `part` - The text stream part
    ///
    /// # Example
    ///
    /// ```ignore
    /// use ai_sdk_core::stream_text::{EnrichedStreamPart, TextStreamPart};
    ///
    /// let enriched = EnrichedStreamPart::new(TextStreamPart::TextDelta {
    ///     id: "text1".to_string(),
    ///     provider_metadata: None,
    ///     text: "Hello".to_string(),
    /// });
    /// ```
    pub fn new(part: TextStreamPart<INPUT, OUTPUT>) -> Self {
        Self {
            part,
            partial_output: None,
        }
    }

    /// Creates a new `EnrichedStreamPart` with both a part and partial output.
    ///
    /// # Arguments
    ///
    /// * `part` - The text stream part
    /// * `partial_output` - The partial output value
    ///
    /// # Example
    ///
    /// ```ignore
    /// use ai_sdk_core::stream_text::{EnrichedStreamPart, TextStreamPart};
    /// use serde_json::Value;
    ///
    /// let enriched = EnrichedStreamPart::with_output(
    ///     TextStreamPart::TextDelta {
    ///         id: "text1".to_string(),
    ///         provider_metadata: None,
    ///         text: "Hello".to_string(),
    ///     },
    ///     Value::String("Hello".to_string()),
    /// );
    /// ```
    pub fn with_output(part: TextStreamPart<INPUT, OUTPUT>, partial_output: OUTPUT) -> Self {
        Self {
            part,
            partial_output: Some(partial_output),
        }
    }

    /// Sets the partial output for this enriched stream part.
    ///
    /// # Arguments
    ///
    /// * `partial_output` - The partial output value
    ///
    /// # Example
    ///
    /// ```ignore
    /// use ai_sdk_core::stream_text::{EnrichedStreamPart, TextStreamPart};
    /// use serde_json::Value;
    ///
    /// let mut enriched = EnrichedStreamPart::new(TextStreamPart::TextDelta {
    ///     id: "text1".to_string(),
    ///     provider_metadata: None,
    ///     text: "Hello".to_string(),
    /// });
    /// enriched.set_partial_output(Value::String("Hello".to_string()));
    /// ```
    pub fn set_partial_output(&mut self, partial_output: OUTPUT) {
        self.partial_output = Some(partial_output);
    }

    /// Consumes the enriched stream part and returns the underlying text stream part.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use ai_sdk_core::stream_text::{EnrichedStreamPart, TextStreamPart};
    ///
    /// let enriched = EnrichedStreamPart::new(TextStreamPart::TextDelta {
    ///     id: "text1".to_string(),
    ///     provider_metadata: None,
    ///     text: "Hello".to_string(),
    /// });
    /// let part = enriched.into_part();
    /// ```
    pub fn into_part(self) -> TextStreamPart<INPUT, OUTPUT> {
        self.part
    }

    /// Gets a reference to the underlying text stream part.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use ai_sdk_core::stream_text::{EnrichedStreamPart, TextStreamPart};
    ///
    /// let enriched = EnrichedStreamPart::new(TextStreamPart::TextDelta {
    ///     id: "text1".to_string(),
    ///     provider_metadata: None,
    ///     text: "Hello".to_string(),
    /// });
    /// let part_ref = enriched.part_ref();
    /// ```
    pub fn part_ref(&self) -> &TextStreamPart<INPUT, OUTPUT> {
        &self.part
    }

    /// Gets a reference to the partial output, if available.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use ai_sdk_core::stream_text::{EnrichedStreamPart, TextStreamPart};
    /// use serde_json::Value;
    ///
    /// let enriched = EnrichedStreamPart::with_output(
    ///     TextStreamPart::TextDelta {
    ///         id: "text1".to_string(),
    ///         provider_metadata: None,
    ///         text: "Hello".to_string(),
    ///     },
    ///     Value::String("Hello".to_string()),
    /// );
    /// if let Some(output) = enriched.partial_output_ref() {
    ///     println!("Partial output: {:?}", output);
    /// }
    /// ```
    pub fn partial_output_ref(&self) -> Option<&OUTPUT> {
        self.partial_output.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream_text::TextStreamPart;

    #[test]
    fn test_new_enriched_stream_part() {
        let part: TextStreamPart<Value, Value> = TextStreamPart::TextDelta {
            id: "text1".to_string(),
            provider_metadata: None,
            text: "Hello".to_string(),
        };

        let enriched = EnrichedStreamPart::new(part.clone());

        assert_eq!(enriched.part, part);
        assert_eq!(enriched.partial_output, None);
    }

    #[test]
    fn test_with_output() {
        let part: TextStreamPart<Value, Value> = TextStreamPart::TextDelta {
            id: "text1".to_string(),
            provider_metadata: None,
            text: "Hello".to_string(),
        };
        let output = Value::String("Hello".to_string());

        let enriched = EnrichedStreamPart::with_output(part.clone(), output.clone());

        assert_eq!(enriched.part, part);
        assert_eq!(enriched.partial_output, Some(output));
    }

    #[test]
    fn test_set_partial_output() {
        let part: TextStreamPart<Value, Value> = TextStreamPart::TextDelta {
            id: "text1".to_string(),
            provider_metadata: None,
            text: "Hello".to_string(),
        };

        let mut enriched = EnrichedStreamPart::new(part);
        assert_eq!(enriched.partial_output, None);

        let output = Value::String("Hello".to_string());
        enriched.set_partial_output(output.clone());

        assert_eq!(enriched.partial_output, Some(output));
    }

    #[test]
    fn test_into_part() {
        let part: TextStreamPart<Value, Value> = TextStreamPart::TextDelta {
            id: "text1".to_string(),
            provider_metadata: None,
            text: "Hello".to_string(),
        };

        let enriched = EnrichedStreamPart::new(part.clone());
        let extracted_part = enriched.into_part();

        assert_eq!(extracted_part, part);
    }

    #[test]
    fn test_part_ref() {
        let part: TextStreamPart<Value, Value> = TextStreamPart::TextDelta {
            id: "text1".to_string(),
            provider_metadata: None,
            text: "Hello".to_string(),
        };

        let enriched = EnrichedStreamPart::new(part.clone());
        let part_ref = enriched.part_ref();

        assert_eq!(part_ref, &part);
    }

    #[test]
    fn test_partial_output_ref() {
        let part: TextStreamPart<Value, Value> = TextStreamPart::TextDelta {
            id: "text1".to_string(),
            provider_metadata: None,
            text: "Hello".to_string(),
        };
        let output = Value::String("Hello".to_string());

        let enriched = EnrichedStreamPart::with_output(part, output.clone());
        let output_ref = enriched.partial_output_ref();

        assert_eq!(output_ref, Some(&output));
    }

    #[test]
    fn test_partial_output_ref_none() {
        let part: TextStreamPart<Value, Value> = TextStreamPart::TextDelta {
            id: "text1".to_string(),
            provider_metadata: None,
            text: "Hello".to_string(),
        };

        let enriched = EnrichedStreamPart::<Value, Value>::new(part);
        let output_ref = enriched.partial_output_ref();

        assert_eq!(output_ref, None);
    }
}
