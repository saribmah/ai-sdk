/// Together AI image model identifiers.
///
/// These are the supported image generation models available through Together AI.
/// For a complete list, see: https://api.together.ai/models
pub type TogetherAIImageModelId = String;

/// Common Together AI image model IDs
#[allow(dead_code)]
pub mod models {
    /// Stable Diffusion XL Base 1.0
    pub const STABLE_DIFFUSION_XL_BASE: &str = "stabilityai/stable-diffusion-xl-base-1.0";

    /// FLUX.1 Dev
    pub const FLUX_1_DEV: &str = "black-forest-labs/FLUX.1-dev";

    /// FLUX.1 Schnell
    pub const FLUX_1_SCHNELL: &str = "black-forest-labs/FLUX.1-schnell";

    /// FLUX.1 Schnell Free
    pub const FLUX_1_SCHNELL_FREE: &str = "black-forest-labs/FLUX.1-schnell-Free";

    /// FLUX.1.1 Pro
    pub const FLUX_1_1_PRO: &str = "black-forest-labs/FLUX.1.1-pro";

    /// FLUX.1 Pro
    pub const FLUX_1_PRO: &str = "black-forest-labs/FLUX.1-pro";
}
