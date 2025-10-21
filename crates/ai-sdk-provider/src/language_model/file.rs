pub struct LanguageModelFile {
    pub media_type: String,
    pub data: FileData,
}

pub enum FileData {
    Base64(String),
    Binary(Vec<u8>),
}

impl LanguageModelFile {
    pub fn from_base64(media_type: impl Into<String>, data: impl Into<String>) -> Self {
        Self {
            media_type: media_type.into(),
            data: FileData::Base64(data.into()),
        }
    }

    pub fn from_binary(media_type: impl Into<String>, data: Vec<u8>) -> Self {
        Self {
            media_type: media_type.into(),
            data: FileData::Binary(data.into()),
        }
    }
}
