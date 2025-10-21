use std::collections::HashMap;
use serde_json::Value;

pub type ProviderOptions = HashMap<String, HashMap<String, Value>>;
