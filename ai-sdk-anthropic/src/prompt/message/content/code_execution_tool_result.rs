use serde::{Deserialize, Serialize};

use crate::prompt::message::cache_control::AnthropicCacheControl;

/// Code execution result for the code_execution_20250522 tool.
///
/// Contains the output from executing code, including stdout, stderr, and return code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeExecutionResult {
    /// The type of content (always "code_execution_result")
    #[serde(rename = "type")]
    pub content_type: String,

    /// Standard output from the code execution
    pub stdout: String,

    /// Standard error from the code execution
    pub stderr: String,

    /// Return code from the code execution
    pub return_code: i32,
}

impl CodeExecutionResult {
    /// Creates a new code execution result.
    ///
    /// # Arguments
    ///
    /// * `stdout` - Standard output from the code execution
    /// * `stderr` - Standard error from the code execution
    /// * `return_code` - Return code from the code execution
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::code_execution_tool_result::CodeExecutionResult;
    ///
    /// let result = CodeExecutionResult::new("Hello, world!", "", 0);
    /// assert_eq!(result.stdout, "Hello, world!");
    /// assert_eq!(result.return_code, 0);
    /// ```
    pub fn new(stdout: impl Into<String>, stderr: impl Into<String>, return_code: i32) -> Self {
        Self {
            content_type: "code_execution_result".to_string(),
            stdout: stdout.into(),
            stderr: stderr.into(),
            return_code,
        }
    }

    /// Creates a successful code execution result (return code 0).
    ///
    /// # Arguments
    ///
    /// * `stdout` - Standard output from the code execution
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::code_execution_tool_result::CodeExecutionResult;
    ///
    /// let result = CodeExecutionResult::success("Output");
    /// assert_eq!(result.return_code, 0);
    /// ```
    pub fn success(stdout: impl Into<String>) -> Self {
        Self::new(stdout, "", 0)
    }

    /// Creates a failed code execution result with a non-zero return code.
    ///
    /// # Arguments
    ///
    /// * `stderr` - Standard error from the code execution
    /// * `return_code` - Non-zero return code from the code execution
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::code_execution_tool_result::CodeExecutionResult;
    ///
    /// let result = CodeExecutionResult::failure("Error message", 1);
    /// assert_eq!(result.return_code, 1);
    /// ```
    pub fn failure(stderr: impl Into<String>, return_code: i32) -> Self {
        Self::new("", stderr, return_code)
    }
}

