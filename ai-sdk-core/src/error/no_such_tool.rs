use crate::error::AISDKError;

/// Builder for [`AISDKError::NoSuchTool`].
///
/// # Examples
///
/// ```
/// use ai_sdk_core::error::{AISDKError, NoSuchToolErrorBuilder};
///
/// let error = NoSuchToolErrorBuilder::new("unknown_tool")
///     .available_tools(vec!["search".to_string(), "calculate".to_string()])
///     .build();
///
/// match error {
///     AISDKError::NoSuchTool { tool_name, available_tools } => {
///         assert_eq!(tool_name, "unknown_tool");
///         assert_eq!(available_tools, vec!["search", "calculate"]);
///     }
///     _ => panic!("Expected NoSuchTool"),
/// }
/// ```
#[derive(Debug, Clone)]
pub struct NoSuchToolErrorBuilder {
    tool_name: String,
    available_tools: Vec<String>,
}

impl NoSuchToolErrorBuilder {
    /// Creates a new builder for a no such tool error.
    ///
    /// # Arguments
    ///
    /// * `tool_name` - The name of the tool that was not found
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::NoSuchToolErrorBuilder;
    ///
    /// let builder = NoSuchToolErrorBuilder::new("unknown_tool");
    /// ```
    pub fn new(tool_name: impl Into<String>) -> Self {
        Self {
            tool_name: tool_name.into(),
            available_tools: Vec::new(),
        }
    }

    /// Sets the list of available tools.
    ///
    /// # Arguments
    ///
    /// * `available_tools` - List of tool names that are available
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::NoSuchToolErrorBuilder;
    ///
    /// let builder = NoSuchToolErrorBuilder::new("unknown_tool")
    ///     .available_tools(vec!["search".to_string(), "calculate".to_string()]);
    /// ```
    pub fn available_tools(mut self, available_tools: Vec<String>) -> Self {
        self.available_tools = available_tools;
        self
    }

    /// Adds a single available tool to the list.
    ///
    /// # Arguments
    ///
    /// * `tool` - Name of an available tool
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::NoSuchToolErrorBuilder;
    ///
    /// let builder = NoSuchToolErrorBuilder::new("unknown_tool")
    ///     .add_available_tool("search")
    ///     .add_available_tool("calculate");
    /// ```
    pub fn add_available_tool(mut self, tool: impl Into<String>) -> Self {
        self.available_tools.push(tool.into());
        self
    }

    /// Builds the [`AISDKError::NoSuchTool`] error.
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::NoSuchToolErrorBuilder;
    ///
    /// let error = NoSuchToolErrorBuilder::new("missing_tool")
    ///     .available_tools(vec!["tool1".to_string(), "tool2".to_string()])
    ///     .build();
    /// ```
    pub fn build(self) -> AISDKError {
        AISDKError::NoSuchTool {
            tool_name: self.tool_name,
            available_tools: self.available_tools,
        }
    }
}

impl AISDKError {
    /// Creates a new no such tool error.
    ///
    /// This is a convenience method that creates an error with a tool name
    /// and available tools list in one call.
    ///
    /// # Arguments
    ///
    /// * `tool_name` - The name of the tool that was not found
    /// * `available_tools` - List of tool names that are available
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::AISDKError;
    ///
    /// let error = AISDKError::no_such_tool(
    ///     "unknown_tool",
    ///     vec!["search".to_string(), "calculate".to_string()]
    /// );
    ///
    /// match error {
    ///     AISDKError::NoSuchTool { tool_name, available_tools } => {
    ///         assert_eq!(tool_name, "unknown_tool");
    ///         assert_eq!(available_tools.len(), 2);
    ///     }
    ///     _ => panic!("Expected NoSuchTool"),
    /// }
    /// ```
    pub fn no_such_tool(tool_name: impl Into<String>, available_tools: Vec<String>) -> Self {
        Self::NoSuchTool {
            tool_name: tool_name.into(),
            available_tools,
        }
    }

