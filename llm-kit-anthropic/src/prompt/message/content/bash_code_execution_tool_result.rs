use serde::{Deserialize, Serialize};

use crate::prompt::message::cache_control::AnthropicCacheControl;

/// Output file reference for bash code execution.
///
/// Represents a file that was created or modified during bash code execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BashCodeExecutionOutput {
    /// The type of content (always "bash_code_execution_output")
    #[serde(rename = "type")]
    pub content_type: String,

    /// ID of the output file
    pub file_id: String,
}

impl BashCodeExecutionOutput {
    /// Creates a new bash code execution output.
    ///
    /// # Arguments
    ///
    /// * `file_id` - ID of the output file
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::bash_code_execution_tool_result::BashCodeExecutionOutput;
    ///
    /// let output = BashCodeExecutionOutput::new("file_abc123");
    /// assert_eq!(output.file_id, "file_abc123");
    /// ```
    pub fn new(file_id: impl Into<String>) -> Self {
        Self {
            content_type: "bash_code_execution_output".to_string(),
            file_id: file_id.into(),
        }
    }
}

impl From<String> for BashCodeExecutionOutput {
    /// Converts a String into a BashCodeExecutionOutput.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::bash_code_execution_tool_result::BashCodeExecutionOutput;
    ///
    /// let output: BashCodeExecutionOutput = "file_123".to_string().into();
    /// assert_eq!(output.file_id, "file_123");
    /// ```
    fn from(file_id: String) -> Self {
        Self::new(file_id)
    }
}

impl From<&str> for BashCodeExecutionOutput {
    /// Converts a string slice into a BashCodeExecutionOutput.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::bash_code_execution_tool_result::BashCodeExecutionOutput;
    ///
    /// let output: BashCodeExecutionOutput = "file_456".into();
    /// assert_eq!(output.file_id, "file_456");
    /// ```
    fn from(file_id: &str) -> Self {
        Self::new(file_id)
    }
}

/// Successful bash code execution result.
///
/// Contains the output from executing bash code, including stdout, stderr, return code,
/// and any output files that were created.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BashCodeExecutionResult {
    /// The type of content (always "bash_code_execution_result")
    #[serde(rename = "type")]
    pub content_type: String,

    /// Standard output from the bash execution
    pub stdout: String,

    /// Standard error from the bash execution
    pub stderr: String,

    /// Return code from the bash execution
    pub return_code: i32,

    /// Output files created during execution
    pub content: Vec<BashCodeExecutionOutput>,
}