/// Anthropic code execution tool result content block.
///
/// Represents the result of code execution from the code_execution_20250522 tool.
/// This is used to return execution results back to the model.
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::prompt::message::content::code_execution_tool_result::{
///     AnthropicCodeExecutionToolResultContent, CodeExecutionResult
/// };
///
/// // Create a successful execution result
/// let result = AnthropicCodeExecutionToolResultContent::new(
///     "toolu_123",
///     CodeExecutionResult::success("Output text")
/// );
///
/// // Create a failed execution result
/// let error_result = AnthropicCodeExecutionToolResultContent::new(
///     "toolu_456",
///     CodeExecutionResult::failure("Error: division by zero", 1)
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicCodeExecutionToolResultContent {
    /// The type of content (always "code_execution_tool_result")
    #[serde(rename = "type")]
    pub content_type: String,

    /// ID of the tool use this is a result for
    pub tool_use_id: String,

    /// The code execution result
    pub content: CodeExecutionResult,

    /// Optional cache control configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<AnthropicCacheControl>,
}

impl AnthropicCodeExecutionToolResultContent {
    /// Creates a new code execution tool result content block.
    ///
    /// # Arguments
    ///
    /// * `tool_use_id` - ID of the tool use this is a result for
    /// * `content` - The code execution result
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::code_execution_tool_result::{
    ///     AnthropicCodeExecutionToolResultContent, CodeExecutionResult
    /// };
    ///
    /// let result = AnthropicCodeExecutionToolResultContent::new(
    ///     "toolu_abc123",
    ///     CodeExecutionResult::new("Output", "", 0)
    /// );
    /// ```
    pub fn new(tool_use_id: impl Into<String>, content: CodeExecutionResult) -> Self {
        Self {
            content_type: "code_execution_tool_result".to_string(),
            tool_use_id: tool_use_id.into(),
            content,
            cache_control: None,
        }
    }

    /// Creates a new code execution tool result with detailed output.
    ///
    /// # Arguments
    ///
    /// * `tool_use_id` - ID of the tool use this is a result for
    /// * `stdout` - Standard output from the code execution
    /// * `stderr` - Standard error from the code execution
    /// * `return_code` - Return code from the code execution
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::code_execution_tool_result::AnthropicCodeExecutionToolResultContent;
    ///
    /// let result = AnthropicCodeExecutionToolResultContent::with_output(
    ///     "toolu_123",
    ///     "Hello, world!",
    ///     "",
    ///     0
    /// );
    /// ```
    pub fn with_output(
        tool_use_id: impl Into<String>,
        stdout: impl Into<String>,
        stderr: impl Into<String>,
        return_code: i32,
    ) -> Self {
        Self::new(
            tool_use_id,
            CodeExecutionResult::new(stdout, stderr, return_code),
        )
    }

    /// Creates a successful code execution tool result (return code 0).
    ///
    /// # Arguments
    ///
    /// * `tool_use_id` - ID of the tool use this is a result for
    /// * `stdout` - Standard output from the code execution
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::code_execution_tool_result::AnthropicCodeExecutionToolResultContent;
    ///
    /// let result = AnthropicCodeExecutionToolResultContent::success(
    ///     "toolu_123",
    ///     "Execution completed"
    /// );
    /// ```
    pub fn success(tool_use_id: impl Into<String>, stdout: impl Into<String>) -> Self {
        Self::new(tool_use_id, CodeExecutionResult::success(stdout))
    }

    /// Creates a failed code execution tool result with a non-zero return code.
    ///
    /// # Arguments
    ///
    /// * `tool_use_id` - ID of the tool use this is a result for
    /// * `stderr` - Standard error from the code execution
    /// * `return_code` - Non-zero return code from the code execution
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::code_execution_tool_result::AnthropicCodeExecutionToolResultContent;
    ///
    /// let result = AnthropicCodeExecutionToolResultContent::failure(
    ///     "toolu_456",
    ///     "Error: file not found",
    ///     2
    /// );
    /// ```
    pub fn failure(
        tool_use_id: impl Into<String>,
        stderr: impl Into<String>,
        return_code: i32,
    ) -> Self {
        Self::new(
            tool_use_id,
            CodeExecutionResult::failure(stderr, return_code),
        )
    }

    /// Sets the cache control for this code execution tool result.
    ///
    /// # Arguments
    ///
    /// * `cache_control` - The cache control configuration
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::code_execution_tool_result::{
    ///     AnthropicCodeExecutionToolResultContent, CodeExecutionResult
    /// };
    /// use ai_sdk_anthropic::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    ///
    /// let result = AnthropicCodeExecutionToolResultContent::success("toolu_123", "Output")
    ///     .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes));
    /// ```
    pub fn with_cache_control(mut self, cache_control: AnthropicCacheControl) -> Self {
        self.cache_control = Some(cache_control);
        self
    }

    /// Removes cache control from this code execution tool result.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::code_execution_tool_result::AnthropicCodeExecutionToolResultContent;
    /// use ai_sdk_anthropic::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    ///
    /// let result = AnthropicCodeExecutionToolResultContent::success("toolu_123", "Output")
    ///     .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour))
    ///     .without_cache_control();
    ///
    /// assert!(result.cache_control.is_none());
    /// ```
    pub fn without_cache_control(mut self) -> Self {
        self.cache_control = None;
        self
    }

    /// Updates the tool use ID.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::code_execution_tool_result::AnthropicCodeExecutionToolResultContent;
    ///
    /// let result = AnthropicCodeExecutionToolResultContent::success("old_id", "Output")
    ///     .with_tool_use_id("new_id");
    ///
    /// assert_eq!(result.tool_use_id, "new_id");
    /// ```
    pub fn with_tool_use_id(mut self, tool_use_id: impl Into<String>) -> Self {
        self.tool_use_id = tool_use_id.into();
        self
    }

    /// Updates the content.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::code_execution_tool_result::{
    ///     AnthropicCodeExecutionToolResultContent, CodeExecutionResult
    /// };
    ///
    /// let result = AnthropicCodeExecutionToolResultContent::success("toolu_123", "Old output")
    ///     .with_content(CodeExecutionResult::failure("New error", 1));
    ///
    /// assert_eq!(result.content.return_code, 1);
    /// ```
    pub fn with_content(mut self, content: CodeExecutionResult) -> Self {
        self.content = content;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    use serde_json::json;

    #[test]
    fn test_code_execution_result_new() {
        let result = CodeExecutionResult::new("stdout output", "stderr output", 0);

        assert_eq!(result.content_type, "code_execution_result");
        assert_eq!(result.stdout, "stdout output");
        assert_eq!(result.stderr, "stderr output");
        assert_eq!(result.return_code, 0);
    }

    #[test]
    fn test_code_execution_result_success() {
        let result = CodeExecutionResult::success("Hello, world!");

        assert_eq!(result.stdout, "Hello, world!");
        assert_eq!(result.stderr, "");
        assert_eq!(result.return_code, 0);
    }

    #[test]
    fn test_code_execution_result_failure() {
        let result = CodeExecutionResult::failure("Error occurred", 1);

        assert_eq!(result.stdout, "");
        assert_eq!(result.stderr, "Error occurred");
        assert_eq!(result.return_code, 1);
    }

    #[test]
    fn test_code_execution_tool_result_new() {
        let exec_result = CodeExecutionResult::new("output", "error", 0);
        let result = AnthropicCodeExecutionToolResultContent::new("toolu_123", exec_result);

        assert_eq!(result.content_type, "code_execution_tool_result");
        assert_eq!(result.tool_use_id, "toolu_123");
        assert_eq!(result.content.stdout, "output");
        assert_eq!(result.content.stderr, "error");
        assert_eq!(result.content.return_code, 0);
        assert!(result.cache_control.is_none());
    }

    #[test]
    fn test_code_execution_tool_result_with_output() {
        let result = AnthropicCodeExecutionToolResultContent::with_output(
            "toolu_123",
            "Hello",
            "Warning",
            0,
        );

        assert_eq!(result.content.stdout, "Hello");
        assert_eq!(result.content.stderr, "Warning");
        assert_eq!(result.content.return_code, 0);
    }

    #[test]
    fn test_code_execution_tool_result_success() {
        let result = AnthropicCodeExecutionToolResultContent::success("toolu_123", "Success!");

        assert_eq!(result.content.stdout, "Success!");
        assert_eq!(result.content.stderr, "");
        assert_eq!(result.content.return_code, 0);
    }

    #[test]
    fn test_code_execution_tool_result_failure() {
        let result =
            AnthropicCodeExecutionToolResultContent::failure("toolu_123", "Fatal error", 2);

        assert_eq!(result.content.stdout, "");
        assert_eq!(result.content.stderr, "Fatal error");
        assert_eq!(result.content.return_code, 2);
    }

    #[test]
    fn test_with_cache_control() {
        let cache = AnthropicCacheControl::new(CacheTTL::FiveMinutes);
        let result = AnthropicCodeExecutionToolResultContent::success("toolu_123", "Output")
            .with_cache_control(cache.clone());

        assert_eq!(result.cache_control, Some(cache));
    }

    #[test]
    fn test_without_cache_control() {
        let result = AnthropicCodeExecutionToolResultContent::success("toolu_123", "Output")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour))
            .without_cache_control();

        assert!(result.cache_control.is_none());
    }

    #[test]
    fn test_with_tool_use_id() {
        let result = AnthropicCodeExecutionToolResultContent::success("old_id", "Output")
            .with_tool_use_id("new_id");

        assert_eq!(result.tool_use_id, "new_id");
    }

    #[test]
    fn test_with_content() {
        let result = AnthropicCodeExecutionToolResultContent::success("toolu_123", "Old")
            .with_content(CodeExecutionResult::failure("New error", 1));

        assert_eq!(result.content.stderr, "New error");
        assert_eq!(result.content.return_code, 1);
    }

    #[test]
    fn test_serialize_without_cache() {
        let result =
            AnthropicCodeExecutionToolResultContent::with_output("toolu_123", "Hello", "", 0);
        let json = serde_json::to_value(&result).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "code_execution_tool_result",
                "tool_use_id": "toolu_123",
                "content": {
                    "type": "code_execution_result",
                    "stdout": "Hello",
                    "stderr": "",
                    "return_code": 0
                }
            })
        );
    }

    #[test]
    fn test_serialize_with_cache() {
        let result = AnthropicCodeExecutionToolResultContent::success("toolu_123", "Output")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes));
        let json = serde_json::to_value(&result).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "code_execution_tool_result",
                "tool_use_id": "toolu_123",
                "content": {
                    "type": "code_execution_result",
                    "stdout": "Output",
                    "stderr": "",
                    "return_code": 0
                },
                "cache_control": {
                    "type": "ephemeral",
                    "ttl": "5m"
                }
            })
        );
    }

    #[test]
    fn test_serialize_with_error() {
        let result = AnthropicCodeExecutionToolResultContent::failure(
            "toolu_456",
            "Error: division by zero",
            1,
        );
        let json = serde_json::to_value(&result).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "code_execution_tool_result",
                "tool_use_id": "toolu_456",
                "content": {
                    "type": "code_execution_result",
                    "stdout": "",
                    "stderr": "Error: division by zero",
                    "return_code": 1
                }
            })
        );
    }

    #[test]
    fn test_deserialize_without_cache() {
        let json = json!({
            "type": "code_execution_tool_result",
            "tool_use_id": "toolu_123",
            "content": {
                "type": "code_execution_result",
                "stdout": "Result",
                "stderr": "",
                "return_code": 0
            }
        });

        let result: AnthropicCodeExecutionToolResultContent = serde_json::from_value(json).unwrap();

        assert_eq!(result.tool_use_id, "toolu_123");
        assert_eq!(result.content.stdout, "Result");
        assert_eq!(result.content.return_code, 0);
        assert!(result.cache_control.is_none());
    }

    #[test]
    fn test_deserialize_with_cache() {
        let json = json!({
            "type": "code_execution_tool_result",
            "tool_use_id": "toolu_123",
            "content": {
                "type": "code_execution_result",
                "stdout": "Cached",
                "stderr": "",
                "return_code": 0
            },
            "cache_control": {
                "type": "ephemeral",
                "ttl": "1h"
            }
        });

        let result: AnthropicCodeExecutionToolResultContent = serde_json::from_value(json).unwrap();

        assert_eq!(result.content.stdout, "Cached");
        assert_eq!(
            result.cache_control,
            Some(AnthropicCacheControl::new(CacheTTL::OneHour))
        );
    }

    #[test]
    fn test_equality() {
        let result1 = AnthropicCodeExecutionToolResultContent::success("id", "same");
        let result2 = AnthropicCodeExecutionToolResultContent::success("id", "same");
        let result3 = AnthropicCodeExecutionToolResultContent::success("id", "different");

        assert_eq!(result1, result2);
        assert_ne!(result1, result3);
    }

    #[test]
    fn test_clone() {
        let result1 = AnthropicCodeExecutionToolResultContent::with_output("id", "out", "err", 1)
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let result2 = result1.clone();

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_builder_chaining() {
        let result = AnthropicCodeExecutionToolResultContent::success("id1", "output1")
            .with_tool_use_id("id2")
            .with_content(CodeExecutionResult::failure("error", 1))
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
            .without_cache_control();

        assert_eq!(result.tool_use_id, "id2");
        assert_eq!(result.content.stderr, "error");
        assert_eq!(result.content.return_code, 1);
        assert!(result.cache_control.is_none());
    }

    #[test]
    fn test_roundtrip_serialization() {
        let original = AnthropicCodeExecutionToolResultContent::with_output(
            "toolu_roundtrip",
            "stdout text",
            "stderr text",
            42,
        )
        .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes));

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicCodeExecutionToolResultContent =
            serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_empty_strings() {
        let result = AnthropicCodeExecutionToolResultContent::with_output("id", "", "", 0);

        assert_eq!(result.content.stdout, "");
        assert_eq!(result.content.stderr, "");
    }

    #[test]
    fn test_multiline_output() {
        let stdout = "Line 1\nLine 2\nLine 3";
        let stderr = "Warning 1\nWarning 2";
        let result = AnthropicCodeExecutionToolResultContent::with_output("id", stdout, stderr, 0);

        assert_eq!(result.content.stdout, stdout);
        assert_eq!(result.content.stderr, stderr);
    }

    #[test]
    fn test_unicode_output() {
        let result = AnthropicCodeExecutionToolResultContent::success("id", "Hello 👋 World 🌍");

        assert_eq!(result.content.stdout, "Hello 👋 World 🌍");
    }

    #[test]
    fn test_negative_return_code() {
        let result = CodeExecutionResult::new("", "Killed", -9);

        assert_eq!(result.return_code, -9);
    }

    #[test]
    fn test_large_return_code() {
        let result = CodeExecutionResult::new("", "Error", 255);

        assert_eq!(result.return_code, 255);
    }
}
