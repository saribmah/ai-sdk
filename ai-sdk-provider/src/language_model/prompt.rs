mod message;

use serde::{Deserialize, Serialize};
use message::Message;

/// A prompt is a list of messages.
///
/// Note: Not all models and prompt formats support multi-modal inputs and
/// tool calls. The validation happens at runtime.
///
/// Note: This is not a user-facing prompt. The AI SDK methods will map the
/// user-facing prompt types such as chat or instruction prompts to this format.
pub type Prompt = Vec<Message>;