impl BashCodeExecutionResult {
    /// Creates a new bash code execution result.
    ///
    /// # Arguments
    ///
    /// * `stdout` - Standard output from the bash execution
    /// * `stderr` - Standard error from the bash execution
    /// * `return_code` - Return code from the bash execution
    /// * `content` - Output files created during execution
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::bash_code_execution_tool_result::{
    ///     BashCodeExecutionResult, BashCodeExecutionOutput
    /// };
    ///
    /// let result = BashCodeExecutionResult::new(
    ///     "Hello, world!",
    ///     "",
    ///     0,
    ///     vec![BashCodeExecutionOutput::new("file_123")]
    /// );
    /// assert_eq!(result.stdout, "Hello, world!");
    /// assert_eq!(result.return_code, 0);
    /// ```
    pub fn new(
        stdout: impl Into<String>,
        stderr: impl Into<String>,
        return_code: i32,
        content: Vec<BashCodeExecutionOutput>,
    ) -> Self {
        Self {
            content_type: "bash_code_execution_result".to_string(),
            stdout: stdout.into(),
            stderr: stderr.into(),
            return_code,
            content,
        }
    }

    /// Creates a successful bash code execution result (return code 0) with no output files.
    ///
    /// # Arguments
    ///
    /// * `stdout` - Standard output from the bash execution
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::bash_code_execution_tool_result::BashCodeExecutionResult;
    ///
    /// let result = BashCodeExecutionResult::success("Output");
    /// assert_eq!(result.return_code, 0);
    /// assert!(result.content.is_empty());
    /// ```
    pub fn success(stdout: impl Into<String>) -> Self {
        Self::new(stdout, "", 0, vec![])
    }

    /// Creates a successful bash code execution result with output files.
    ///
    /// # Arguments
    ///
    /// * `stdout` - Standard output from the bash execution
    /// * `content` - Output files created during execution
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::bash_code_execution_tool_result::{
    ///     BashCodeExecutionResult, BashCodeExecutionOutput
    /// };
    ///
    /// let result = BashCodeExecutionResult::success_with_files(
    ///     "File created",
    ///     vec![BashCodeExecutionOutput::new("file_123")]
    /// );
    /// assert_eq!(result.content.len(), 1);
    /// ```
    pub fn success_with_files(
        stdout: impl Into<String>,
        content: Vec<BashCodeExecutionOutput>,
    ) -> Self {
        Self::new(stdout, "", 0, content)
    }

    /// Creates a failed bash code execution result with a non-zero return code.
    ///
    /// # Arguments
    ///
    /// * `stderr` - Standard error from the bash execution
    /// * `return_code` - Non-zero return code from the bash execution
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::bash_code_execution_tool_result::BashCodeExecutionResult;
    ///
    /// let result = BashCodeExecutionResult::failure("Error message", 1);
    /// assert_eq!(result.return_code, 1);
    /// ```
    pub fn failure(stderr: impl Into<String>, return_code: i32) -> Self {
        Self::new("", stderr, return_code, vec![])
    }

    /// Adds an output file to the result.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::bash_code_execution_tool_result::{
    ///     BashCodeExecutionResult, BashCodeExecutionOutput
    /// };
    ///
    /// let result = BashCodeExecutionResult::success("Output")
    ///     .with_output_file(BashCodeExecutionOutput::new("file_1"))
    ///     .with_output_file(BashCodeExecutionOutput::new("file_2"));
    /// assert_eq!(result.content.len(), 2);
    /// ```
    pub fn with_output_file(mut self, output: BashCodeExecutionOutput) -> Self {
        self.content.push(output);
        self
    }

    /// Adds an output file by file ID.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::bash_code_execution_tool_result::BashCodeExecutionResult;
    ///
    /// let result = BashCodeExecutionResult::success("Output")
    ///     .with_file_id("file_123");
    /// assert_eq!(result.content.len(), 1);
    /// ```
    pub fn with_file_id(mut self, file_id: impl Into<String>) -> Self {
        self.content.push(BashCodeExecutionOutput::new(file_id));
        self
    }

    /// Sets the output files.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::bash_code_execution_tool_result::{
    ///     BashCodeExecutionResult, BashCodeExecutionOutput
    /// };
    ///
    /// let result = BashCodeExecutionResult::success("Output")
    ///     .with_files(vec![
    ///         BashCodeExecutionOutput::new("file_1"),
    ///         BashCodeExecutionOutput::new("file_2")
    ///     ]);
    /// assert_eq!(result.content.len(), 2);
    /// ```
    pub fn with_files(mut self, content: Vec<BashCodeExecutionOutput>) -> Self {
        self.content = content;
        self
    }
}

/// Error result for bash code execution.
///
/// Represents an error that occurred during bash code execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BashCodeExecutionErrorResult {
    /// The type of content (always "bash_code_execution_tool_result_error")
    #[serde(rename = "type")]
    pub content_type: String,

    /// Error code describing what went wrong
    pub error_code: String,
}

impl BashCodeExecutionErrorResult {
    /// Creates a new bash code execution error result.
    ///
    /// # Arguments
    ///
    /// * `error_code` - Error code describing what went wrong
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::bash_code_execution_tool_result::BashCodeExecutionErrorResult;
    ///
    /// let result = BashCodeExecutionErrorResult::new("EXECUTION_TIMEOUT");
    /// assert_eq!(result.error_code, "EXECUTION_TIMEOUT");
    /// ```
    pub fn new(error_code: impl Into<String>) -> Self {
        Self {
            content_type: "bash_code_execution_tool_result_error".to_string(),
            error_code: error_code.into(),
        }
    }
}

/// Content type for bash code execution tool results.
///
/// Can be either a successful execution result or an error result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BashCodeExecutionResultContent {
    /// Successful execution result
    #[serde(rename = "bash_code_execution_result")]
    Result {
        /// Standard output from the bash execution
        stdout: String,
        /// Standard error from the bash execution
        stderr: String,
        /// Return code from the bash execution
        return_code: i32,
        /// Output files created during execution
        content: Vec<BashCodeExecutionOutput>,
    },

