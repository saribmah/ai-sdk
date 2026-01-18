use serde::{Deserialize, Serialize};

/// Anthropic Messages API model identifiers.
///
/// See: <https://docs.claude.com/en/docs/about-claude/models/overview>
pub type AnthropicMessagesModelId = String;

/// Anthropic file part provider options for document-specific features.
/// These options apply to individual file parts (documents).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicFilePartProviderOptions {
    /// Citation configuration for this document.
    /// When enabled, this document will generate citations in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citations: Option<CitationConfig>,

    /// Custom title for the document.
    /// If not provided, the filename will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Context about the document that will be passed to the model
    /// but not used towards cited content.
    /// Useful for storing document metadata as text or stringified JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
}

impl Default for AnthropicFilePartProviderOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl AnthropicFilePartProviderOptions {
    /// Creates a new instance with default values.
    pub fn new() -> Self {
        Self {
            citations: None,
            title: None,
            context: None,
        }
    }

    /// Enables citations for this document.
    pub fn with_citations(mut self, enabled: bool) -> Self {
        self.citations = Some(CitationConfig { enabled });
        self
    }

    /// Sets a custom title for the document.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets context about the document.
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }
}

/// Citation configuration for documents.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitationConfig {
    /// Enable citations for this document
    pub enabled: bool,
}

/// Anthropic provider options for customizing model behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AnthropicProviderOptions {
    /// Whether to send reasoning information.
    #[serde(skip_serializing_if = "Option::is_none", rename = "sendReasoning")]
    pub send_reasoning: Option<bool>,

    /// Thinking configuration for extended thinking mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<ThinkingConfig>,

    /// Whether to disable parallel function calling during tool use. Default is false.
    /// When set to true, Claude will use at most one tool per response.
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "disableParallelToolUse"
    )]
    pub disable_parallel_tool_use: Option<bool>,

    /// Cache control settings for this message.
    /// See <https://docs.anthropic.com/en/docs/build-with-claude/prompt-caching>>
    #[serde(skip_serializing_if = "Option::is_none", rename = "cacheControl")]
    pub cache_control: Option<CacheControl>,

    /// MCP (Model Context Protocol) server configurations.
    #[serde(skip_serializing_if = "Option::is_none", rename = "mcpServers")]
    pub mcp_servers: Option<Vec<McpServer>>,

    /// Agent Skills configuration. Skills enable Claude to perform specialized tasks
    /// like document processing (PPTX, DOCX, PDF, XLSX) and data analysis.
    /// Requires code execution tool to be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<ContainerConfig>,

    /// Whether to enable tool streaming (and structured output streaming).
    ///
    /// When set to false, the model will return all tool calls and results
    /// at once after a delay.
    ///
    /// @default true
    #[serde(skip_serializing_if = "Option::is_none", rename = "toolStreaming")]
    pub tool_streaming: Option<bool>,
}

impl AnthropicProviderOptions {
    /// Creates a new instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether to send reasoning information.
    pub fn with_send_reasoning(mut self, send_reasoning: bool) -> Self {
        self.send_reasoning = Some(send_reasoning);
        self
    }

    /// Sets the thinking configuration.
    pub fn with_thinking(mut self, thinking: ThinkingConfig) -> Self {
        self.thinking = Some(thinking);
        self
    }

    /// Sets whether to disable parallel tool use.
    pub fn with_disable_parallel_tool_use(mut self, disable: bool) -> Self {
        self.disable_parallel_tool_use = Some(disable);
        self
    }

    /// Sets cache control configuration.
    pub fn with_cache_control(mut self, cache_control: CacheControl) -> Self {
        self.cache_control = Some(cache_control);
        self
    }

    /// Sets MCP server configurations.
    pub fn with_mcp_servers(mut self, servers: Vec<McpServer>) -> Self {
        self.mcp_servers = Some(servers);
        self
    }

