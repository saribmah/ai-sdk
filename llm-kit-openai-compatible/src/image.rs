/// Image generation model implementation for OpenAI-compatible APIs.
pub mod image_model;
/// Settings and configuration for image generation models.
pub mod settings;

pub use image_model::{OpenAICompatibleImageModel, OpenAICompatibleImageModelConfig};
pub use settings::OpenAICompatibleImageModelId;
