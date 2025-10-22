use thiserror::Error;

#[derive(Debug, Error)]
pub enum AISDKError {
    #[error("Invalid argument for parameter '{parameter}': {message} (value: {value})")]
    InvalidArgument {
        parameter: String,
        value: String,
        message: String,
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
}
