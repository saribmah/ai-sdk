use serde::{Deserialize, Serialize};

use crate::prompt::message::cache_control::AnthropicCacheControl;

/// Error result for text editor code execution.
///
/// Represents an error that occurred during text editor code execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextEditorCodeExecutionErrorResult {
    /// The type of content (always "text_editor_code_execution_tool_result_error")
    #[serde(rename = "type")]
    pub content_type: String,

    /// Error code describing what went wrong
    pub error_code: String,
}

impl TextEditorCodeExecutionErrorResult {
    /// Creates a new error result.
    ///
    /// # Arguments
    ///
    /// * `error_code` - Error code describing what went wrong
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::TextEditorCodeExecutionErrorResult;
    ///
    /// let result = TextEditorCodeExecutionErrorResult::new("FILE_NOT_FOUND");
    /// assert_eq!(result.error_code, "FILE_NOT_FOUND");
    /// ```
    pub fn new(error_code: impl Into<String>) -> Self {
        Self {
            content_type: "text_editor_code_execution_tool_result_error".to_string(),
            error_code: error_code.into(),
        }
    }
}

/// Create result for text editor code execution.
///
/// Represents the result of creating or updating a file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextEditorCodeExecutionCreateResult {
    /// The type of content (always "text_editor_code_execution_create_result")
    #[serde(rename = "type")]
    pub content_type: String,

    /// Whether this was a file update (true) or a new file creation (false)
    pub is_file_update: bool,
}

impl TextEditorCodeExecutionCreateResult {
    /// Creates a new create result.
    ///
    /// # Arguments
    ///
    /// * `is_file_update` - Whether this was a file update (true) or new file creation (false)
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::TextEditorCodeExecutionCreateResult;
    ///
    /// let result = TextEditorCodeExecutionCreateResult::new(false);
    /// assert!(!result.is_file_update);
    /// ```
    pub fn new(is_file_update: bool) -> Self {
        Self {
            content_type: "text_editor_code_execution_create_result".to_string(),
            is_file_update,
        }
    }

    /// Creates a result for a new file creation.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::TextEditorCodeExecutionCreateResult;
    ///
    /// let result = TextEditorCodeExecutionCreateResult::created();
    /// assert!(!result.is_file_update);
    /// ```
    pub fn created() -> Self {
        Self::new(false)
    }

    /// Creates a result for a file update.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::TextEditorCodeExecutionCreateResult;
    ///
    /// let result = TextEditorCodeExecutionCreateResult::updated();
    /// assert!(result.is_file_update);
    /// ```
    pub fn updated() -> Self {
        Self::new(true)
    }
}

/// View result for text editor code execution.
///
/// Represents the result of viewing a file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextEditorCodeExecutionViewResult {
    /// The type of content (always "text_editor_code_execution_view_result")
    #[serde(rename = "type")]
    pub content_type: String,

    /// The file content
    pub content: String,

    /// The file type/extension
    pub file_type: String,

    /// Number of lines in the view (if windowed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_lines: Option<u64>,

    /// Starting line number of the view (if windowed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_line: Option<u64>,

    /// Total number of lines in the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_lines: Option<u64>,
}

impl TextEditorCodeExecutionViewResult {
    /// Creates a new view result.
    ///
    /// # Arguments
    ///
    /// * `content` - The file content
    /// * `file_type` - The file type/extension
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::TextEditorCodeExecutionViewResult;
    ///
    /// let result = TextEditorCodeExecutionViewResult::new("print('hello')", "py");
    /// assert_eq!(result.content, "print('hello')");
    /// assert_eq!(result.file_type, "py");
    /// ```
    pub fn new(content: impl Into<String>, file_type: impl Into<String>) -> Self {
        Self {
            content_type: "text_editor_code_execution_view_result".to_string(),
            content: content.into(),
            file_type: file_type.into(),
            num_lines: None,
            start_line: None,
            total_lines: None,
        }
    }

