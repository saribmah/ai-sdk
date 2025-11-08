use crate::generate_text::GeneratedFile;
use serde::{Deserialize, Serialize};

/// A generated audio file.
///
/// This extends `GeneratedFile` with audio-specific properties like format.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratedAudioFile {
    /// The underlying file data.
    #[serde(flatten)]
    file: GeneratedFile,

    /// Audio format of the file (e.g., 'mp3', 'wav', 'ogg', etc.)
    pub format: String,
}

impl GeneratedAudioFile {
    /// Creates a new `GeneratedAudioFile` from base64-encoded data.
    ///
    /// The audio format is automatically determined from the media type.
    /// If the media type is "audio/mpeg", the format is set to "mp3".
    /// Otherwise, the format is extracted from the media type's subtype.
    ///
    /// # Arguments
    ///
    /// * `base64` - The audio content as a base64-encoded string
    /// * `media_type` - The IANA media type of the audio file
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_core::generate_speech::GeneratedAudioFile;
    ///
    /// let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3");
    /// assert_eq!(audio.format, "mp3");
    /// ```
    pub fn from_base64(base64: impl Into<String>, media_type: impl Into<String>) -> Self {
        let media_type_str = media_type.into();
        let format = Self::determine_format(&media_type_str);
        let file = GeneratedFile::from_base64(base64, media_type_str);

        Self { file, format }
    }

    /// Creates a new `GeneratedAudioFile` from raw bytes.
    ///
    /// The audio format is automatically determined from the media type.
    /// If the media type is "audio/mpeg", the format is set to "mp3".
    /// Otherwise, the format is extracted from the media type's subtype.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The audio content as raw bytes
    /// * `media_type` - The IANA media type of the audio file
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_core::generate_speech::GeneratedAudioFile;
    ///
    /// let audio = GeneratedAudioFile::from_bytes(b"audio data", "audio/wav");
    /// assert_eq!(audio.format, "wav");
    /// ```
    pub fn from_bytes(bytes: impl Into<Vec<u8>>, media_type: impl Into<String>) -> Self {
        let media_type_str = media_type.into();
        let format = Self::determine_format(&media_type_str);
        let file = GeneratedFile::from_bytes(bytes, media_type_str);

        Self { file, format }
    }

    /// Creates a new `GeneratedAudioFile` with an explicit format.
    ///
    /// Use this when you want to override the format determination logic.
    ///
    /// # Arguments
    ///
    /// * `base64` - The audio content as a base64-encoded string
    /// * `media_type` - The IANA media type of the audio file
    /// * `format` - The audio format (e.g., "mp3", "wav")
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_core::generate_speech::GeneratedAudioFile;
    ///
    /// let audio = GeneratedAudioFile::from_base64_with_format(
    ///     "SGVsbG8gV29ybGQh",
    ///     "audio/mpeg",
    ///     "mp3"
    /// );
    /// assert_eq!(audio.format, "mp3");
    /// ```
    pub fn from_base64_with_format(
        base64: impl Into<String>,
        media_type: impl Into<String>,
        format: impl Into<String>,
    ) -> Self {
        let file = GeneratedFile::from_base64(base64, media_type);
        Self {
            file,
            format: format.into(),
        }
    }

    /// Creates a new `GeneratedAudioFile` from bytes with an explicit format.
    ///
    /// Use this when you want to override the format determination logic.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The audio content as raw bytes
    /// * `media_type` - The IANA media type of the audio file
    /// * `format` - The audio format (e.g., "mp3", "wav")
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_core::generate_speech::GeneratedAudioFile;
    ///
    /// let audio = GeneratedAudioFile::from_bytes_with_format(
    ///     b"audio data",
    ///     "audio/mpeg",
    ///     "mp3"
    /// );
    /// assert_eq!(audio.format, "mp3");
    /// ```
    pub fn from_bytes_with_format(
        bytes: impl Into<Vec<u8>>,
        media_type: impl Into<String>,
        format: impl Into<String>,
    ) -> Self {
        let file = GeneratedFile::from_bytes(bytes, media_type);
        Self {
            file,
            format: format.into(),
        }
    }