    /// Adds a single MCP server configuration.
    pub fn add_mcp_server(mut self, server: McpServer) -> Self {
        if let Some(ref mut servers) = self.mcp_servers {
            servers.push(server);
        } else {
            self.mcp_servers = Some(vec![server]);
        }
        self
    }

    /// Sets container configuration for agent skills.
    pub fn with_container(mut self, container: ContainerConfig) -> Self {
        self.container = Some(container);
        self
    }

    /// Sets whether to enable tool streaming.
    pub fn with_tool_streaming(mut self, enabled: bool) -> Self {
        self.tool_streaming = Some(enabled);
        self
    }
}

/// Thinking configuration for extended thinking mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThinkingConfig {
    /// Type of thinking mode
    #[serde(rename = "type")]
    pub thinking_type: ThinkingType,

    /// Budget for thinking tokens
    #[serde(skip_serializing_if = "Option::is_none", rename = "budgetTokens")]
    pub budget_tokens: Option<u64>,
}

impl ThinkingConfig {
    /// Creates a new thinking configuration.
    pub fn new(thinking_type: ThinkingType) -> Self {
        Self {
            thinking_type,
            budget_tokens: None,
        }
    }

    /// Creates an enabled thinking configuration.
    pub fn enabled() -> Self {
        Self::new(ThinkingType::Enabled)
    }

    /// Creates a disabled thinking configuration.
    pub fn disabled() -> Self {
        Self::new(ThinkingType::Disabled)
    }

    /// Sets the budget for thinking tokens.
    pub fn with_budget_tokens(mut self, tokens: u64) -> Self {
        self.budget_tokens = Some(tokens);
        self
    }
}

/// Type of thinking mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThinkingType {
    /// Extended thinking enabled
    Enabled,
    /// Extended thinking disabled
    Disabled,
}

/// Cache control settings for prompt caching.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheControl {
    /// Type of cache control (always "ephemeral" for now)
    #[serde(rename = "type")]
    pub cache_type: String,

    /// Time-to-live for the cache
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<CacheTtl>,
}

impl CacheControl {
    /// Creates a new ephemeral cache control.
    pub fn ephemeral() -> Self {
        Self {
            cache_type: "ephemeral".to_string(),
            ttl: None,
        }
    }

    /// Sets the TTL for the cache.
    pub fn with_ttl(mut self, ttl: CacheTtl) -> Self {
        self.ttl = Some(ttl);
        self
    }
}

/// Time-to-live options for cache control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CacheTtl {
    /// 5 minutes TTL
    #[serde(rename = "5m")]
    FiveMinutes,
    /// 1 hour TTL
    #[serde(rename = "1h")]
    OneHour,
}

/// MCP (Model Context Protocol) server configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct McpServer {
    /// Type of server (currently only "url" is supported)
    #[serde(rename = "type")]
    pub server_type: String,

    /// Name of the MCP server
    pub name: String,

    /// URL of the MCP server
    pub url: String,

    /// Optional authorization token
    #[serde(skip_serializing_if = "Option::is_none", rename = "authorizationToken")]
    pub authorization_token: Option<String>,

    /// Tool configuration for this server
    #[serde(skip_serializing_if = "Option::is_none", rename = "toolConfiguration")]
    pub tool_configuration: Option<ToolConfiguration>,
}

impl McpServer {
    /// Creates a new MCP server configuration.
    pub fn new(name: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            server_type: "url".to_string(),
            name: name.into(),
            url: url.into(),
            authorization_token: None,
            tool_configuration: None,
        }
    }

    /// Sets the authorization token.
    pub fn with_authorization_token(mut self, token: impl Into<String>) -> Self {
        self.authorization_token = Some(token.into());
        self
    }

    /// Sets the tool configuration.
    pub fn with_tool_configuration(mut self, config: ToolConfiguration) -> Self {
        self.tool_configuration = Some(config);
        self
    }
}

/// Tool configuration for MCP servers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolConfiguration {
    /// Whether tools from this server are enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// List of allowed tool names
    #[serde(skip_serializing_if = "Option::is_none", rename = "allowedTools")]
    pub allowed_tools: Option<Vec<String>>,
}

