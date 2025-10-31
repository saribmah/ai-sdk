use serde::Deserialize;
use serde_json::Value;
use std::marker::PhantomData;

#[cfg(test)]
use serde::Serialize;

/// Trait for defining output parsing from streamed text.
///
/// This trait allows you to specify how the streamed text should be parsed
/// into structured data. It supports both partial parsing (as text streams in)
/// and complete parsing (when the stream finishes).
///
/// # Type Parameters
///
/// * `Partial` - The type of partial results emitted during streaming
/// * `Complete` - The type of the final complete result
pub trait Output: Send + Sync {
    /// The type of partial results that can be emitted during streaming
    type Partial: Send + Sync;
    
    /// The type of the complete result when streaming finishes
    type Complete: Send + Sync;
    
    /// Parse partial output from accumulated text so far.
    ///
    /// This is called as text accumulates during streaming to extract
    /// partial structured data. It should be tolerant of incomplete JSON.
    ///
    /// # Arguments
    ///
    /// * `text` - The accumulated text so far
    ///
    /// # Returns
    ///
    /// Returns `Ok(Some(partial))` if partial data could be extracted,
    /// `Ok(None)` if not enough data is available yet, or
    /// `Err(error)` if parsing failed.
    fn parse_partial(&self, text: &str) -> Result<Option<Self::Partial>, OutputParseError>;
    
    /// Parse the complete output from the final text.
    ///
    /// This is called when streaming completes to extract the final
    /// structured data. It should validate that the JSON is complete and valid.
    ///
    /// # Arguments
    ///
    /// * `text` - The complete accumulated text
    ///
    /// # Returns
    ///
    /// Returns `Ok(complete)` with the parsed result, or
    /// `Err(error)` if parsing or validation failed.
    fn parse_complete(&self, text: &str) -> Result<Self::Complete, OutputParseError>;
}

/// Error that can occur during output parsing.
#[derive(Debug, Clone)]
pub struct OutputParseError {
    pub message: String,
    pub kind: OutputParseErrorKind,
}

/// The kind of output parse error.
#[derive(Debug, Clone, PartialEq)]
pub enum OutputParseErrorKind {
    /// JSON syntax error
    InvalidJson,
    
    /// JSON is valid but doesn't match the expected schema
    SchemaMismatch,
    
    /// Required field is missing
    MissingField,
    
    /// Type mismatch (expected one type, got another)
    TypeMismatch,
    
    /// Other error
    Other,
}

impl OutputParseError {
    pub fn new(message: impl Into<String>, kind: OutputParseErrorKind) -> Self {
        Self {
            message: message.into(),
            kind,
        }
    }
    
    pub fn invalid_json(message: impl Into<String>) -> Self {
        Self::new(message, OutputParseErrorKind::InvalidJson)
    }
    
    pub fn schema_mismatch(message: impl Into<String>) -> Self {
        Self::new(message, OutputParseErrorKind::SchemaMismatch)
    }
    
    pub fn missing_field(field: impl Into<String>) -> Self {
        Self::new(
            format!("Missing required field: {}", field.into()),
            OutputParseErrorKind::MissingField,
        )
    }
    
    pub fn type_mismatch(message: impl Into<String>) -> Self {
        Self::new(message, OutputParseErrorKind::TypeMismatch)
    }
}

impl std::fmt::Display for OutputParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for OutputParseError {}

/// Output specification for plain text (default).
///
/// This is the default output type that doesn't parse the text,
/// just passes it through as-is.
pub struct TextOutput;

impl TextOutput {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TextOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl Output for TextOutput {
    type Partial = String;
    type Complete = String;
    
    fn parse_partial(&self, text: &str) -> Result<Option<Self::Partial>, OutputParseError> {
        if text.is_empty() {
            Ok(None)
        } else {
            Ok(Some(text.to_string()))
        }
    }
    
    fn parse_complete(&self, text: &str) -> Result<Self::Complete, OutputParseError> {
        Ok(text.to_string())
    }
}

/// Output specification for unstructured JSON.
///
/// This parses the text as JSON and returns serde_json::Value.
/// It attempts incremental parsing for partial results.
pub struct JsonOutput;

impl JsonOutput {
    pub fn new() -> Self {
        Self
    }
}

impl Default for JsonOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl Output for JsonOutput {
    type Partial = Value;
    type Complete = Value;
    
    fn parse_partial(&self, text: &str) -> Result<Option<Self::Partial>, OutputParseError> {
        if text.trim().is_empty() {
            return Ok(None);
        }
        
        // Try to parse as complete JSON first
        match serde_json::from_str::<Value>(text) {
            Ok(value) => Ok(Some(value)),
            Err(_) => {
                // If it fails, try to repair and parse incrementally
                // This is a simple approach - just try to close any open structures
                let repaired = repair_partial_json(text);
                match serde_json::from_str::<Value>(&repaired) {
                    Ok(value) => Ok(Some(value)),
                    Err(_) => Ok(None), // Not enough data yet
                }
            }
        }
    }
    