    /// Sets the number of lines in the view.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::TextEditorCodeExecutionViewResult;
    ///
    /// let result = TextEditorCodeExecutionViewResult::new("content", "txt")
    ///     .with_num_lines(10);
    /// assert_eq!(result.num_lines, Some(10));
    /// ```
    pub fn with_num_lines(mut self, num_lines: u64) -> Self {
        self.num_lines = Some(num_lines);
        self
    }

    /// Sets the starting line number of the view.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::TextEditorCodeExecutionViewResult;
    ///
    /// let result = TextEditorCodeExecutionViewResult::new("content", "txt")
    ///     .with_start_line(5);
    /// assert_eq!(result.start_line, Some(5));
    /// ```
    pub fn with_start_line(mut self, start_line: u64) -> Self {
        self.start_line = Some(start_line);
        self
    }

    /// Sets the total number of lines in the file.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::TextEditorCodeExecutionViewResult;
    ///
    /// let result = TextEditorCodeExecutionViewResult::new("content", "txt")
    ///     .with_total_lines(100);
    /// assert_eq!(result.total_lines, Some(100));
    /// ```
    pub fn with_total_lines(mut self, total_lines: u64) -> Self {
        self.total_lines = Some(total_lines);
        self
    }

    /// Sets all line information at once (for windowed views).
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::TextEditorCodeExecutionViewResult;
    ///
    /// let result = TextEditorCodeExecutionViewResult::new("content", "txt")
    ///     .with_window(10, 5, 100);
    /// ```
    pub fn with_window(mut self, num_lines: u64, start_line: u64, total_lines: u64) -> Self {
        self.num_lines = Some(num_lines);
        self.start_line = Some(start_line);
        self.total_lines = Some(total_lines);
        self
    }
}

/// String replace result for text editor code execution.
///
/// Represents the result of replacing text in a file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextEditorCodeExecutionStrReplaceResult {
    /// The type of content (always "text_editor_code_execution_str_replace_result")
    #[serde(rename = "type")]
    pub content_type: String,

    /// The lines that were affected by the replacement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<String>>,

    /// Number of new lines after replacement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_lines: Option<u64>,

    /// Starting line number of new content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_start: Option<u64>,

    /// Number of old lines before replacement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_lines: Option<u64>,

    /// Starting line number of old content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_start: Option<u64>,
}

impl TextEditorCodeExecutionStrReplaceResult {
    /// Creates a new string replace result.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::TextEditorCodeExecutionStrReplaceResult;
    ///
    /// let result = TextEditorCodeExecutionStrReplaceResult::new();
    /// ```
    pub fn new() -> Self {
        Self {
            content_type: "text_editor_code_execution_str_replace_result".to_string(),
            lines: None,
            new_lines: None,
            new_start: None,
            old_lines: None,
            old_start: None,
        }
    }

    /// Sets the affected lines.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::TextEditorCodeExecutionStrReplaceResult;
    ///
    /// let result = TextEditorCodeExecutionStrReplaceResult::new()
    ///     .with_lines(vec!["line 1".to_string(), "line 2".to_string()]);
    /// ```
    pub fn with_lines(mut self, lines: Vec<String>) -> Self {
        self.lines = Some(lines);
        self
    }

    /// Sets the number of new lines.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::TextEditorCodeExecutionStrReplaceResult;
    ///
    /// let result = TextEditorCodeExecutionStrReplaceResult::new()
    ///     .with_new_lines(5);
    /// ```
    pub fn with_new_lines(mut self, new_lines: u64) -> Self {
        self.new_lines = Some(new_lines);
        self
    }

    /// Sets the starting line number of new content.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::TextEditorCodeExecutionStrReplaceResult;
    ///
    /// let result = TextEditorCodeExecutionStrReplaceResult::new()
    ///     .with_new_start(10);
    /// ```
    pub fn with_new_start(mut self, new_start: u64) -> Self {
        self.new_start = Some(new_start);
        self
    }

