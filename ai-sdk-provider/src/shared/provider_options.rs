use serde_json::Value;
use std::collections::HashMap;

pub type SharedProviderOptions = HashMap<String, HashMap<String, Value>>;
