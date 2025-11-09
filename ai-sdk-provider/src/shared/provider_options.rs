use serde_json::Value;
use std::collections::HashMap;

/// Provider-specific options organized by provider name
pub type SharedProviderOptions = HashMap<String, HashMap<String, Value>>;