    /// Sets the number of old lines.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::TextEditorCodeExecutionStrReplaceResult;
    ///
    /// let result = TextEditorCodeExecutionStrReplaceResult::new()
    ///     .with_old_lines(3);
    /// ```
    pub fn with_old_lines(mut self, old_lines: u64) -> Self {
        self.old_lines = Some(old_lines);
        self
    }

    /// Sets the starting line number of old content.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::TextEditorCodeExecutionStrReplaceResult;
    ///
    /// let result = TextEditorCodeExecutionStrReplaceResult::new()
    ///     .with_old_start(10);
    /// ```
    pub fn with_old_start(mut self, old_start: u64) -> Self {
        self.old_start = Some(old_start);
        self
    }

    /// Sets all replacement information at once.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::TextEditorCodeExecutionStrReplaceResult;
    ///
    /// let result = TextEditorCodeExecutionStrReplaceResult::new()
    ///     .with_replacement_info(3, 10, 5, 10);
    /// ```
    pub fn with_replacement_info(
        mut self,
        old_lines: u64,
        old_start: u64,
        new_lines: u64,
        new_start: u64,
    ) -> Self {
        self.old_lines = Some(old_lines);
        self.old_start = Some(old_start);
        self.new_lines = Some(new_lines);
        self.new_start = Some(new_start);
        self
    }
}

impl Default for TextEditorCodeExecutionStrReplaceResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Content type for text editor code execution tool results.
///
/// Represents the different types of results that can be returned from
/// text editor code execution operations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TextEditorCodeExecutionResultContent {
    /// Error result
    #[serde(rename = "text_editor_code_execution_tool_result_error")]
    Error {
        /// Error code describing what went wrong
        error_code: String,
    },

    /// Create/update result
    #[serde(rename = "text_editor_code_execution_create_result")]
    Create {
        /// Whether this was a file update (true) or new file creation (false)
        is_file_update: bool,
    },

    /// View result
    #[serde(rename = "text_editor_code_execution_view_result")]
    View {
        /// The file content
        content: String,
        /// The file type/extension
        file_type: String,
        /// Number of lines in the view (if windowed)
        #[serde(skip_serializing_if = "Option::is_none")]
        num_lines: Option<u64>,
        /// Starting line number of the view (if windowed)
        #[serde(skip_serializing_if = "Option::is_none")]
        start_line: Option<u64>,
        /// Total number of lines in the file
        #[serde(skip_serializing_if = "Option::is_none")]
        total_lines: Option<u64>,
    },

    /// String replace result
    #[serde(rename = "text_editor_code_execution_str_replace_result")]
    StrReplace {
        /// The lines that were affected by the replacement
        #[serde(skip_serializing_if = "Option::is_none")]
        lines: Option<Vec<String>>,
        /// Number of new lines after replacement
        #[serde(skip_serializing_if = "Option::is_none")]
        new_lines: Option<u64>,
        /// Starting line number of new content
        #[serde(skip_serializing_if = "Option::is_none")]
        new_start: Option<u64>,
        /// Number of old lines before replacement
        #[serde(skip_serializing_if = "Option::is_none")]
        old_lines: Option<u64>,
        /// Starting line number of old content
        #[serde(skip_serializing_if = "Option::is_none")]
        old_start: Option<u64>,
    },
}

impl TextEditorCodeExecutionResultContent {
    /// Creates an error result.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::TextEditorCodeExecutionResultContent;
    ///
    /// let result = TextEditorCodeExecutionResultContent::error("FILE_NOT_FOUND");
    /// ```
    pub fn error(error_code: impl Into<String>) -> Self {
        Self::Error {
            error_code: error_code.into(),
        }
    }

    /// Creates a create result.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::TextEditorCodeExecutionResultContent;
    ///
    /// let result = TextEditorCodeExecutionResultContent::create(false);
    /// ```
    pub fn create(is_file_update: bool) -> Self {
        Self::Create { is_file_update }
    }