impl ToolConfiguration {
    /// Creates a new tool configuration.
    pub fn new() -> Self {
        Self {
            enabled: None,
            allowed_tools: None,
        }
    }

    /// Sets whether tools are enabled.
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }

    /// Sets allowed tools.
    pub fn with_allowed_tools(mut self, tools: Vec<String>) -> Self {
        self.allowed_tools = Some(tools);
        self
    }
}

impl Default for ToolConfiguration {
    fn default() -> Self {
        Self::new()
    }
}

/// Container configuration for agent skills.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContainerConfig {
    /// Container ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// List of skills to enable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skills: Option<Vec<Skill>>,
}

impl ContainerConfig {
    /// Creates a new container configuration.
    pub fn new() -> Self {
        Self {
            id: None,
            skills: None,
        }
    }

    /// Sets the container ID.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the skills.
    pub fn with_skills(mut self, skills: Vec<Skill>) -> Self {
        self.skills = Some(skills);
        self
    }

    /// Adds a single skill.
    pub fn add_skill(mut self, skill: Skill) -> Self {
        if let Some(ref mut skills) = self.skills {
            skills.push(skill);
        } else {
            self.skills = Some(vec![skill]);
        }
        self
    }
}

impl Default for ContainerConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Skill configuration for agent tasks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Skill {
    /// Type of skill
    #[serde(rename = "type")]
    pub skill_type: SkillType,

    /// Skill identifier
    #[serde(rename = "skillId")]
    pub skill_id: String,

    /// Optional skill version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl Skill {
    /// Creates a new anthropic skill.
    pub fn anthropic(skill_id: impl Into<String>) -> Self {
        Self {
            skill_type: SkillType::Anthropic,
            skill_id: skill_id.into(),
            version: None,
        }
    }

    /// Creates a new custom skill.
    pub fn custom(skill_id: impl Into<String>) -> Self {
        Self {
            skill_type: SkillType::Custom,
            skill_id: skill_id.into(),
            version: None,
        }
    }

    /// Sets the skill version.
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }
}