    /// Sets the filename for this audio file.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_core::generate_speech::GeneratedAudioFile;
    ///
    /// let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3")
    ///     .with_name("speech.mp3");
    /// assert_eq!(audio.name(), Some("speech.mp3"));
    /// ```
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.file = self.file.with_name(name);
        self
    }

    /// Gets the filename of this audio file.
    pub fn name(&self) -> Option<&str> {
        self.file.name.as_deref()
    }

    /// Gets the audio content as a base64-encoded string.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_core::generate_speech::GeneratedAudioFile;
    ///
    /// let audio = GeneratedAudioFile::from_bytes(b"Hello World!", "audio/mp3");
    /// let base64 = audio.base64();
    /// assert_eq!(base64, "SGVsbG8gV29ybGQh");
    /// ```
    pub fn base64(&self) -> &str {
        self.file.base64()
    }

    /// Gets the audio content as raw bytes.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_core::generate_speech::GeneratedAudioFile;
    ///
    /// let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3");
    /// let bytes = audio.bytes();
    /// assert_eq!(bytes, b"Hello World!");
    /// ```
    pub fn bytes(&self) -> &[u8] {
        self.file.bytes()
    }

    /// Gets the audio content as a `Vec<u8>`.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_core::generate_speech::GeneratedAudioFile;
    ///
    /// let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3");
    /// let bytes = audio.to_vec();
    /// assert_eq!(bytes, b"Hello World!");
    /// ```
    pub fn to_vec(&self) -> Vec<u8> {
        self.file.to_vec()
    }

    /// Gets the media type of this audio file.
    pub fn media_type(&self) -> &str {
        &self.file.media_type
    }

    /// Gets a reference to the underlying GeneratedFile.
    pub fn as_file(&self) -> &GeneratedFile {
        &self.file
    }

    /// Converts this audio file into the underlying GeneratedFile.
    pub fn into_file(self) -> GeneratedFile {
        self.file
    }

    /// Determines the audio format from a media type.
    ///
    /// # Logic
    ///
    /// - If media type is "audio/mpeg", returns "mp3"
    /// - Otherwise, extracts the subtype from the media type (e.g., "audio/wav" -> "wav")
    /// - If extraction fails, defaults to "mp3"
    fn determine_format(media_type: &str) -> String {
        // Handle special case for audio/mpeg -> mp3
        if media_type == "audio/mpeg" {
            return "mp3".to_string();
        }

        // Try to extract format from media type
        if media_type.contains('/') {
            let parts: Vec<&str> = media_type.split('/').collect();
            if parts.len() == 2 && !parts[1].is_empty() {
                return parts[1].to_string();
            }
        }

        // Default to mp3 if we can't determine the format
        "mp3".to_string()
    }
}

/// A generated audio file with a type field.
///
/// This is an extension of `GeneratedAudioFile` that includes a `type` field
/// set to "audio", useful for serialization and type identification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratedAudioFileWithType {
    /// The underlying audio file data.
    #[serde(flatten)]
    audio: GeneratedAudioFile,

    /// The type field, always set to "audio".
    pub r#type: String,
}

impl GeneratedAudioFileWithType {
    /// Creates a new `GeneratedAudioFileWithType` from a `GeneratedAudioFile`.
    pub fn new(audio: GeneratedAudioFile) -> Self {
        Self {
            audio,
            r#type: "audio".to_string(),
        }
    }

    /// Creates a new `GeneratedAudioFileWithType` from base64-encoded data.
    pub fn from_base64(base64: impl Into<String>, media_type: impl Into<String>) -> Self {
        Self::new(GeneratedAudioFile::from_base64(base64, media_type))
    }

    /// Creates a new `GeneratedAudioFileWithType` from raw bytes.
    pub fn from_bytes(bytes: impl Into<Vec<u8>>, media_type: impl Into<String>) -> Self {
        Self::new(GeneratedAudioFile::from_bytes(bytes, media_type))
    }

