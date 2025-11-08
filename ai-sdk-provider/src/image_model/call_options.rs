use crate::shared::provider_options::SharedProviderOptions;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use tokio_util::sync;

/// Size specification for image generation.
/// Must have the format `{width}x{height}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ImageSize {
    pub width: u32,
    pub height: u32,
}

impl ImageSize {
    /// Create a new image size.
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl fmt::Display for ImageSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

impl FromStr for ImageSize {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('x').collect();
        if parts.len() != 2 {
            return Err(format!(
                "Invalid size format: {}. Expected format: WIDTHxHEIGHT",
                s
            ));
        }

        let width = parts[0]
            .parse::<u32>()
            .map_err(|e| format!("Invalid width: {}", e))?;
        let height = parts[1]
            .parse::<u32>()
            .map_err(|e| format!("Invalid height: {}", e))?;

        Ok(ImageSize::new(width, height))
    }
}

/// Aspect ratio specification for image generation.
/// Must have the format `{width}:{height}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AspectRatio {
    pub width: u32,
    pub height: u32,
}

impl AspectRatio {
    /// Create a new aspect ratio.
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Common 16:9 aspect ratio
    pub fn ratio_16_9() -> Self {
        Self::new(16, 9)
    }

    /// Common 4:3 aspect ratio
    pub fn ratio_4_3() -> Self {
        Self::new(4, 3)
    }

    /// Square 1:1 aspect ratio
    pub fn ratio_1_1() -> Self {
        Self::new(1, 1)
    }

    /// Portrait 9:16 aspect ratio
    pub fn ratio_9_16() -> Self {
        Self::new(9, 16)
    }

    /// Portrait 3:4 aspect ratio
    pub fn ratio_3_4() -> Self {
        Self::new(3, 4)
    }
}

impl fmt::Display for AspectRatio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.width, self.height)
    }
}

impl FromStr for AspectRatio {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(format!(
                "Invalid aspect ratio format: {}. Expected format: WIDTH:HEIGHT",
                s
            ));
        }

        let width = parts[0]
            .parse::<u32>()
            .map_err(|e| format!("Invalid width: {}", e))?;
        let height = parts[1]
            .parse::<u32>()
            .map_err(|e| format!("Invalid height: {}", e))?;

        Ok(AspectRatio::new(width, height))
    }
}

/// Image model call options.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageModelCallOptions {
    /// Prompt for the image generation.
    pub prompt: String,

    /// Number of images to generate.
    pub n: u32,

    /// Size of the images to generate.
    /// Must have the format `{width}x{height}`.
    /// `None` will use the provider's default size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageSize>,

    /// Aspect ratio of the images to generate.
    /// Must have the format `{width}:{height}`.
    /// `None` will use the provider's default aspect ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<AspectRatio>,

    /// Seed for the image generation.
    /// `None` will use the provider's default seed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,

    /// Additional provider-specific options that are passed through to the provider
    /// as body parameters.
    ///
    /// The outer map is keyed by the provider name, and the inner
    /// map is keyed by the provider-specific metadata key.
    /// ```rust
    /// use std::collections::HashMap;
    /// use serde_json::json;
    ///
    /// let mut provider_options = HashMap::new();
    /// let mut openai_options = HashMap::new();
    /// openai_options.insert("style".to_string(), json!("vivid"));
    /// provider_options.insert("openai".to_string(), openai_options);
    /// ```
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<SharedProviderOptions>,

    /// Additional HTTP headers to be sent with the request.
    /// Only applicable for HTTP-based providers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,

    /// Abort/cancellation signal (not serialized, used for runtime control).
    #[serde(skip)]
    pub abort_signal: Option<sync::CancellationToken>,
}

impl ImageModelCallOptions {
    /// Create new call options with a prompt and number of images.
    pub fn new(prompt: impl Into<String>, n: u32) -> Self {
        Self {
            prompt: prompt.into(),
            n,
            size: None,
            aspect_ratio: None,
            seed: None,
            provider_options: None,
            headers: None,
            abort_signal: None,
        }
    }

    // Builder methods
    pub fn with_size(mut self, size: ImageSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_aspect_ratio(mut self, aspect_ratio: AspectRatio) -> Self {
        self.aspect_ratio = Some(aspect_ratio);
        self
    }

    pub fn with_seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn with_provider_options(mut self, options: SharedProviderOptions) -> Self {
        self.provider_options = Some(options);
        self
    }

    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }

    pub fn with_abort_signal(mut self, signal: sync::CancellationToken) -> Self {
        self.abort_signal = Some(signal);
        self
    }
}