    /// Creates a view result.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::TextEditorCodeExecutionResultContent;
    ///
    /// let result = TextEditorCodeExecutionResultContent::view("content", "txt");
    /// ```
    pub fn view(content: impl Into<String>, file_type: impl Into<String>) -> Self {
        Self::View {
            content: content.into(),
            file_type: file_type.into(),
            num_lines: None,
            start_line: None,
            total_lines: None,
        }
    }

    /// Creates a string replace result.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::TextEditorCodeExecutionResultContent;
    ///
    /// let result = TextEditorCodeExecutionResultContent::str_replace();
    /// ```
    pub fn str_replace() -> Self {
        Self::StrReplace {
            lines: None,
            new_lines: None,
            new_start: None,
            old_lines: None,
            old_start: None,
        }
    }
}

/// Anthropic text editor code execution tool result content block.
///
/// Represents the result of text editor code execution from the code_execution_20250825 tool.
/// This is used to return execution results back to the model.
///
/// # Example
///
/// ```
/// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::{
///     AnthropicTextEditorCodeExecutionToolResultContent, TextEditorCodeExecutionResultContent
/// };
///
/// // Create a successful file creation result
/// let result = AnthropicTextEditorCodeExecutionToolResultContent::new(
///     "toolu_123",
///     TextEditorCodeExecutionResultContent::create(false)
/// );
///
/// // Create an error result
/// let error_result = AnthropicTextEditorCodeExecutionToolResultContent::error(
///     "toolu_456",
///     "FILE_NOT_FOUND"
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicTextEditorCodeExecutionToolResultContent {
    /// The type of content (always "text_editor_code_execution_tool_result")
    #[serde(rename = "type")]
    pub content_type: String,

    /// ID of the tool use this is a result for
    pub tool_use_id: String,

    /// The execution result content
    pub content: TextEditorCodeExecutionResultContent,

    /// Optional cache control configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<AnthropicCacheControl>,
}