    /// Gets a reference to the underlying audio file.
    pub fn audio(&self) -> &GeneratedAudioFile {
        &self.audio
    }

    /// Converts this into the underlying audio file.
    pub fn into_audio(self) -> GeneratedAudioFile {
        self.audio
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_base64_mp3() {
        let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3");
        assert_eq!(audio.format, "mp3");
        assert_eq!(audio.media_type(), "audio/mp3");
        assert_eq!(audio.base64(), "SGVsbG8gV29ybGQh");
    }

    #[test]
    fn test_from_base64_mpeg() {
        let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mpeg");
        assert_eq!(audio.format, "mp3");
        assert_eq!(audio.media_type(), "audio/mpeg");
    }

    #[test]
    fn test_from_base64_wav() {
        let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/wav");
        assert_eq!(audio.format, "wav");
        assert_eq!(audio.media_type(), "audio/wav");
    }

    #[test]
    fn test_from_base64_ogg() {
        let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/ogg");
        assert_eq!(audio.format, "ogg");
        assert_eq!(audio.media_type(), "audio/ogg");
    }

    #[test]
    fn test_from_bytes() {
        let audio = GeneratedAudioFile::from_bytes(b"Hello World!", "audio/mp3");
        assert_eq!(audio.format, "mp3");
        assert_eq!(audio.bytes(), b"Hello World!");
        assert_eq!(audio.base64(), "SGVsbG8gV29ybGQh");
    }

    #[test]
    fn test_from_base64_with_format() {
        let audio =
            GeneratedAudioFile::from_base64_with_format("SGVsbG8gV29ybGQh", "audio/mpeg", "mp3");
        assert_eq!(audio.format, "mp3");
        assert_eq!(audio.media_type(), "audio/mpeg");
    }

    #[test]
    fn test_from_bytes_with_format() {
        let audio =
            GeneratedAudioFile::from_bytes_with_format(b"Hello World!", "audio/mpeg", "mp3");
        assert_eq!(audio.format, "mp3");
        assert_eq!(audio.bytes(), b"Hello World!");
    }

    #[test]
    fn test_with_name() {
        let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3")
            .with_name("speech.mp3");
        assert_eq!(audio.name(), Some("speech.mp3"));
    }

    #[test]
    fn test_to_vec() {
        let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3");
        let vec = audio.to_vec();
        assert_eq!(vec, b"Hello World!");
    }

    #[test]
    fn test_determine_format_mpeg() {
        let format = GeneratedAudioFile::determine_format("audio/mpeg");
        assert_eq!(format, "mp3");
    }

    #[test]
    fn test_determine_format_mp3() {
        let format = GeneratedAudioFile::determine_format("audio/mp3");
        assert_eq!(format, "mp3");
    }

    #[test]
    fn test_determine_format_wav() {
        let format = GeneratedAudioFile::determine_format("audio/wav");
        assert_eq!(format, "wav");
    }

    #[test]
    fn test_determine_format_ogg() {
        let format = GeneratedAudioFile::determine_format("audio/ogg");
        assert_eq!(format, "ogg");
    }

    #[test]
    fn test_determine_format_flac() {
        let format = GeneratedAudioFile::determine_format("audio/flac");
        assert_eq!(format, "flac");
    }

    #[test]
    fn test_determine_format_invalid() {
        // Invalid media type should default to mp3
        let format = GeneratedAudioFile::determine_format("invalid");
        assert_eq!(format, "mp3");
    }

    #[test]
    fn test_determine_format_empty_subtype() {
        // Empty subtype should default to mp3
        let format = GeneratedAudioFile::determine_format("audio/");
        assert_eq!(format, "mp3");
    }

    #[test]
    fn test_as_file() {
        let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3");
        let file = audio.as_file();
        assert_eq!(file.media_type, "audio/mp3");
    }

    #[test]
    fn test_into_file() {
        let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3");
        let file = audio.into_file();
        assert_eq!(file.media_type, "audio/mp3");
    }

    #[test]
    fn test_equality() {
        let audio1 = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3");
        let audio2 = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3");
        assert_eq!(audio1, audio2);
    }

    #[test]
    fn test_inequality_different_content() {
        let audio1 = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3");
        let audio2 = GeneratedAudioFile::from_base64("R29vZGJ5ZQ==", "audio/mp3");
        assert_ne!(audio1, audio2);
    }

    #[test]
    fn test_inequality_different_format() {
        let audio1 = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3");
        let audio2 = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/wav");
        assert_ne!(audio1, audio2);
    }

    #[test]
    fn test_clone() {
        let audio1 =
            GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3").with_name("test.mp3");
        let audio2 = audio1.clone();
        assert_eq!(audio1, audio2);
        assert_eq!(audio1.name(), audio2.name());
    }

    #[test]
    fn test_generated_audio_file_with_type_new() {
        let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3");
        let with_type = GeneratedAudioFileWithType::new(audio);
        assert_eq!(with_type.r#type, "audio");
        assert_eq!(with_type.audio().format, "mp3");
    }

    #[test]
    fn test_generated_audio_file_with_type_from_base64() {
        let with_type = GeneratedAudioFileWithType::from_base64("SGVsbG8gV29ybGQh", "audio/mp3");
        assert_eq!(with_type.r#type, "audio");
        assert_eq!(with_type.audio().format, "mp3");
    }

    #[test]
    fn test_generated_audio_file_with_type_from_bytes() {
        let with_type = GeneratedAudioFileWithType::from_bytes(b"Hello World!", "audio/wav");
        assert_eq!(with_type.r#type, "audio");
        assert_eq!(with_type.audio().format, "wav");
    }

    #[test]
    fn test_generated_audio_file_with_type_into_audio() {
        let with_type = GeneratedAudioFileWithType::from_base64("SGVsbG8gV29ybGQh", "audio/mp3");
        let audio = with_type.into_audio();
        assert_eq!(audio.format, "mp3");
    }

    #[test]
    fn test_serialization() {
        let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3");
        let json = serde_json::to_value(&audio).unwrap();

        assert!(json.get("format").is_some());
        assert_eq!(json.get("format").unwrap(), "mp3");
        // GeneratedFile uses snake_case for serialization
        assert!(json.get("media_type").is_some());
        assert_eq!(json.get("media_type").unwrap(), "audio/mp3");
    }

    #[test]
    fn test_serialization_with_type() {
        let with_type = GeneratedAudioFileWithType::from_base64("SGVsbG8gV29ybGQh", "audio/mp3");
        let json = serde_json::to_value(&with_type).unwrap();

        assert!(json.get("type").is_some());
        assert_eq!(json.get("type").unwrap(), "audio");
        assert!(json.get("format").is_some());
        assert_eq!(json.get("format").unwrap(), "mp3");
    }

    #[test]
    fn test_round_trip_serialization() {
        let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3");
        let json = serde_json::to_value(&audio).unwrap();
        let deserialized: GeneratedAudioFile = serde_json::from_value(json).unwrap();

        assert_eq!(deserialized.format, "mp3");
        assert_eq!(deserialized.media_type(), "audio/mp3");
        assert_eq!(deserialized.base64(), "SGVsbG8gV29ybGQh");
    }

    #[test]
    fn test_various_audio_formats() {
        let formats = vec![
            ("audio/mp3", "mp3"),
            ("audio/mpeg", "mp3"),
            ("audio/wav", "wav"),
            ("audio/ogg", "ogg"),
            ("audio/flac", "flac"),
            ("audio/aac", "aac"),
            ("audio/opus", "opus"),
            ("audio/webm", "webm"),
        ];

        for (media_type, expected_format) in formats {
            let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", media_type);
            assert_eq!(
                audio.format, expected_format,
                "Failed for media type: {}",
                media_type
            );
        }
    }
}
