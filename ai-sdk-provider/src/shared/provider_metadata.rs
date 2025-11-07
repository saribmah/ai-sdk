use serde_json::Value;
use std::collections::HashMap;

pub type SharedProviderMetadata = HashMap<String, HashMap<String, Value>>;