    /// Error result
    #[serde(rename = "bash_code_execution_tool_result_error")]
    Error {
        /// Error code describing what went wrong
        error_code: String,
    },
}

impl BashCodeExecutionResultContent {
    /// Creates a successful execution result.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::bash_code_execution_tool_result::BashCodeExecutionResultContent;
    ///
    /// let content = BashCodeExecutionResultContent::result("output", "", 0, vec![]);
    /// ```
    pub fn result(
        stdout: impl Into<String>,
        stderr: impl Into<String>,
        return_code: i32,
        content: Vec<BashCodeExecutionOutput>,
    ) -> Self {
        Self::Result {
            stdout: stdout.into(),
            stderr: stderr.into(),
            return_code,
            content,
        }
    }

    /// Creates an error result.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::bash_code_execution_tool_result::BashCodeExecutionResultContent;
    ///
    /// let content = BashCodeExecutionResultContent::error("TIMEOUT");
    /// ```
    pub fn error(error_code: impl Into<String>) -> Self {
        Self::Error {
            error_code: error_code.into(),
        }
    }

    /// Creates a successful result with no output files.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::bash_code_execution_tool_result::BashCodeExecutionResultContent;
    ///
    /// let content = BashCodeExecutionResultContent::success("output");
    /// ```
    pub fn success(stdout: impl Into<String>) -> Self {
        Self::result(stdout, "", 0, vec![])
    }
}

/// Anthropic bash code execution tool result content block.
///
/// Represents the result of bash code execution from the code_execution_20250825 tool.
/// This is used to return execution results back to the model.
///
/// # Example
///
/// ```
/// use llm_kit_anthropic::prompt::message::content::bash_code_execution_tool_result::{
///     AnthropicBashCodeExecutionToolResultContent, BashCodeExecutionResultContent,
///     BashCodeExecutionOutput
/// };
///
/// // Create a successful execution result
/// let result = AnthropicBashCodeExecutionToolResultContent::success(
///     "toolu_123",
///     "Output text"
/// );
///
/// // Create a result with output files
/// let result_with_files = AnthropicBashCodeExecutionToolResultContent::result(
///     "toolu_456",
///     "File created",
///     "",
///     0,
///     vec![BashCodeExecutionOutput::new("file_123")]
/// );
///
/// // Create an error result
/// let error_result = AnthropicBashCodeExecutionToolResultContent::error(
///     "toolu_789",
///     "EXECUTION_FAILED"
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicBashCodeExecutionToolResultContent {
    /// The type of content (always "bash_code_execution_tool_result")
    #[serde(rename = "type")]
    pub content_type: String,

    /// ID of the tool use this is a result for
    pub tool_use_id: String,

    /// The execution result content
    pub content: BashCodeExecutionResultContent,

    /// Optional cache control configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<AnthropicCacheControl>,
}

