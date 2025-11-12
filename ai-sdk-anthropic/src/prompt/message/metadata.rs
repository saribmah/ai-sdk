use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// Skill type for container skills.
///
/// Specifies whether a skill is built-in (Anthropic) or user-defined (custom).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SkillType {
    /// Built-in Anthropic skill
    Anthropic,
    /// User-defined custom skill
    Custom,
}

/// Skill information for container-based tools.
///
/// Represents a skill that has been loaded into a container for execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContainerSkill {
    /// Type of skill: either 'anthropic' (built-in) or 'custom' (user-defined)
    #[serde(rename = "type")]
    pub skill_type: SkillType,

    /// Skill ID (1-64 characters)
    #[serde(rename = "skillId")]
    pub skill_id: String,

    /// Skill version or 'latest' for most recent version (1-64 characters)
    pub version: String,
}

impl ContainerSkill {
    /// Creates a new container skill.
    ///
    /// # Arguments
    ///
    /// * `skill_type` - Type of skill (Anthropic or Custom)
    /// * `skill_id` - Skill identifier (1-64 characters)
    /// * `version` - Skill version or "latest" (1-64 characters)
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::metadata::{ContainerSkill, SkillType};
    ///
    /// let skill = ContainerSkill::new(
    ///     SkillType::Anthropic,
    ///     "python-interpreter",
    ///     "1.0.0"
    /// );
    /// assert_eq!(skill.skill_id, "python-interpreter");
    /// ```
    pub fn new(
        skill_type: SkillType,
        skill_id: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            skill_type,
            skill_id: skill_id.into(),
            version: version.into(),
        }
    }

    /// Creates a new Anthropic (built-in) skill.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::metadata::ContainerSkill;
    ///
    /// let skill = ContainerSkill::anthropic("python", "1.0.0");
    /// ```
    pub fn anthropic(skill_id: impl Into<String>, version: impl Into<String>) -> Self {
        Self::new(SkillType::Anthropic, skill_id, version)
    }

    /// Creates a new custom (user-defined) skill.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::metadata::ContainerSkill;
    ///
    /// let skill = ContainerSkill::custom("my-skill", "2.1.0");
    /// ```
    pub fn custom(skill_id: impl Into<String>, version: impl Into<String>) -> Self {
        Self::new(SkillType::Custom, skill_id, version)
    }

    /// Creates a skill with the latest version.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::metadata::{ContainerSkill, SkillType};
    ///
    /// let skill = ContainerSkill::latest(SkillType::Anthropic, "python");
    /// ```
    pub fn latest(skill_type: SkillType, skill_id: impl Into<String>) -> Self {
        Self::new(skill_type, skill_id, "latest")
    }
}

/// Container information for code execution tools.
///
/// Provides details about the container used for executing code in this request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContainerInfo {
    /// The time at which the container will expire (RFC3339 timestamp)
    #[serde(rename = "expiresAt")]
    pub expires_at: String,

    /// Identifier for the container used in this request
    pub id: String,

    /// Skills loaded in the container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skills: Option<Vec<ContainerSkill>>,
}