    fn parse_complete(&self, text: &str) -> Result<Self::Complete, OutputParseError> {
        serde_json::from_str(text)
            .map_err(|e| OutputParseError::invalid_json(format!("Failed to parse JSON: {}", e)))
    }
}

/// Output specification for structured objects with a schema.
///
/// This parses the text as JSON and deserializes it into a specific type.
/// For partial parsing, it attempts to deserialize what's available so far.
pub struct ObjectOutput<T>
where
    T: for<'de> Deserialize<'de> + Send + Sync,
{
    _phantom: PhantomData<T>,
}

impl<T> ObjectOutput<T>
where
    T: for<'de> Deserialize<'de> + Send + Sync,
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> Default for ObjectOutput<T>
where
    T: for<'de> Deserialize<'de> + Send + Sync,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Output for ObjectOutput<T>
where
    T: for<'de> Deserialize<'de> + Send + Sync + Clone + 'static,
{
    type Partial = T;
    type Complete = T;
    
    fn parse_partial(&self, text: &str) -> Result<Option<Self::Partial>, OutputParseError> {
        if text.trim().is_empty() {
            return Ok(None);
        }
        
        // Try direct parsing first
        match serde_json::from_str::<T>(text) {
            Ok(value) => Ok(Some(value)),
            Err(_) => {
                // Try to repair and parse
                let repaired = repair_partial_json(text);
                match serde_json::from_str::<T>(&repaired) {
                    Ok(value) => Ok(Some(value)),
                    Err(_) => Ok(None), // Not enough data yet
                }
            }
        }
    }
    
    fn parse_complete(&self, text: &str) -> Result<Self::Complete, OutputParseError> {
        serde_json::from_str(text).map_err(|e| {
            OutputParseError::invalid_json(format!("Failed to parse object: {}", e))
        })
    }
}

/// Output specification for arrays of a specific element type.
///
/// This parses the text as a JSON array and deserializes each element.
pub struct ArrayOutput<T>
where
    T: for<'de> Deserialize<'de> + Send + Sync,
{
    _phantom: PhantomData<T>,
}

impl<T> ArrayOutput<T>
where
    T: for<'de> Deserialize<'de> + Send + Sync,
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> Default for ArrayOutput<T>
where
    T: for<'de> Deserialize<'de> + Send + Sync,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Output for ArrayOutput<T>
where
    T: for<'de> Deserialize<'de> + Send + Sync + Clone + 'static,
{
    type Partial = Vec<T>;
    type Complete = Vec<T>;
    
    fn parse_partial(&self, text: &str) -> Result<Option<Self::Partial>, OutputParseError> {
        if text.trim().is_empty() {
            return Ok(None);
        }
        
        // Try direct parsing first
        match serde_json::from_str::<Vec<T>>(text) {
            Ok(value) => Ok(Some(value)),
            Err(_) => {
                // Try to repair and parse
                let repaired = repair_partial_json(text);
                match serde_json::from_str::<Vec<T>>(&repaired) {
                    Ok(value) => Ok(Some(value)),
                    Err(_) => Ok(None), // Not enough data yet
                }
            }
        }
    }
    
    fn parse_complete(&self, text: &str) -> Result<Self::Complete, OutputParseError> {
        serde_json::from_str(text).map_err(|e| {
            OutputParseError::invalid_json(format!("Failed to parse array: {}", e))
        })
    }
}

/// Output specification for choosing one of several string options.
pub struct ChoiceOutput {
    options: Vec<String>,
}

impl ChoiceOutput {
    pub fn new(options: Vec<String>) -> Self {
        Self { options }
    }
}

impl Output for ChoiceOutput {
    type Partial = Option<String>;
    type Complete = String;
    
    fn parse_partial(&self, text: &str) -> Result<Option<Self::Partial>, OutputParseError> {
        let text = text.trim();
        if text.is_empty() {
            return Ok(None);
        }
        
        // Check if any option is a prefix or the text starts with it
        for option in &self.options {
            if text.contains(option) {
                return Ok(Some(Some(option.clone())));
            }
        }
        
        Ok(Some(None)) // Text present but no match yet
    }
    