impl AnthropicBashCodeExecutionToolResultContent {
    /// Creates a new bash code execution tool result content block.
    ///
    /// # Arguments
    ///
    /// * `tool_use_id` - ID of the tool use this is a result for
    /// * `content` - The execution result content
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::bash_code_execution_tool_result::{
    ///     AnthropicBashCodeExecutionToolResultContent, BashCodeExecutionResultContent
    /// };
    ///
    /// let result = AnthropicBashCodeExecutionToolResultContent::new(
    ///     "toolu_abc123",
    ///     BashCodeExecutionResultContent::success("Output")
    /// );
    /// ```
    pub fn new(tool_use_id: impl Into<String>, content: BashCodeExecutionResultContent) -> Self {
        Self {
            content_type: "bash_code_execution_tool_result".to_string(),
            tool_use_id: tool_use_id.into(),
            content,
            cache_control: None,
        }
    }

    /// Creates a successful execution result with detailed output.
    ///
    /// # Arguments
    ///
    /// * `tool_use_id` - ID of the tool use this is a result for
    /// * `stdout` - Standard output from the bash execution
    /// * `stderr` - Standard error from the bash execution
    /// * `return_code` - Return code from the bash execution
    /// * `content` - Output files created during execution
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::bash_code_execution_tool_result::{
    ///     AnthropicBashCodeExecutionToolResultContent, BashCodeExecutionOutput
    /// };
    ///
    /// let result = AnthropicBashCodeExecutionToolResultContent::result(
    ///     "toolu_123",
    ///     "Hello, world!",
    ///     "",
    ///     0,
    ///     vec![BashCodeExecutionOutput::new("file_123")]
    /// );
    /// ```
    pub fn result(
        tool_use_id: impl Into<String>,
        stdout: impl Into<String>,
        stderr: impl Into<String>,
        return_code: i32,
        content: Vec<BashCodeExecutionOutput>,
    ) -> Self {
        Self::new(
            tool_use_id,
            BashCodeExecutionResultContent::result(stdout, stderr, return_code, content),
        )
    }

    /// Creates a successful execution result (return code 0) with no output files.
    ///
    /// # Arguments
    ///
    /// * `tool_use_id` - ID of the tool use this is a result for
    /// * `stdout` - Standard output from the bash execution
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::bash_code_execution_tool_result::AnthropicBashCodeExecutionToolResultContent;
    ///
    /// let result = AnthropicBashCodeExecutionToolResultContent::success(
    ///     "toolu_123",
    ///     "Execution completed"
    /// );
    /// ```
    pub fn success(tool_use_id: impl Into<String>, stdout: impl Into<String>) -> Self {
        Self::new(tool_use_id, BashCodeExecutionResultContent::success(stdout))
    }

    /// Creates an error result.
    ///
    /// # Arguments
    ///
    /// * `tool_use_id` - ID of the tool use this is a result for
    /// * `error_code` - Error code describing what went wrong
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::bash_code_execution_tool_result::AnthropicBashCodeExecutionToolResultContent;
    ///
    /// let result = AnthropicBashCodeExecutionToolResultContent::error(
    ///     "toolu_456",
    ///     "PERMISSION_DENIED"
    /// );
    /// ```
    pub fn error(tool_use_id: impl Into<String>, error_code: impl Into<String>) -> Self {
        Self::new(
            tool_use_id,
            BashCodeExecutionResultContent::error(error_code),
        )
    }

    /// Sets the cache control for this tool result.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::bash_code_execution_tool_result::AnthropicBashCodeExecutionToolResultContent;
    /// use llm_kit_anthropic::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    ///
    /// let result = AnthropicBashCodeExecutionToolResultContent::success("toolu_123", "Output")
    ///     .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes));
    /// ```
    pub fn with_cache_control(mut self, cache_control: AnthropicCacheControl) -> Self {
        self.cache_control = Some(cache_control);
        self
    }

    /// Removes cache control from this tool result.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::bash_code_execution_tool_result::AnthropicBashCodeExecutionToolResultContent;
    /// use llm_kit_anthropic::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    ///
    /// let result = AnthropicBashCodeExecutionToolResultContent::success("toolu_123", "Output")
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
    /// use llm_kit_anthropic::prompt::message::content::bash_code_execution_tool_result::AnthropicBashCodeExecutionToolResultContent;
    ///
    /// let result = AnthropicBashCodeExecutionToolResultContent::success("old_id", "Output")
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
    /// use llm_kit_anthropic::prompt::message::content::bash_code_execution_tool_result::{
    ///     AnthropicBashCodeExecutionToolResultContent, BashCodeExecutionResultContent
    /// };
    ///
    /// let result = AnthropicBashCodeExecutionToolResultContent::success("toolu_123", "Old output")
    ///     .with_content(BashCodeExecutionResultContent::error("NEW_ERROR"));
    /// ```
    pub fn with_content(mut self, content: BashCodeExecutionResultContent) -> Self {
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
    fn test_bash_output_new() {
        let output = BashCodeExecutionOutput::new("file_123");

        assert_eq!(output.content_type, "bash_code_execution_output");
        assert_eq!(output.file_id, "file_123");
    }

    #[test]
    fn test_bash_output_from_string() {
        let output: BashCodeExecutionOutput = "file_456".to_string().into();
        assert_eq!(output.file_id, "file_456");
    }

    #[test]
    fn test_bash_output_from_str() {
        let output: BashCodeExecutionOutput = "file_789".into();
        assert_eq!(output.file_id, "file_789");
    }

    #[test]
    fn test_bash_result_new() {
        let output = BashCodeExecutionOutput::new("file_1");
        let result = BashCodeExecutionResult::new("stdout", "stderr", 0, vec![output]);

        assert_eq!(result.content_type, "bash_code_execution_result");
        assert_eq!(result.stdout, "stdout");
        assert_eq!(result.stderr, "stderr");
        assert_eq!(result.return_code, 0);
        assert_eq!(result.content.len(), 1);
    }

    #[test]
    fn test_bash_result_success() {
        let result = BashCodeExecutionResult::success("Hello, world!");

        assert_eq!(result.stdout, "Hello, world!");
        assert_eq!(result.stderr, "");
        assert_eq!(result.return_code, 0);
        assert!(result.content.is_empty());
    }

    #[test]
    fn test_bash_result_success_with_files() {
        let outputs = vec![
            BashCodeExecutionOutput::new("file_1"),
            BashCodeExecutionOutput::new("file_2"),
        ];
        let result = BashCodeExecutionResult::success_with_files("Output", outputs);

        assert_eq!(result.stdout, "Output");
        assert_eq!(result.content.len(), 2);
    }

    #[test]
    fn test_bash_result_failure() {
        let result = BashCodeExecutionResult::failure("Error occurred", 1);

        assert_eq!(result.stdout, "");
        assert_eq!(result.stderr, "Error occurred");
        assert_eq!(result.return_code, 1);
        assert!(result.content.is_empty());
    }

    #[test]
    fn test_bash_result_with_output_file() {
        let result = BashCodeExecutionResult::success("Output")
            .with_output_file(BashCodeExecutionOutput::new("file_1"))
            .with_output_file(BashCodeExecutionOutput::new("file_2"));

        assert_eq!(result.content.len(), 2);
    }

    #[test]
    fn test_bash_result_with_file_id() {
        let result = BashCodeExecutionResult::success("Output")
            .with_file_id("file_1")
            .with_file_id("file_2");

        assert_eq!(result.content.len(), 2);
        assert_eq!(result.content[0].file_id, "file_1");
        assert_eq!(result.content[1].file_id, "file_2");
    }

    #[test]
    fn test_bash_result_with_files() {
        let files = vec![
            BashCodeExecutionOutput::new("file_1"),
            BashCodeExecutionOutput::new("file_2"),
        ];
        let result = BashCodeExecutionResult::success("Output").with_files(files);

        assert_eq!(result.content.len(), 2);
    }

    #[test]
    fn test_bash_error_result_new() {
        let error = BashCodeExecutionErrorResult::new("TIMEOUT");

        assert_eq!(error.content_type, "bash_code_execution_tool_result_error");
        assert_eq!(error.error_code, "TIMEOUT");
    }

    #[test]
    fn test_result_content_result() {
        let content = BashCodeExecutionResultContent::result("out", "err", 0, vec![]);

        match content {
            BashCodeExecutionResultContent::Result {
                stdout,
                stderr,
                return_code,
                content,
            } => {
                assert_eq!(stdout, "out");
                assert_eq!(stderr, "err");
                assert_eq!(return_code, 0);
                assert!(content.is_empty());
            }
            _ => panic!("Expected Result variant"),
        }
    }

    #[test]
    fn test_result_content_error() {
        let content = BashCodeExecutionResultContent::error("ERROR_CODE");

        match content {
            BashCodeExecutionResultContent::Error { error_code } => {
                assert_eq!(error_code, "ERROR_CODE");
            }
            _ => panic!("Expected Error variant"),
        }
    }

    #[test]
    fn test_result_content_success() {
        let content = BashCodeExecutionResultContent::success("output");

        match content {
            BashCodeExecutionResultContent::Result {
                stdout,
                return_code,
                ..
            } => {
                assert_eq!(stdout, "output");
                assert_eq!(return_code, 0);
            }
            _ => panic!("Expected Result variant"),
        }
    }

    #[test]
    fn test_tool_result_new() {
        let result = AnthropicBashCodeExecutionToolResultContent::new(
            "toolu_123",
            BashCodeExecutionResultContent::success("Output"),
        );

        assert_eq!(result.content_type, "bash_code_execution_tool_result");
        assert_eq!(result.tool_use_id, "toolu_123");
        assert!(result.cache_control.is_none());
    }

    #[test]
    fn test_tool_result_result() {
        let outputs = vec![BashCodeExecutionOutput::new("file_1")];
        let result = AnthropicBashCodeExecutionToolResultContent::result(
            "toolu_123",
            "out",
            "err",
            0,
            outputs,
        );

        assert_eq!(result.tool_use_id, "toolu_123");
        match result.content {
            BashCodeExecutionResultContent::Result { content, .. } => {
                assert_eq!(content.len(), 1);
            }
            _ => panic!("Expected Result variant"),
        }
    }

    #[test]
    fn test_tool_result_success() {
        let result = AnthropicBashCodeExecutionToolResultContent::success("toolu_123", "Success!");

        match result.content {
            BashCodeExecutionResultContent::Result {
                stdout,
                return_code,
                ..
            } => {
                assert_eq!(stdout, "Success!");
                assert_eq!(return_code, 0);
            }
            _ => panic!("Expected Result variant"),
        }
    }

    #[test]
    fn test_tool_result_error() {
        let result = AnthropicBashCodeExecutionToolResultContent::error("toolu_123", "ERROR_CODE");

        match result.content {
            BashCodeExecutionResultContent::Error { error_code } => {
                assert_eq!(error_code, "ERROR_CODE");
            }
            _ => panic!("Expected Error variant"),
        }
    }

    #[test]
    fn test_with_cache_control() {
        let cache = AnthropicCacheControl::new(CacheTTL::FiveMinutes);
        let result = AnthropicBashCodeExecutionToolResultContent::success("toolu_123", "Output")
            .with_cache_control(cache.clone());

        assert_eq!(result.cache_control, Some(cache));
    }

    #[test]
    fn test_without_cache_control() {
        let result = AnthropicBashCodeExecutionToolResultContent::success("toolu_123", "Output")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour))
            .without_cache_control();

        assert!(result.cache_control.is_none());
    }

    #[test]
    fn test_with_tool_use_id() {
        let result = AnthropicBashCodeExecutionToolResultContent::success("old_id", "Output")
            .with_tool_use_id("new_id");

        assert_eq!(result.tool_use_id, "new_id");
    }

    #[test]
    fn test_with_content() {
        let result = AnthropicBashCodeExecutionToolResultContent::success("toolu_123", "Old")
            .with_content(BashCodeExecutionResultContent::error("NEW_ERROR"));

        match result.content {
            BashCodeExecutionResultContent::Error { error_code } => {
                assert_eq!(error_code, "NEW_ERROR");
            }
            _ => panic!("Expected Error variant"),
        }
    }

    #[test]
    fn test_serialize_success_result() {
        let result = AnthropicBashCodeExecutionToolResultContent::success("toolu_123", "Hello");
        let json = serde_json::to_value(&result).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "bash_code_execution_tool_result",
                "tool_use_id": "toolu_123",
                "content": {
                    "type": "bash_code_execution_result",
                    "stdout": "Hello",
                    "stderr": "",
                    "return_code": 0,
                    "content": []
                }
            })
        );
    }

    #[test]
    fn test_serialize_result_with_files() {
        let outputs = vec![
            BashCodeExecutionOutput::new("file_1"),
            BashCodeExecutionOutput::new("file_2"),
        ];
        let result = AnthropicBashCodeExecutionToolResultContent::result(
            "toolu_123",
            "Created files",
            "",
            0,
            outputs,
        );
        let json = serde_json::to_value(&result).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "bash_code_execution_tool_result",
                "tool_use_id": "toolu_123",
                "content": {
                    "type": "bash_code_execution_result",
                    "stdout": "Created files",
                    "stderr": "",
                    "return_code": 0,
                    "content": [
                        {
                            "type": "bash_code_execution_output",
                            "file_id": "file_1"
                        },
                        {
                            "type": "bash_code_execution_output",
                            "file_id": "file_2"
                        }
                    ]
                }
            })
        );
    }

    #[test]
    fn test_serialize_error_result() {
        let result =
            AnthropicBashCodeExecutionToolResultContent::error("toolu_456", "EXECUTION_ERROR");
        let json = serde_json::to_value(&result).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "bash_code_execution_tool_result",
                "tool_use_id": "toolu_456",
                "content": {
                    "type": "bash_code_execution_tool_result_error",
                    "error_code": "EXECUTION_ERROR"
                }
            })
        );
    }

    #[test]
    fn test_serialize_with_cache() {
        let result = AnthropicBashCodeExecutionToolResultContent::success("toolu_123", "Output")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let json = serde_json::to_value(&result).unwrap();

        let obj = json.as_object().unwrap();
        assert!(obj.contains_key("cache_control"));
    }

    #[test]
    fn test_deserialize_success_result() {
        let json = json!({
            "type": "bash_code_execution_tool_result",
            "tool_use_id": "toolu_123",
            "content": {
                "type": "bash_code_execution_result",
                "stdout": "Result",
                "stderr": "",
                "return_code": 0,
                "content": []
            }
        });

        let result: AnthropicBashCodeExecutionToolResultContent =
            serde_json::from_value(json).unwrap();

        assert_eq!(result.tool_use_id, "toolu_123");
        match result.content {
            BashCodeExecutionResultContent::Result {
                stdout,
                return_code,
                ..
            } => {
                assert_eq!(stdout, "Result");
                assert_eq!(return_code, 0);
            }
            _ => panic!("Expected Result variant"),
        }
    }

    #[test]
    fn test_deserialize_result_with_files() {
        let json = json!({
            "type": "bash_code_execution_tool_result",
            "tool_use_id": "toolu_123",
            "content": {
                "type": "bash_code_execution_result",
                "stdout": "Files created",
                "stderr": "",
                "return_code": 0,
                "content": [
                    {"type": "bash_code_execution_output", "file_id": "file_1"},
                    {"type": "bash_code_execution_output", "file_id": "file_2"}
                ]
            }
        });

        let result: AnthropicBashCodeExecutionToolResultContent =
            serde_json::from_value(json).unwrap();

        match result.content {
            BashCodeExecutionResultContent::Result { content, .. } => {
                assert_eq!(content.len(), 2);
                assert_eq!(content[0].file_id, "file_1");
                assert_eq!(content[1].file_id, "file_2");
            }
            _ => panic!("Expected Result variant"),
        }
    }

    #[test]
    fn test_deserialize_error_result() {
        let json = json!({
            "type": "bash_code_execution_tool_result",
            "tool_use_id": "toolu_456",
            "content": {
                "type": "bash_code_execution_tool_result_error",
                "error_code": "TIMEOUT"
            }
        });

        let result: AnthropicBashCodeExecutionToolResultContent =
            serde_json::from_value(json).unwrap();

        match result.content {
            BashCodeExecutionResultContent::Error { error_code } => {
                assert_eq!(error_code, "TIMEOUT");
            }
            _ => panic!("Expected Error variant"),
        }
    }

    #[test]
    fn test_equality() {
        let result1 = AnthropicBashCodeExecutionToolResultContent::success("id", "same");
        let result2 = AnthropicBashCodeExecutionToolResultContent::success("id", "same");
        let result3 = AnthropicBashCodeExecutionToolResultContent::success("id", "different");

        assert_eq!(result1, result2);
        assert_ne!(result1, result3);
    }

    #[test]
    fn test_clone() {
        let result1 = AnthropicBashCodeExecutionToolResultContent::result(
            "id",
            "out",
            "err",
            1,
            vec![BashCodeExecutionOutput::new("file_1")],
        )
        .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let result2 = result1.clone();

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_builder_chaining() {
        let result = AnthropicBashCodeExecutionToolResultContent::success("id1", "output1")
            .with_tool_use_id("id2")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
            .with_content(BashCodeExecutionResultContent::error("ERROR"))
            .without_cache_control();

        assert_eq!(result.tool_use_id, "id2");
        match result.content {
            BashCodeExecutionResultContent::Error { error_code } => {
                assert_eq!(error_code, "ERROR");
            }
            _ => panic!("Expected Error variant"),
        }
        assert!(result.cache_control.is_none());
    }

    #[test]
    fn test_roundtrip_serialization() {
        let original = AnthropicBashCodeExecutionToolResultContent::result(
            "toolu_roundtrip",
            "stdout text",
            "stderr text",
            42,
            vec![
                BashCodeExecutionOutput::new("file_1"),
                BashCodeExecutionOutput::new("file_2"),
            ],
        )
        .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes));

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicBashCodeExecutionToolResultContent =
            serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }
}