/// Type of skill.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SkillType {
    /// Anthropic-provided skill
    Anthropic,
    /// Custom skill
    Custom,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_file_part_options_default() {
        let options = AnthropicFilePartProviderOptions::new();
        assert!(options.citations.is_none());
        assert!(options.title.is_none());
        assert!(options.context.is_none());
    }

    #[test]
    fn test_file_part_options_builder() {
        let options = AnthropicFilePartProviderOptions::new()
            .with_citations(true)
            .with_title("My Document")
            .with_context("Important context");

        assert!(options.citations.as_ref().unwrap().enabled);
        assert_eq!(options.title.as_ref().unwrap(), "My Document");
        assert_eq!(options.context.as_ref().unwrap(), "Important context");
    }

    #[test]
    fn test_provider_options_default() {
        let options = AnthropicProviderOptions::new();
        assert!(options.send_reasoning.is_none());
        assert!(options.thinking.is_none());
        assert!(options.disable_parallel_tool_use.is_none());
    }

    #[test]
    fn test_provider_options_builder() {
        let options = AnthropicProviderOptions::new()
            .with_send_reasoning(true)
            .with_thinking(ThinkingConfig::enabled().with_budget_tokens(1000))
            .with_disable_parallel_tool_use(false)
            .with_tool_streaming(true);

        assert_eq!(options.send_reasoning, Some(true));
        assert_eq!(
            options.thinking.as_ref().unwrap().thinking_type,
            ThinkingType::Enabled
        );
        assert_eq!(options.thinking.as_ref().unwrap().budget_tokens, Some(1000));
        assert_eq!(options.disable_parallel_tool_use, Some(false));
        assert_eq!(options.tool_streaming, Some(true));
    }

    #[test]
    fn test_thinking_config() {
        let config = ThinkingConfig::enabled().with_budget_tokens(2000);
        assert_eq!(config.thinking_type, ThinkingType::Enabled);
        assert_eq!(config.budget_tokens, Some(2000));

        let config = ThinkingConfig::disabled();
        assert_eq!(config.thinking_type, ThinkingType::Disabled);
        assert!(config.budget_tokens.is_none());
    }

    #[test]
    fn test_cache_control() {
        let cache = CacheControl::ephemeral().with_ttl(CacheTtl::FiveMinutes);
        assert_eq!(cache.cache_type, "ephemeral");
        assert_eq!(cache.ttl, Some(CacheTtl::FiveMinutes));
    }

    #[test]
    fn test_mcp_server() {
        let server = McpServer::new("my-server", "https://example.com")
            .with_authorization_token("token123")
            .with_tool_configuration(
                ToolConfiguration::new()
                    .with_enabled(true)
                    .with_allowed_tools(vec!["tool1".to_string(), "tool2".to_string()]),
            );

        assert_eq!(server.name, "my-server");
        assert_eq!(server.url, "https://example.com");
        assert_eq!(server.authorization_token, Some("token123".to_string()));
        assert!(server.tool_configuration.is_some());
    }

    #[test]
    fn test_container_config() {
        let container = ContainerConfig::new()
            .with_id("container-123")
            .add_skill(Skill::anthropic("document-processing"))
            .add_skill(Skill::custom("my-skill").with_version("1.0"));

        assert_eq!(container.id, Some("container-123".to_string()));
        assert_eq!(container.skills.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_skill_creation() {
        let skill = Skill::anthropic("data-analysis").with_version("2.0");
        assert_eq!(skill.skill_type, SkillType::Anthropic);
        assert_eq!(skill.skill_id, "data-analysis");
        assert_eq!(skill.version, Some("2.0".to_string()));

        let skill = Skill::custom("custom-processor");
        assert_eq!(skill.skill_type, SkillType::Custom);
        assert_eq!(skill.skill_id, "custom-processor");
    }

    #[test]
    fn test_serialize_provider_options() {
        let options = AnthropicProviderOptions::new()
            .with_send_reasoning(true)
            .with_disable_parallel_tool_use(true);

        let json = serde_json::to_value(&options).unwrap();
        assert_eq!(json["sendReasoning"], json!(true));
        assert_eq!(json["disableParallelToolUse"], json!(true));
    }

    #[test]
    fn test_serialize_thinking_config() {
        let config = ThinkingConfig::enabled().with_budget_tokens(1500);
        let json = serde_json::to_value(&config).unwrap();

        assert_eq!(json["type"], json!("enabled"));
        assert_eq!(json["budgetTokens"], json!(1500));
    }

    #[test]
    fn test_serialize_cache_ttl() {
        let ttl = CacheTtl::FiveMinutes;
        let json = serde_json::to_value(ttl).unwrap();
        assert_eq!(json, json!("5m"));

        let ttl = CacheTtl::OneHour;
        let json = serde_json::to_value(ttl).unwrap();
        assert_eq!(json, json!("1h"));
    }

    #[test]
    fn test_deserialize_provider_options() {
        let json = json!({
            "sendReasoning": true,
            "thinking": {
                "type": "enabled",
                "budgetTokens": 1000
            },
            "disableParallelToolUse": false
        });

        let options: AnthropicProviderOptions = serde_json::from_value(json).unwrap();
        assert_eq!(options.send_reasoning, Some(true));
        assert!(options.thinking.is_some());
        assert_eq!(options.disable_parallel_tool_use, Some(false));
    }

    #[test]
    fn test_roundtrip_serialization() {
        let original = AnthropicProviderOptions::new()
            .with_thinking(ThinkingConfig::enabled().with_budget_tokens(2000))
            .with_cache_control(CacheControl::ephemeral().with_ttl(CacheTtl::OneHour))
            .add_mcp_server(McpServer::new("server1", "https://example.com"));

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicProviderOptions = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }
}
