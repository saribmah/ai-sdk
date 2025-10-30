use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataContent {
    Bytes(Vec<u8>),
    Base64(String),
    Url(url::Url),
}