    fn parse_complete(&self, text: &str) -> Result<Self::Complete, OutputParseError> {
        let text = text.trim();
        
        // Try exact match first
        for option in &self.options {
            if text == option {
                return Ok(option.clone());
            }
        }
        
        // Try contains match
        for option in &self.options {
            if text.contains(option) {
                return Ok(option.clone());
            }
        }
        
        Err(OutputParseError::schema_mismatch(format!(
            "Text '{}' does not match any of the allowed choices: {:?}",
            text, self.options
        )))
    }
}

/// Attempt to repair partial JSON by closing open structures.
///
/// This is a simple heuristic approach that tries to make incomplete JSON
/// parseable by adding closing brackets/braces.
pub fn repair_partial_json(json: &str) -> String {
    let mut repaired = json.to_string();
    
    // Count open/close brackets and braces
    let mut brace_count = 0;
    let mut bracket_count = 0;
    let mut in_string = false;
    let mut escape_next = false;
    
    for ch in json.chars() {
        if escape_next {
            escape_next = false;
            continue;
        }
        
        match ch {
            '\\' => escape_next = true,
            '"' => in_string = !in_string,
            '{' if !in_string => brace_count += 1,
            '}' if !in_string => brace_count -= 1,
            '[' if !in_string => bracket_count += 1,
            ']' if !in_string => bracket_count -= 1,
            _ => {}
        }
    }
    
    // Close any open strings
    if in_string {
        repaired.push('"');
    }
    
    // Close any open brackets/braces
    for _ in 0..bracket_count {
        repaired.push(']');
    }
    for _ in 0..brace_count {
        repaired.push('}');
    }
    
    repaired
}

/// Helper functions for creating output specifications
pub mod output {
    use super::*;
    
    /// Create a text output specification (default).
    pub fn text() -> TextOutput {
        TextOutput::new()
    }
    
    /// Create a JSON output specification.
    pub fn json() -> JsonOutput {
        JsonOutput::new()
    }
    
    /// Create an object output specification.
    pub fn object<T>() -> ObjectOutput<T>
    where
        T: for<'de> Deserialize<'de> + Send + Sync,
    {
        ObjectOutput::new()
    }
    
    /// Create an array output specification.
    pub fn array<T>() -> ArrayOutput<T>
    where
        T: for<'de> Deserialize<'de> + Send + Sync,
    {
        ArrayOutput::new()
    }
    
    /// Create a choice output specification.
    pub fn choice(options: Vec<String>) -> ChoiceOutput {
        ChoiceOutput::new(options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
use serde::Deserialize;

    #[test]
    fn test_text_output() {
        let output = TextOutput::new();
        
        // Partial parsing
        assert_eq!(output.parse_partial("").unwrap(), None);
        assert_eq!(output.parse_partial("Hello").unwrap(), Some("Hello".to_string()));
        
        // Complete parsing
        assert_eq!(output.parse_complete("Hello, World!").unwrap(), "Hello, World!");
    }

    #[test]
    fn test_json_output() {
        let output = JsonOutput::new();
        
        // Complete JSON
        let result = output.parse_partial(r#"{"key": "value"}"#).unwrap();
        assert!(result.is_some());
        
        // Incomplete JSON - should try to repair
        let result = output.parse_partial(r#"{"key": "val"#).unwrap();
        // May or may not parse depending on repair logic
        
        // Complete parsing
        let complete = output.parse_complete(r#"{"key": "value"}"#).unwrap();
        assert_eq!(complete["key"], "value");
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct TestObject {
        name: String,
        age: u32,
    }

    #[test]
    fn test_object_output() {
        let output = ObjectOutput::<TestObject>::new();
        
        // Complete object
        let result = output.parse_partial(r#"{"name": "Alice", "age": 30}"#).unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "Alice");
        
        // Complete parsing
        let complete = output.parse_complete(r#"{"name": "Bob", "age": 25}"#).unwrap();
        assert_eq!(complete.name, "Bob");
        assert_eq!(complete.age, 25);
    }

    #[test]
    fn test_array_output() {
        let output = ArrayOutput::<i32>::new();
        
        // Complete array
        let result = output.parse_partial("[1, 2, 3]").unwrap();
        assert_eq!(result, Some(vec![1, 2, 3]));
        
        // Complete parsing
        let complete = output.parse_complete("[4, 5, 6]").unwrap();
        assert_eq!(complete, vec![4, 5, 6]);
    }

    #[test]
    fn test_choice_output() {
        let output = ChoiceOutput::new(vec!["yes".to_string(), "no".to_string(), "maybe".to_string()]);
        
        // Partial - should find match
        let result = output.parse_partial("I think yes is the answer").unwrap();
        assert_eq!(result, Some(Some("yes".to_string())));
        
        // Complete - exact match
        let complete = output.parse_complete("no").unwrap();
        assert_eq!(complete, "no");
        
        // Complete - contains match
        let complete = output.parse_complete("maybe tomorrow").unwrap();
        assert_eq!(complete, "maybe");
        
        // Error - no match
        assert!(output.parse_complete("definitely").is_err());
    }

    #[test]
    fn test_repair_partial_json() {
        assert_eq!(repair_partial_json(r#"{"key": "val"#), r#"{"key": "val"}"#);
        assert_eq!(repair_partial_json(r#"[1, 2, 3"#), r#"[1, 2, 3]"#);
        assert_eq!(repair_partial_json(r#"{"a": [1, 2"#), r#"{"a": [1, 2]}"#);
        assert_eq!(repair_partial_json(r#"{"key"#), r#"{"key"}"#);
    }
}
