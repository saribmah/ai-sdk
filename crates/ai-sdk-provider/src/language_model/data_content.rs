pub enum LanguageModelDataContent {
    Bytes(Vec<u8>),
    Base64(String),
    Url(url::Url)
}