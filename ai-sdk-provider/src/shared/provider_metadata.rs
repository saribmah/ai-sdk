use serde_json::Value;
use std::collections::HashMap;

/// Provider-specific metadata organized by provider name
pub type SharedProviderMetadata = HashMap<String, HashMap<String, Value>>;
