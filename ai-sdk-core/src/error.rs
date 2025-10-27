use thiserror::Error;

#[derive(Debug, Error)]
pub enum AISDKError {
    #[error("Invalid argument for parameter '{parameter}': {message} (value: {value})")]
    InvalidArgument {
        parameter: String,
        value: String,
        message: String,
    },

    #[error("Invalid prompt: {message}")]
    InvalidPrompt { message: String },

    #[error("Model error: {message}")]
    ModelError { message: String },

    #[error("Invalid tool input for tool '{tool_name}': {message}")]
    InvalidToolInput {
        tool_name: String,
        tool_input: String,
        message: String,
    },

    #[error("No such tool: '{tool_name}'. Available tools: {available_tools:?}")]
    NoSuchTool {
        tool_name: String,
        available_tools: Vec<String>,
    },
}

impl AISDKError {
    pub fn invalid_argument(parameter: impl Into<String>, value: impl std::fmt::Display, message: impl Into<String>) -> Self {
        Self::InvalidArgument {
            parameter: parameter.into(),
            value: value.to_string(),
            message: message.into(),
        }
    }

    pub fn invalid_prompt(message: impl Into<String>) -> Self {
        Self::InvalidPrompt {
            message: message.into(),
        }
    }

    pub fn model_error(message: impl Into<String>) -> Self {
        Self::ModelError {
            message: message.into(),
        }
    }

    pub fn invalid_tool_input(
        tool_name: impl Into<String>,
        tool_input: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::InvalidToolInput {
            tool_name: tool_name.into(),
            tool_input: tool_input.into(),
            message: message.into(),
        }
    }

    pub fn no_such_tool(tool_name: impl Into<String>, available_tools: Vec<String>) -> Self {
        Self::NoSuchTool {
            tool_name: tool_name.into(),
            available_tools,
        }
    }
}