impl AnthropicTextEditorCodeExecutionToolResultContent {
    /// Creates a new text editor code execution tool result content block.
    ///
    /// # Arguments
    ///
    /// * `tool_use_id` - ID of the tool use this is a result for
    /// * `content` - The execution result content
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::{
    ///     AnthropicTextEditorCodeExecutionToolResultContent, TextEditorCodeExecutionResultContent
    /// };
    ///
    /// let result = AnthropicTextEditorCodeExecutionToolResultContent::new(
    ///     "toolu_abc123",
    ///     TextEditorCodeExecutionResultContent::create(true)
    /// );
    /// ```
    pub fn new(
        tool_use_id: impl Into<String>,
        content: TextEditorCodeExecutionResultContent,
    ) -> Self {
        Self {
            content_type: "text_editor_code_execution_tool_result".to_string(),
            tool_use_id: tool_use_id.into(),
            content,
            cache_control: None,
        }
    }

    /// Creates an error result.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::AnthropicTextEditorCodeExecutionToolResultContent;
    ///
    /// let result = AnthropicTextEditorCodeExecutionToolResultContent::error(
    ///     "toolu_123",
    ///     "PERMISSION_DENIED"
    /// );
    /// ```
    pub fn error(tool_use_id: impl Into<String>, error_code: impl Into<String>) -> Self {
        Self::new(
            tool_use_id,
            TextEditorCodeExecutionResultContent::error(error_code),
        )
    }

    /// Creates a create result.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::AnthropicTextEditorCodeExecutionToolResultContent;
    ///
    /// let result = AnthropicTextEditorCodeExecutionToolResultContent::create(
    ///     "toolu_123",
    ///     false
    /// );
    /// ```
    pub fn create(tool_use_id: impl Into<String>, is_file_update: bool) -> Self {
        Self::new(
            tool_use_id,
            TextEditorCodeExecutionResultContent::create(is_file_update),
        )
    }

    /// Creates a view result.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::AnthropicTextEditorCodeExecutionToolResultContent;
    ///
    /// let result = AnthropicTextEditorCodeExecutionToolResultContent::view(
    ///     "toolu_123",
    ///     "print('hello')",
    ///     "py"
    /// );
    /// ```
    pub fn view(
        tool_use_id: impl Into<String>,
        content: impl Into<String>,
        file_type: impl Into<String>,
    ) -> Self {
        Self::new(
            tool_use_id,
            TextEditorCodeExecutionResultContent::view(content, file_type),
        )
    }

    /// Creates a string replace result.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::AnthropicTextEditorCodeExecutionToolResultContent;
    ///
    /// let result = AnthropicTextEditorCodeExecutionToolResultContent::str_replace("toolu_123");
    /// ```
    pub fn str_replace(tool_use_id: impl Into<String>) -> Self {
        Self::new(
            tool_use_id,
            TextEditorCodeExecutionResultContent::str_replace(),
        )
    }

    /// Sets the cache control for this tool result.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::AnthropicTextEditorCodeExecutionToolResultContent;
    /// use llm_kit_anthropic::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    ///
    /// let result = AnthropicTextEditorCodeExecutionToolResultContent::create("toolu_123", false)
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
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::AnthropicTextEditorCodeExecutionToolResultContent;
    /// use llm_kit_anthropic::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    ///
    /// let result = AnthropicTextEditorCodeExecutionToolResultContent::create("toolu_123", false)
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
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::AnthropicTextEditorCodeExecutionToolResultContent;
    ///
    /// let result = AnthropicTextEditorCodeExecutionToolResultContent::create("old_id", false)
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
    /// use llm_kit_anthropic::prompt::message::content::text_editor_code_execution_tool_result::{
    ///     AnthropicTextEditorCodeExecutionToolResultContent, TextEditorCodeExecutionResultContent
    /// };
    ///
    /// let result = AnthropicTextEditorCodeExecutionToolResultContent::create("toolu_123", false)
    ///     .with_content(TextEditorCodeExecutionResultContent::error("FILE_NOT_FOUND"));
    /// ```
    pub fn with_content(mut self, content: TextEditorCodeExecutionResultContent) -> Self {
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
    fn test_error_result_new() {
        let result = TextEditorCodeExecutionErrorResult::new("FILE_NOT_FOUND");

        assert_eq!(
            result.content_type,
            "text_editor_code_execution_tool_result_error"
        );
        assert_eq!(result.error_code, "FILE_NOT_FOUND");
    }

    #[test]
    fn test_create_result_new() {
        let result = TextEditorCodeExecutionCreateResult::new(true);

        assert_eq!(
            result.content_type,
            "text_editor_code_execution_create_result"
        );
        assert!(result.is_file_update);
    }

    #[test]
    fn test_create_result_created() {
        let result = TextEditorCodeExecutionCreateResult::created();

        assert!(!result.is_file_update);
    }

    #[test]
    fn test_create_result_updated() {
        let result = TextEditorCodeExecutionCreateResult::updated();

        assert!(result.is_file_update);
    }

    #[test]
    fn test_view_result_new() {
        let result = TextEditorCodeExecutionViewResult::new("content", "py");

        assert_eq!(
            result.content_type,
            "text_editor_code_execution_view_result"
        );
        assert_eq!(result.content, "content");
        assert_eq!(result.file_type, "py");
        assert!(result.num_lines.is_none());
        assert!(result.start_line.is_none());
        assert!(result.total_lines.is_none());
    }

    #[test]
    fn test_view_result_with_window() {
        let result =
            TextEditorCodeExecutionViewResult::new("content", "txt").with_window(10, 5, 100);

        assert_eq!(result.num_lines, Some(10));
        assert_eq!(result.start_line, Some(5));
        assert_eq!(result.total_lines, Some(100));
    }

    #[test]
    fn test_str_replace_result_new() {
        let result = TextEditorCodeExecutionStrReplaceResult::new();

        assert_eq!(
            result.content_type,
            "text_editor_code_execution_str_replace_result"
        );
        assert!(result.lines.is_none());
        assert!(result.new_lines.is_none());
        assert!(result.new_start.is_none());
        assert!(result.old_lines.is_none());
        assert!(result.old_start.is_none());
    }

    #[test]
    fn test_str_replace_result_with_replacement_info() {
        let result =
            TextEditorCodeExecutionStrReplaceResult::new().with_replacement_info(3, 10, 5, 12);

        assert_eq!(result.old_lines, Some(3));
        assert_eq!(result.old_start, Some(10));
        assert_eq!(result.new_lines, Some(5));
        assert_eq!(result.new_start, Some(12));
    }

    #[test]
    fn test_result_content_error() {
        let content = TextEditorCodeExecutionResultContent::error("TEST_ERROR");

        match content {
            TextEditorCodeExecutionResultContent::Error { error_code } => {
                assert_eq!(error_code, "TEST_ERROR");
            }
            _ => panic!("Expected Error variant"),
        }
    }

    #[test]
    fn test_result_content_create() {
        let content = TextEditorCodeExecutionResultContent::create(true);

        match content {
            TextEditorCodeExecutionResultContent::Create { is_file_update } => {
                assert!(is_file_update);
            }
            _ => panic!("Expected Create variant"),
        }
    }

    #[test]
    fn test_result_content_view() {
        let content = TextEditorCodeExecutionResultContent::view("test content", "rs");

        match content {
            TextEditorCodeExecutionResultContent::View {
                content, file_type, ..
            } => {
                assert_eq!(content, "test content");
                assert_eq!(file_type, "rs");
            }
            _ => panic!("Expected View variant"),
        }
    }

    #[test]
    fn test_result_content_str_replace() {
        let content = TextEditorCodeExecutionResultContent::str_replace();

        match content {
            TextEditorCodeExecutionResultContent::StrReplace { .. } => {
                // Success
            }
            _ => panic!("Expected StrReplace variant"),
        }
    }

    #[test]
    fn test_tool_result_new() {
        let result = AnthropicTextEditorCodeExecutionToolResultContent::new(
            "toolu_123",
            TextEditorCodeExecutionResultContent::create(false),
        );

        assert_eq!(
            result.content_type,
            "text_editor_code_execution_tool_result"
        );
        assert_eq!(result.tool_use_id, "toolu_123");
        assert!(result.cache_control.is_none());
    }

    #[test]
    fn test_tool_result_error() {
        let result =
            AnthropicTextEditorCodeExecutionToolResultContent::error("toolu_123", "ERROR_CODE");

        assert_eq!(result.tool_use_id, "toolu_123");
        match result.content {
            TextEditorCodeExecutionResultContent::Error { error_code } => {
                assert_eq!(error_code, "ERROR_CODE");
            }
            _ => panic!("Expected Error variant"),
        }
    }

    #[test]
    fn test_tool_result_create() {
        let result = AnthropicTextEditorCodeExecutionToolResultContent::create("toolu_123", true);

        match result.content {
            TextEditorCodeExecutionResultContent::Create { is_file_update } => {
                assert!(is_file_update);
            }
            _ => panic!("Expected Create variant"),
        }
    }

    #[test]
    fn test_tool_result_view() {
        let result = AnthropicTextEditorCodeExecutionToolResultContent::view(
            "toolu_123",
            "file content",
            "py",
        );

        match result.content {
            TextEditorCodeExecutionResultContent::View {
                content, file_type, ..
            } => {
                assert_eq!(content, "file content");
                assert_eq!(file_type, "py");
            }
            _ => panic!("Expected View variant"),
        }
    }

    #[test]
    fn test_tool_result_str_replace() {
        let result = AnthropicTextEditorCodeExecutionToolResultContent::str_replace("toolu_123");

        match result.content {
            TextEditorCodeExecutionResultContent::StrReplace { .. } => {
                // Success
            }
            _ => panic!("Expected StrReplace variant"),
        }
    }

    #[test]
    fn test_with_cache_control() {
        let cache = AnthropicCacheControl::new(CacheTTL::FiveMinutes);
        let result = AnthropicTextEditorCodeExecutionToolResultContent::create("toolu_123", false)
            .with_cache_control(cache.clone());

        assert_eq!(result.cache_control, Some(cache));
    }

    #[test]
    fn test_without_cache_control() {
        let result = AnthropicTextEditorCodeExecutionToolResultContent::create("toolu_123", false)
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour))
            .without_cache_control();

        assert!(result.cache_control.is_none());
    }

