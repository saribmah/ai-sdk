use crate::prompt::message::tool::definition::Tool;
use serde_json::Value;
use std::collections::HashMap;

/// A set of tools indexed by their names.
///
/// The key is the tool name that will be used by the language model.
/// Each tool is a `Tool<Value, Value>` which can accept and return any JSON-serializable data.
///
/// # Example
///
/// ```
/// use ai_sdk_core::ToolSet;
/// use ai_sdk_core::message::tool::definition::Tool;
/// use serde_json::json;
///
/// let mut tools = ToolSet::new();
/// tools.insert("get_weather".to_string(), Tool::function(json!({
///     "type": "object",
///     "properties": {
///         "city": { "type": "string" },
///         "units": { "type": "string", "enum": ["celsius", "fahrenheit"] }
///     },
///     "required": ["city"]
/// })));
///
/// tools.insert("calculate".to_string(), Tool::function(json!({
///     "type": "object",
///     "properties": {
///         "expression": { "type": "string" }
///     }
/// })));
///
/// // Access a tool by name
/// if let Some(tool) = tools.get("get_weather") {
///     // Use the tool...
/// }
///
/// // Check if a tool exists
/// assert!(tools.contains_key("calculate"));
///
/// // Get all tool names
/// let tool_names: Vec<&String> = tools.keys().collect();
/// ```
pub type ToolSet = HashMap<String, Tool<Value, Value>>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_toolset_creation() {
        let mut tools = ToolSet::new();
        assert_eq!(tools.len(), 0);
        assert!(tools.is_empty());
    }

    #[test]
    fn test_toolset_insert_and_get() {
        let mut tools = ToolSet::new();

        let tool = Tool::function(json!({
            "type": "object",
            "properties": {
                "city": { "type": "string" }
            }
        }));

        tools.insert("get_weather".to_string(), tool);

        assert_eq!(tools.len(), 1);
        assert!(tools.contains_key("get_weather"));
        assert!(tools.get("get_weather").is_some());
    }

    #[test]
    fn test_toolset_multiple_tools() {
        let mut tools = ToolSet::new();

        tools.insert(
            "tool1".to_string(),
            Tool::function(json!({"type": "object"})),
        );
        tools.insert(
            "tool2".to_string(),
            Tool::function(json!({"type": "object"})),
        );
        tools.insert(
            "tool3".to_string(),
            Tool::function(json!({"type": "object"})),
        );

        assert_eq!(tools.len(), 3);

        let keys: Vec<&String> = tools.keys().collect();
        assert!(keys.contains(&&"tool1".to_string()));
        assert!(keys.contains(&&"tool2".to_string()));
        assert!(keys.contains(&&"tool3".to_string()));
    }

    #[test]
    fn test_toolset_remove() {
        let mut tools = ToolSet::new();

        tools.insert(
            "temp_tool".to_string(),
            Tool::function(json!({"type": "object"})),
        );

        assert!(tools.contains_key("temp_tool"));

        tools.remove("temp_tool");

        assert!(!tools.contains_key("temp_tool"));
        assert_eq!(tools.len(), 0);
    }

    #[test]
    fn test_toolset_contains_key() {
        let mut tools = ToolSet::new();

        tools.insert(
            "existing_tool".to_string(),
            Tool::function(json!({"type": "object"})),
        );

        assert!(tools.contains_key("existing_tool"));
        assert!(!tools.contains_key("nonexistent_tool"));
    }

    #[test]
    fn test_toolset_iteration() {
        let mut tools = ToolSet::new();

        tools.insert("a".to_string(), Tool::function(json!({"type": "object"})));
        tools.insert("b".to_string(), Tool::function(json!({"type": "object"})));

        let mut count = 0;
        for (name, _tool) in &tools {
            count += 1;
            assert!(name == "a" || name == "b");
        }
        assert_eq!(count, 2);
    }

    #[test]
    fn test_toolset_clear() {
        let mut tools = ToolSet::new();

        tools.insert(
            "tool1".to_string(),
            Tool::function(json!({"type": "object"})),
        );
        tools.insert(
            "tool2".to_string(),
            Tool::function(json!({"type": "object"})),
        );

        assert_eq!(tools.len(), 2);

        tools.clear();

        assert_eq!(tools.len(), 0);
        assert!(tools.is_empty());
    }
}