    /// Creates a builder for a no such tool error.
    ///
    /// # Arguments
    ///
    /// * `tool_name` - The name of the tool that was not found
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::AISDKError;
    ///
    /// let error = AISDKError::no_such_tool_builder("missing_tool")
    ///     .add_available_tool("search")
    ///     .add_available_tool("calculate")
    ///     .build();
    /// ```
    pub fn no_such_tool_builder(tool_name: impl Into<String>) -> NoSuchToolErrorBuilder {
        NoSuchToolErrorBuilder::new(tool_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_such_tool_simple() {
        let error = AISDKError::no_such_tool(
            "unknown_tool",
            vec!["search".to_string(), "calculate".to_string()],
        );

        match error {
            AISDKError::NoSuchTool {
                tool_name,
                available_tools,
            } => {
                assert_eq!(tool_name, "unknown_tool");
                assert_eq!(available_tools.len(), 2);
                assert!(available_tools.contains(&"search".to_string()));
                assert!(available_tools.contains(&"calculate".to_string()));
            }
            _ => panic!("Expected NoSuchTool"),
        }
    }

    #[test]
    fn test_no_such_tool_builder() {
        let error = NoSuchToolErrorBuilder::new("missing_tool")
            .available_tools(vec!["tool1".to_string(), "tool2".to_string()])
            .build();

        match error {
            AISDKError::NoSuchTool {
                tool_name,
                available_tools,
            } => {
                assert_eq!(tool_name, "missing_tool");
                assert_eq!(available_tools, vec!["tool1", "tool2"]);
            }
            _ => panic!("Expected NoSuchTool"),
        }
    }

    #[test]
    fn test_no_such_tool_builder_add_individual() {
        let error = NoSuchToolErrorBuilder::new("bad_tool")
            .add_available_tool("search")
            .add_available_tool("calculate")
            .add_available_tool("weather")
            .build();

        match error {
            AISDKError::NoSuchTool {
                tool_name,
                available_tools,
            } => {
                assert_eq!(tool_name, "bad_tool");
                assert_eq!(available_tools.len(), 3);
                assert!(available_tools.contains(&"search".to_string()));
                assert!(available_tools.contains(&"calculate".to_string()));
                assert!(available_tools.contains(&"weather".to_string()));
            }
            _ => panic!("Expected NoSuchTool"),
        }
    }

    #[test]
    fn test_no_such_tool_builder_via_error() {
        let error = AISDKError::no_such_tool_builder("invalid_tool")
            .add_available_tool("tool_a")
            .add_available_tool("tool_b")
            .build();

        match error {
            AISDKError::NoSuchTool {
                tool_name,
                available_tools,
            } => {
                assert_eq!(tool_name, "invalid_tool");
                assert_eq!(available_tools, vec!["tool_a", "tool_b"]);
            }
            _ => panic!("Expected NoSuchTool"),
        }
    }

    #[test]
    fn test_no_such_tool_empty_available_tools() {
        let error = AISDKError::no_such_tool("some_tool", vec![]);

        match error {
            AISDKError::NoSuchTool {
                tool_name,
                available_tools,
            } => {
                assert_eq!(tool_name, "some_tool");
                assert!(available_tools.is_empty());
            }
            _ => panic!("Expected NoSuchTool"),
        }
    }

    #[test]
    fn test_no_such_tool_display() {
        let error = AISDKError::no_such_tool(
            "bad_tool",
            vec!["good_tool1".to_string(), "good_tool2".to_string()],
        );
        let display = format!("{}", error);
        assert!(display.contains("No such tool"));
        assert!(display.contains("bad_tool"));
        assert!(display.contains("Available tools"));
    }

    #[test]
    fn test_no_such_tool_with_many_tools() {
        let available = (0..10).map(|i| format!("tool_{}", i)).collect::<Vec<_>>();
        let error = AISDKError::no_such_tool("unknown", available.clone());

        match error {
            AISDKError::NoSuchTool {
                tool_name,
                available_tools,
            } => {
                assert_eq!(tool_name, "unknown");
                assert_eq!(available_tools.len(), 10);
                for i in 0..10 {
                    assert!(available_tools.contains(&format!("tool_{}", i)));
                }
            }
            _ => panic!("Expected NoSuchTool"),
        }
    }

    #[test]
    fn test_no_such_tool_builder_mixed_methods() {
        let error = NoSuchToolErrorBuilder::new("test_tool")
            .add_available_tool("tool1")
            .available_tools(vec!["tool2".to_string(), "tool3".to_string()])
            .add_available_tool("tool4")
            .build();

        match error {
            AISDKError::NoSuchTool {
                tool_name,
                available_tools,
            } => {
                assert_eq!(tool_name, "test_tool");
                // available_tools() replaces previous tools, so we should have tool2, tool3, tool4
                assert_eq!(available_tools.len(), 3);
                assert!(available_tools.contains(&"tool2".to_string()));
                assert!(available_tools.contains(&"tool3".to_string()));
                assert!(available_tools.contains(&"tool4".to_string()));
            }
            _ => panic!("Expected NoSuchTool"),
        }
    }
}
