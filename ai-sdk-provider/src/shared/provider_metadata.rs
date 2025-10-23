use std::collections::HashMap;
use serde_json::Value;

pub type ProviderMetadata = HashMap<String, HashMap<String, Value>>;
