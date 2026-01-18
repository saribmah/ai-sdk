use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataContent {
    Bytes(Vec<u8>),
    Base64(String),
    Url(url::Url),
}
