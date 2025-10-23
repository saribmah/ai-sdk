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
}