impl ContainerInfo {
    /// Creates a new container info.
    ///
    /// # Arguments
    ///
    /// * `expires_at` - RFC3339 timestamp for container expiration
    /// * `id` - Container identifier
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::metadata::ContainerInfo;
    ///
    /// let container = ContainerInfo::new(
    ///     "2024-12-31T23:59:59Z",
    ///     "container_abc123"
    /// );
    /// assert_eq!(container.id, "container_abc123");
    /// ```
    pub fn new(expires_at: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            expires_at: expires_at.into(),
            id: id.into(),
            skills: None,
        }
    }

    /// Sets the skills for this container.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::metadata::{ContainerInfo, ContainerSkill};
    ///
    /// let container = ContainerInfo::new("2024-12-31T23:59:59Z", "container_123")
    ///     .with_skills(vec![
    ///         ContainerSkill::anthropic("python", "1.0.0"),
    ///         ContainerSkill::custom("my-tool", "latest")
    ///     ]);
    /// ```
    pub fn with_skills(mut self, skills: Vec<ContainerSkill>) -> Self {
        self.skills = Some(skills);
        self
    }

    /// Adds a skill to the container.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::metadata::{ContainerInfo, ContainerSkill};
    ///
    /// let container = ContainerInfo::new("2024-12-31T23:59:59Z", "container_123")
    ///     .with_skill(ContainerSkill::anthropic("python", "1.0.0"));
    /// ```
    pub fn with_skill(mut self, skill: ContainerSkill) -> Self {
        if let Some(ref mut skills) = self.skills {
            skills.push(skill);
        } else {
            self.skills = Some(vec![skill]);
        }
        self
    }

    /// Removes all skills from the container.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::metadata::{ContainerInfo, ContainerSkill};
    ///
    /// let container = ContainerInfo::new("2024-12-31T23:59:59Z", "container_123")
    ///     .with_skill(ContainerSkill::anthropic("python", "1.0.0"))
    ///     .without_skills();
    ///
    /// assert!(container.skills.is_none());
    /// ```
    pub fn without_skills(mut self) -> Self {
        self.skills = None;
        self
    }
}

/// Anthropic message metadata.
///
/// Contains metadata about the API response, including token usage, stop sequences,
/// and container information for code execution tools.
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::prompt::message::metadata::AnthropicMessageMetadata;
/// use serde_json::json;
///
/// let metadata = AnthropicMessageMetadata::new(json!({
///     "input_tokens": 100,
///     "output_tokens": 50
/// }));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicMessageMetadata {
    /// Token usage information as a JSON object
    pub usage: JsonValue,

    /// Cache creation input tokens (deprecated, use value in usage object instead)
    ///
    /// This field is deprecated and will be removed in AI SDK 6.
    /// Use the value from the usage object instead.
    #[serde(rename = "cacheCreationInputTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_creation_input_tokens: Option<i64>,

    /// Stop sequence that caused generation to stop, if any
    #[serde(rename = "stopSequence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequence: Option<String>,

    /// Information about the container used in this request
    ///
    /// This will be non-null if a container tool (e.g., code execution) was used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<ContainerInfo>,
}

impl AnthropicMessageMetadata {
    /// Creates a new message metadata with usage information.
    ///
    /// # Arguments
    ///
    /// * `usage` - Token usage information as a JSON object
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::metadata::AnthropicMessageMetadata;
    /// use serde_json::json;
    ///
    /// let metadata = AnthropicMessageMetadata::new(json!({
    ///     "input_tokens": 150,
    ///     "output_tokens": 75,
    ///     "cache_creation_input_tokens": 50
    /// }));
    /// ```
    pub fn new(usage: JsonValue) -> Self {
        Self {
            usage,
            cache_creation_input_tokens: None,
            stop_sequence: None,
            container: None,
        }
    }

    /// Sets the cache creation input tokens.
    ///
    /// # Deprecated
    ///
    /// This field is deprecated. Use the value in the usage object instead.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::metadata::AnthropicMessageMetadata;
    /// use serde_json::json;
    ///
    /// let metadata = AnthropicMessageMetadata::new(json!({"input_tokens": 100}))
    ///     .with_cache_creation_input_tokens(50);
    /// ```
    #[deprecated(note = "Use value in usage object instead")]
    pub fn with_cache_creation_input_tokens(mut self, tokens: i64) -> Self {
        self.cache_creation_input_tokens = Some(tokens);
        self
    }

    /// Sets the stop sequence.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::metadata::AnthropicMessageMetadata;
    /// use serde_json::json;
    ///
    /// let metadata = AnthropicMessageMetadata::new(json!({"input_tokens": 100}))
    ///     .with_stop_sequence("END");
    /// ```
    pub fn with_stop_sequence(mut self, stop_sequence: impl Into<String>) -> Self {
        self.stop_sequence = Some(stop_sequence.into());
        self
    }

