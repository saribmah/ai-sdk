use serde_json::Value;
use std::collections::HashMap;

pub type ProviderMetadata = HashMap<String, HashMap<String, Value>>;
