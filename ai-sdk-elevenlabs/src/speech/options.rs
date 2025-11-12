/// ElevenLabs speech model identifiers.
pub type ElevenLabsSpeechModelId = String;

/// ElevenLabs voice identifiers.
pub type ElevenLabsSpeechVoiceId = String;

/// Common voice IDs for convenience.
pub mod voices {
    /// Default voice - Rachel (female, American)
    pub const RACHEL: &str = "21m00Tcm4TlvDq8ikWAM";

    /// Domi (female, American)
    pub const DOMI: &str = "AZnzlk1XvdvUeBnXmlld";

    /// Bella (female, American)
    pub const BELLA: &str = "EXAVITQu4vr4xnSDxMaL";

    /// Antoni (male, American)
    pub const ANTONI: &str = "ErXwobaYiN019PkySvjV";

    /// Elli (female, American)
    pub const ELLI: &str = "MF3mGyEYCl7XYWbV9V6O";

    /// Josh (male, American)
    pub const JOSH: &str = "TxGEqnHWrfWFTfGW9XjX";

    /// Arnold (male, American)
    pub const ARNOLD: &str = "VR6AewLTigWG4xSOukaG";

    /// Adam (male, American)
    pub const ADAM: &str = "pNInz6obpgDQGcFmaJgB";

    /// Sam (male, American)
    pub const SAM: &str = "yoZ06aMxZJJ28mfd3POQ";
}

/// Common model IDs for convenience.
pub mod models {
    /// ElevenLabs Multilingual v2 model
    pub const ELEVEN_MULTILINGUAL_V2: &str = "eleven_multilingual_v2";

    /// ElevenLabs Monolingual v1 model
    pub const ELEVEN_MONOLINGUAL_V1: &str = "eleven_monolingual_v1";

    /// ElevenLabs Turbo v2 model (fastest)
    pub const ELEVEN_TURBO_V2: &str = "eleven_turbo_v2";

    /// ElevenLabs Turbo v2.5 model
    pub const ELEVEN_TURBO_V2_5: &str = "eleven_turbo_v2_5";
}
