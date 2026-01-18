use serde::{Deserialize, Serialize};

/// AssemblyAI transcription model IDs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssemblyAITranscriptionModelId {
    /// Best quality model (highest accuracy)
    Best,
    /// Nano model (faster, lower cost)
    Nano,
}

impl AssemblyAITranscriptionModelId {
    /// Get the model ID as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Best => "best",
            Self::Nano => "nano",
        }
    }
}

impl From<&str> for AssemblyAITranscriptionModelId {
    fn from(s: &str) -> Self {
        match s {
            "nano" => Self::Nano,
            _ => Self::Best,
        }
    }
}

impl From<String> for AssemblyAITranscriptionModelId {
    fn from(s: String) -> Self {
        s.as_str().into()
    }
}

/// Boost parameter for word recognition.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BoostParam {
    Low,
    Default,
    High,
}

/// Language code for transcription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LanguageCode {
    #[serde(rename = "en")]
    En,
    #[serde(rename = "en_au")]
    EnAu,
    #[serde(rename = "en_uk")]
    EnUk,
    #[serde(rename = "en_us")]
    EnUs,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "pt")]
    Pt,
    #[serde(rename = "nl")]
    Nl,
    // Add more languages as needed
    Other(String),
}

/// Audio quality for PII redaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RedactPiiAudioQuality {
    Mp3,
    Wav,
}

/// PII redaction policies.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactPiiPolicy {
    AccountNumber,
    BankingInformation,
    BloodType,
    CreditCardCvv,
    CreditCardExpiration,
    CreditCardNumber,
    Date,
    DateInterval,
    DateOfBirth,
    DriversLicense,
    Drug,
    Duration,
    EmailAddress,
    Event,
    Filename,
    GenderSexuality,
    HealthcareNumber,
    Injury,
    IpAddress,
    Language,
    Location,
    MaritalStatus,
    MedicalCondition,
    MedicalProcess,
    MoneyAmount,
    Nationality,
    NumberSequence,
    Occupation,
    Organization,
    PassportNumber,
    Password,
    PersonAge,
    PersonName,
    PhoneNumber,
    PhysicalAttribute,
    PoliticalAffiliation,
    Religion,
    Statistics,
    Time,
    Url,
    UsSocialSecurityNumber,
    Username,
    VehicleId,
    ZodiacSign,
}

/// PII substitution method.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactPiiSub {
    EntityName,
    Hash,
}

/// Summary model type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SummaryModel {
    Informative,
    Conversational,
    Catchy,
}

/// Summary type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SummaryType {
    Bullets,
    BulletsVerbose,
    Gist,
    Headline,
    Paragraph,
}

/// Custom spelling rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomSpelling {
    /// Words or phrases to replace
    pub from: Vec<String>,
    /// Word to replace with
    pub to: String,
}

/// AssemblyAI-specific transcription options.
///
/// These options can be passed via provider options when calling do_generate().
///
/// # Example
///
/// ```no_run
/// use llm_kit_assemblyai::AssemblyAIClient;
/// use llm_kit_provider::transcription_model::call_options::TranscriptionModelCallOptions;
/// use llm_kit_provider::shared::provider_options::SharedProviderOptions;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let provider = AssemblyAIClient::new().api_key("key").build();
/// let model = provider.transcription_model("best");
///
/// let audio_data = vec![]; // Your audio data
/// let mut provider_options = SharedProviderOptions::new();
/// provider_options.insert(
///     "assemblyai".to_string(),
///     vec![
///         ("speakerLabels".to_string(), serde_json::json!(true)),
///         ("speakersExpected".to_string(), serde_json::json!(2)),
///     ]
///     .into_iter()
///     .collect(),
/// );
///
/// let call_options = TranscriptionModelCallOptions::mp3(audio_data)
///     .with_provider_options(provider_options);
///
/// let result = model.do_generate(call_options).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssemblyAITranscriptionOptions {
    /// End time of the audio in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_end_at: Option<i64>,

    /// Start time of the audio in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_start_from: Option<i64>,

    /// Enable auto chapter generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_chapters: Option<bool>,

    /// Enable auto highlights
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_highlights: Option<bool>,

    /// Boost parameter for word recognition
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost_param: Option<BoostParam>,

    /// Enable content safety filtering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_safety: Option<bool>,

    /// Confidence threshold for content safety (25-100)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_safety_confidence: Option<i32>,

    /// Custom spelling rules
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_spelling: Option<Vec<CustomSpelling>>,

    /// Include filler words (um, uh, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disfluencies: Option<bool>,

    /// Enable entity detection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_detection: Option<bool>,

    /// Filter profanity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_profanity: Option<bool>,

    /// Format text with punctuation and capitalization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_text: Option<bool>,

    /// Enable IAB categories detection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iab_categories: Option<bool>,

    /// Language code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<LanguageCode>,

    /// Confidence threshold for language detection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_confidence_threshold: Option<f64>,

    /// Enable language detection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_detection: Option<bool>,

    /// Process audio as multichannel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multichannel: Option<bool>,

    /// Add punctuation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub punctuate: Option<bool>,

    /// Redact PII
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redact_pii: Option<bool>,

    /// Redact PII in audio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redact_pii_audio: Option<bool>,

    /// Audio format for PII redaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redact_pii_audio_quality: Option<RedactPiiAudioQuality>,

    /// List of PII types to redact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redact_pii_policies: Option<Vec<RedactPiiPolicy>>,

    /// Substitution method for redacted PII
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redact_pii_sub: Option<RedactPiiSub>,

    /// Enable sentiment analysis
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment_analysis: Option<bool>,

    /// Identify different speakers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_labels: Option<bool>,

    /// Number of speakers expected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speakers_expected: Option<i32>,

    /// Speech detection threshold (0-1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_threshold: Option<f64>,

    /// Generate summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summarization: Option<bool>,

    /// Model for summarization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_model: Option<SummaryModel>,

    /// Type of summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_type: Option<SummaryType>,

    /// Webhook authentication header name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_auth_header_name: Option<String>,

    /// Webhook authentication header value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_auth_header_value: Option<String>,

    /// Webhook URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,

    /// Words to boost recognition for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_boost: Option<Vec<String>>,
}