    #[test]
    fn test_with_tool_use_id() {
        let result = AnthropicTextEditorCodeExecutionToolResultContent::create("old_id", false)
            .with_tool_use_id("new_id");

        assert_eq!(result.tool_use_id, "new_id");
    }

    #[test]
    fn test_with_content() {
        let result = AnthropicTextEditorCodeExecutionToolResultContent::create("toolu_123", false)
            .with_content(TextEditorCodeExecutionResultContent::error("NEW_ERROR"));

        match result.content {
            TextEditorCodeExecutionResultContent::Error { error_code } => {
                assert_eq!(error_code, "NEW_ERROR");
            }
            _ => panic!("Expected Error variant"),
        }
    }

    #[test]
    fn test_serialize_error_result() {
        let result =
            AnthropicTextEditorCodeExecutionToolResultContent::error("toolu_123", "FILE_ERROR");
        let json = serde_json::to_value(&result).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "text_editor_code_execution_tool_result",
                "tool_use_id": "toolu_123",
                "content": {
                    "type": "text_editor_code_execution_tool_result_error",
                    "error_code": "FILE_ERROR"
                }
            })
        );
    }

    #[test]
    fn test_serialize_create_result() {
        let result = AnthropicTextEditorCodeExecutionToolResultContent::create("toolu_123", true);
        let json = serde_json::to_value(&result).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "text_editor_code_execution_tool_result",
                "tool_use_id": "toolu_123",
                "content": {
                    "type": "text_editor_code_execution_create_result",
                    "is_file_update": true
                }
            })
        );
    }

    #[test]
    fn test_serialize_view_result() {
        let result =
            AnthropicTextEditorCodeExecutionToolResultContent::view("toolu_123", "content", "py");
        let json = serde_json::to_value(&result).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "text_editor_code_execution_tool_result",
                "tool_use_id": "toolu_123",
                "content": {
                    "type": "text_editor_code_execution_view_result",
                    "content": "content",
                    "file_type": "py"
                }
            })
        );
    }

    #[test]
    fn test_serialize_view_result_with_window() {
        let mut result =
            AnthropicTextEditorCodeExecutionToolResultContent::view("toolu_123", "content", "txt");
        if let TextEditorCodeExecutionResultContent::View {
            ref mut num_lines,
            ref mut start_line,
            ref mut total_lines,
            ..
        } = result.content
        {
            *num_lines = Some(10);
            *start_line = Some(5);
            *total_lines = Some(100);
        }

        let json = serde_json::to_value(&result).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "text_editor_code_execution_tool_result",
                "tool_use_id": "toolu_123",
                "content": {
                    "type": "text_editor_code_execution_view_result",
                    "content": "content",
                    "file_type": "txt",
                    "num_lines": 10,
                    "start_line": 5,
                    "total_lines": 100
                }
            })
        );
    }

    #[test]
    fn test_serialize_str_replace_result() {
        let result = AnthropicTextEditorCodeExecutionToolResultContent::str_replace("toolu_123");
        let json = serde_json::to_value(&result).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "text_editor_code_execution_tool_result",
                "tool_use_id": "toolu_123",
                "content": {
                    "type": "text_editor_code_execution_str_replace_result"
                }
            })
        );
    }

    #[test]
    fn test_serialize_str_replace_result_with_info() {
        let mut result =
            AnthropicTextEditorCodeExecutionToolResultContent::str_replace("toolu_123");
        if let TextEditorCodeExecutionResultContent::StrReplace {
            ref mut lines,
            ref mut old_lines,
            ref mut old_start,
            ref mut new_lines,
            ref mut new_start,
        } = result.content
        {
            *lines = Some(vec!["line 1".to_string(), "line 2".to_string()]);
            *old_lines = Some(3);
            *old_start = Some(10);
            *new_lines = Some(5);
            *new_start = Some(12);
        }

        let json = serde_json::to_value(&result).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "text_editor_code_execution_tool_result",
                "tool_use_id": "toolu_123",
                "content": {
                    "type": "text_editor_code_execution_str_replace_result",
                    "lines": ["line 1", "line 2"],
                    "old_lines": 3,
                    "old_start": 10,
                    "new_lines": 5,
                    "new_start": 12
                }
            })
        );
    }

    #[test]
    fn test_serialize_with_cache() {
        let result = AnthropicTextEditorCodeExecutionToolResultContent::create("toolu_123", false)
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let json = serde_json::to_value(&result).unwrap();

        let obj = json.as_object().unwrap();
        assert!(obj.contains_key("cache_control"));
    }

    #[test]
    fn test_deserialize_error_result() {
        let json = json!({
            "type": "text_editor_code_execution_tool_result",
            "tool_use_id": "toolu_123",
            "content": {
                "type": "text_editor_code_execution_tool_result_error",
                "error_code": "ERROR"
            }
        });

        let result: AnthropicTextEditorCodeExecutionToolResultContent =
            serde_json::from_value(json).unwrap();

        assert_eq!(result.tool_use_id, "toolu_123");
        match result.content {
            TextEditorCodeExecutionResultContent::Error { error_code } => {
                assert_eq!(error_code, "ERROR");
            }
            _ => panic!("Expected Error variant"),
        }
    }

    #[test]
    fn test_deserialize_create_result() {
        let json = json!({
            "type": "text_editor_code_execution_tool_result",
            "tool_use_id": "toolu_123",
            "content": {
                "type": "text_editor_code_execution_create_result",
                "is_file_update": false
            }
        });

        let result: AnthropicTextEditorCodeExecutionToolResultContent =
            serde_json::from_value(json).unwrap();

        match result.content {
            TextEditorCodeExecutionResultContent::Create { is_file_update } => {
                assert!(!is_file_update);
            }
            _ => panic!("Expected Create variant"),
        }
    }

    #[test]
    fn test_equality() {
        let result1 = AnthropicTextEditorCodeExecutionToolResultContent::create("id", false);
        let result2 = AnthropicTextEditorCodeExecutionToolResultContent::create("id", false);
        let result3 = AnthropicTextEditorCodeExecutionToolResultContent::create("id", true);

        assert_eq!(result1, result2);
        assert_ne!(result1, result3);
    }

    #[test]
    fn test_clone() {
        let result1 =
            AnthropicTextEditorCodeExecutionToolResultContent::view("id", "content", "py")
                .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let result2 = result1.clone();

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_builder_chaining() {
        let result = AnthropicTextEditorCodeExecutionToolResultContent::create("id1", false)
            .with_tool_use_id("id2")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
            .with_content(TextEditorCodeExecutionResultContent::error("ERROR"))
            .without_cache_control();

        assert_eq!(result.tool_use_id, "id2");
        match result.content {
            TextEditorCodeExecutionResultContent::Error { error_code } => {
                assert_eq!(error_code, "ERROR");
            }
            _ => panic!("Expected Error variant"),
        }
        assert!(result.cache_control.is_none());
    }

    #[test]
    fn test_roundtrip_serialization() {
        let original = AnthropicTextEditorCodeExecutionToolResultContent::view(
            "toolu_roundtrip",
            "test content",
            "rs",
        )
        .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes));

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicTextEditorCodeExecutionToolResultContent =
            serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }
}
