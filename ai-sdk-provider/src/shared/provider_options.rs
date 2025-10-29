use serde_json::Value;
use std::collections::HashMap;

pub type ProviderOptions = HashMap<String, HashMap<String, Value>>;