    /// Sets the container information.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::metadata::{
    ///     AnthropicMessageMetadata, ContainerInfo, ContainerSkill
    /// };
    /// use serde_json::json;
    ///
    /// let container = ContainerInfo::new("2024-12-31T23:59:59Z", "container_123")
    ///     .with_skill(ContainerSkill::anthropic("python", "1.0.0"));
    ///
    /// let metadata = AnthropicMessageMetadata::new(json!({"input_tokens": 100}))
    ///     .with_container(container);
    /// ```
    pub fn with_container(mut self, container: ContainerInfo) -> Self {
        self.container = Some(container);
        self
    }

    /// Removes the stop sequence.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::metadata::AnthropicMessageMetadata;
    /// use serde_json::json;
    ///
    /// let metadata = AnthropicMessageMetadata::new(json!({"input_tokens": 100}))
    ///     .with_stop_sequence("END")
    ///     .without_stop_sequence();
    ///
    /// assert!(metadata.stop_sequence.is_none());
    /// ```
    pub fn without_stop_sequence(mut self) -> Self {
        self.stop_sequence = None;
        self
    }

    /// Removes the container information.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::metadata::{
    ///     AnthropicMessageMetadata, ContainerInfo
    /// };
    /// use serde_json::json;
    ///
    /// let metadata = AnthropicMessageMetadata::new(json!({"input_tokens": 100}))
    ///     .with_container(ContainerInfo::new("2024-12-31T23:59:59Z", "container_123"))
    ///     .without_container();
    ///
    /// assert!(metadata.container.is_none());
    /// ```
    pub fn without_container(mut self) -> Self {
        self.container = None;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_skill_type_serialize() {
        let anthropic = SkillType::Anthropic;
        let custom = SkillType::Custom;

        assert_eq!(
            serde_json::to_value(&anthropic).unwrap(),
            json!("anthropic")
        );
        assert_eq!(serde_json::to_value(&custom).unwrap(), json!("custom"));
    }

    #[test]
    fn test_skill_type_deserialize() {
        let anthropic: SkillType = serde_json::from_value(json!("anthropic")).unwrap();
        let custom: SkillType = serde_json::from_value(json!("custom")).unwrap();

        assert_eq!(anthropic, SkillType::Anthropic);
        assert_eq!(custom, SkillType::Custom);
    }

    #[test]
    fn test_container_skill_new() {
        let skill = ContainerSkill::new(SkillType::Anthropic, "python", "1.0.0");

        assert_eq!(skill.skill_type, SkillType::Anthropic);
        assert_eq!(skill.skill_id, "python");
        assert_eq!(skill.version, "1.0.0");
    }

    #[test]
    fn test_container_skill_anthropic() {
        let skill = ContainerSkill::anthropic("python", "1.0.0");

        assert_eq!(skill.skill_type, SkillType::Anthropic);
        assert_eq!(skill.skill_id, "python");
    }

    #[test]
    fn test_container_skill_custom() {
        let skill = ContainerSkill::custom("my-tool", "2.1.0");

        assert_eq!(skill.skill_type, SkillType::Custom);
        assert_eq!(skill.skill_id, "my-tool");
        assert_eq!(skill.version, "2.1.0");
    }

    #[test]
    fn test_container_skill_latest() {
        let skill = ContainerSkill::latest(SkillType::Anthropic, "python");

        assert_eq!(skill.version, "latest");
    }

    #[test]
    fn test_container_skill_serialize() {
        let skill = ContainerSkill::anthropic("python", "1.0.0");
        let json = serde_json::to_value(&skill).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "anthropic",
                "skillId": "python",
                "version": "1.0.0"
            })
        );
    }

    #[test]
    fn test_container_skill_deserialize() {
        let json = json!({
            "type": "custom",
            "skillId": "my-skill",
            "version": "2.0.0"
        });

        let skill: ContainerSkill = serde_json::from_value(json).unwrap();

        assert_eq!(skill.skill_type, SkillType::Custom);
        assert_eq!(skill.skill_id, "my-skill");
        assert_eq!(skill.version, "2.0.0");
    }

    #[test]
    fn test_container_info_new() {
        let container = ContainerInfo::new("2024-12-31T23:59:59Z", "container_123");

        assert_eq!(container.expires_at, "2024-12-31T23:59:59Z");
        assert_eq!(container.id, "container_123");
        assert!(container.skills.is_none());
    }

    #[test]
    fn test_container_info_with_skills() {
        let skills = vec![
            ContainerSkill::anthropic("python", "1.0.0"),
            ContainerSkill::custom("my-tool", "latest"),
        ];
        let container =
            ContainerInfo::new("2024-12-31T23:59:59Z", "container_123").with_skills(skills);

        assert_eq!(container.skills.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_container_info_with_skill() {
        let container = ContainerInfo::new("2024-12-31T23:59:59Z", "container_123")
            .with_skill(ContainerSkill::anthropic("python", "1.0.0"))
            .with_skill(ContainerSkill::custom("tool", "2.0.0"));

        assert_eq!(container.skills.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_container_info_without_skills() {
        let container = ContainerInfo::new("2024-12-31T23:59:59Z", "container_123")
            .with_skill(ContainerSkill::anthropic("python", "1.0.0"))
            .without_skills();

        assert!(container.skills.is_none());
    }

    #[test]
    fn test_container_info_serialize_without_skills() {
        let container = ContainerInfo::new("2024-12-31T23:59:59Z", "container_123");
        let json = serde_json::to_value(&container).unwrap();

        assert_eq!(
            json,
            json!({
                "expiresAt": "2024-12-31T23:59:59Z",
                "id": "container_123"
            })
        );
    }

    #[test]
    fn test_container_info_serialize_with_skills() {
        let container = ContainerInfo::new("2024-12-31T23:59:59Z", "container_123")
            .with_skills(vec![ContainerSkill::anthropic("python", "1.0.0")]);
        let json = serde_json::to_value(&container).unwrap();

        assert_eq!(
            json,
            json!({
                "expiresAt": "2024-12-31T23:59:59Z",
                "id": "container_123",
                "skills": [
                    {
                        "type": "anthropic",
                        "skillId": "python",
                        "version": "1.0.0"
                    }
                ]
            })
        );
    }

    #[test]
    fn test_container_info_deserialize() {
        let json = json!({
            "expiresAt": "2024-12-31T23:59:59Z",
            "id": "container_456",
            "skills": [
                {
                    "type": "anthropic",
                    "skillId": "python",
                    "version": "1.0.0"
                }
            ]
        });

        let container: ContainerInfo = serde_json::from_value(json).unwrap();

        assert_eq!(container.id, "container_456");
        assert_eq!(container.skills.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_metadata_new() {
        let usage = json!({"input_tokens": 100, "output_tokens": 50});
        let metadata = AnthropicMessageMetadata::new(usage.clone());

        assert_eq!(metadata.usage, usage);
        assert!(metadata.cache_creation_input_tokens.is_none());
        assert!(metadata.stop_sequence.is_none());
        assert!(metadata.container.is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn test_metadata_with_cache_creation_tokens() {
        let metadata = AnthropicMessageMetadata::new(json!({"input_tokens": 100}))
            .with_cache_creation_input_tokens(50);

        assert_eq!(metadata.cache_creation_input_tokens, Some(50));
    }

    #[test]
    fn test_metadata_with_stop_sequence() {
        let metadata =
            AnthropicMessageMetadata::new(json!({"input_tokens": 100})).with_stop_sequence("END");

        assert_eq!(metadata.stop_sequence, Some("END".to_string()));
    }

    #[test]
    fn test_metadata_with_container() {
        let container = ContainerInfo::new("2024-12-31T23:59:59Z", "container_123");
        let metadata =
            AnthropicMessageMetadata::new(json!({"input_tokens": 100})).with_container(container);

        assert!(metadata.container.is_some());
        assert_eq!(metadata.container.as_ref().unwrap().id, "container_123");
    }

    #[test]
    fn test_metadata_without_stop_sequence() {
        let metadata = AnthropicMessageMetadata::new(json!({"input_tokens": 100}))
            .with_stop_sequence("END")
            .without_stop_sequence();

        assert!(metadata.stop_sequence.is_none());
    }

    #[test]
    fn test_metadata_without_container() {
        let metadata = AnthropicMessageMetadata::new(json!({"input_tokens": 100}))
            .with_container(ContainerInfo::new("2024-12-31T23:59:59Z", "container_123"))
            .without_container();

        assert!(metadata.container.is_none());
    }

    #[test]
    fn test_metadata_serialize_minimal() {
        let metadata = AnthropicMessageMetadata::new(json!({
            "input_tokens": 100,
            "output_tokens": 50
        }));
        let json = serde_json::to_value(&metadata).unwrap();

        assert_eq!(
            json,
            json!({
                "usage": {
                    "input_tokens": 100,
                    "output_tokens": 50
                }
            })
        );
    }

    #[test]
    #[allow(deprecated)]
    fn test_metadata_serialize_full() {
        let container = ContainerInfo::new("2024-12-31T23:59:59Z", "container_123")
            .with_skill(ContainerSkill::anthropic("python", "1.0.0"));

        let metadata = AnthropicMessageMetadata::new(json!({"input_tokens": 100}))
            .with_cache_creation_input_tokens(50)
            .with_stop_sequence("STOP")
            .with_container(container);

        let json = serde_json::to_value(&metadata).unwrap();

        assert!(json["usage"].is_object());
        assert_eq!(json["cacheCreationInputTokens"], 50);
        assert_eq!(json["stopSequence"], "STOP");
        assert!(json["container"].is_object());
    }

    #[test]
    fn test_metadata_deserialize() {
        let json = json!({
            "usage": {
                "input_tokens": 100,
                "output_tokens": 50
            },
            "cacheCreationInputTokens": 25,
            "stopSequence": "END",
            "container": {
                "expiresAt": "2024-12-31T23:59:59Z",
                "id": "container_789"
            }
        });

        let metadata: AnthropicMessageMetadata = serde_json::from_value(json).unwrap();

        assert_eq!(metadata.cache_creation_input_tokens, Some(25));
        assert_eq!(metadata.stop_sequence, Some("END".to_string()));
        assert_eq!(metadata.container.as_ref().unwrap().id, "container_789");
    }

    #[test]
    fn test_equality() {
        let metadata1 = AnthropicMessageMetadata::new(json!({"input_tokens": 100}));
        let metadata2 = AnthropicMessageMetadata::new(json!({"input_tokens": 100}));
        let metadata3 = AnthropicMessageMetadata::new(json!({"input_tokens": 200}));

        assert_eq!(metadata1, metadata2);
        assert_ne!(metadata1, metadata3);
    }

    #[test]
    fn test_clone() {
        let metadata1 = AnthropicMessageMetadata::new(json!({"input_tokens": 100}))
            .with_stop_sequence("END")
            .with_container(ContainerInfo::new("2024-12-31T23:59:59Z", "container_123"));
        let metadata2 = metadata1.clone();

        assert_eq!(metadata1, metadata2);
    }

    #[test]
    fn test_roundtrip_serialization() {
        let container =
            ContainerInfo::new("2024-12-31T23:59:59Z", "container_123").with_skills(vec![
                ContainerSkill::anthropic("python", "1.0.0"),
                ContainerSkill::custom("my-tool", "latest"),
            ]);

        let original = AnthropicMessageMetadata::new(json!({
            "input_tokens": 150,
            "output_tokens": 75,
            "cache_creation_input_tokens": 50
        }))
        .with_stop_sequence("STOP")
        .with_container(container);

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicMessageMetadata = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_builder_chaining() {
        let metadata = AnthropicMessageMetadata::new(json!({"input_tokens": 100}))
            .with_stop_sequence("END")
            .without_stop_sequence()
            .with_container(ContainerInfo::new("2024-12-31T23:59:59Z", "container_123"))
            .without_container();

        assert!(metadata.stop_sequence.is_none());
        assert!(metadata.container.is_none());
    }
}
