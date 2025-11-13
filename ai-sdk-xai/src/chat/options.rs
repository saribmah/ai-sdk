use serde::{Deserialize, Serialize};

/// xAI chat model IDs.
///
/// See <https://console.x.ai> for the full list of available models.
pub type XaiChatModelId = String;

/// xAI-specific provider options.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XaiProviderOptions {
    /// Reasoning effort level ("low" or "high").
    /// Controls the depth of reasoning for models that support it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<String>,

    /// Whether to enable parallel function calling during tool use.
    /// When true, the model can call multiple functions in parallel.
    /// When false, the model will call functions sequentially.
    /// Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_function_calling: Option<bool>,

    /// Search parameters for web/X search integration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_parameters: Option<SearchParameters>,
}

/// Search parameters for xAI's integrated search functionality.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchParameters {
    /// Search mode preference:
    /// - "off": disables search completely
    /// - "auto": model decides whether to search (default)
    /// - "on": always enables search
    pub mode: String,

    /// Whether to return citations in the response (defaults to true).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_citations: Option<bool>,

    /// Start date for search data (ISO8601 format: YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_date: Option<String>,

    /// End date for search data (ISO8601 format: YYYY-MM-DD).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_date: Option<String>,

    /// Maximum number of search results to consider (1-50, defaults to 20).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_search_results: Option<u32>,

    /// Data sources to search from (defaults to ["web", "x"] if not specified).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<SearchSource>>,
}

/// Search source types for xAI search.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SearchSource {
    /// Web search source.
    Web {
        /// Two-letter country code (e.g., "US", "GB").
        #[serde(skip_serializing_if = "Option::is_none")]
        country: Option<String>,

        /// List of websites to exclude from search (max 5).
        #[serde(skip_serializing_if = "Option::is_none")]
        excluded_websites: Option<Vec<String>>,

        /// List of websites to restrict search to (max 5).
        #[serde(skip_serializing_if = "Option::is_none")]
        allowed_websites: Option<Vec<String>>,

        /// Enable safe search filtering.
        #[serde(skip_serializing_if = "Option::is_none")]
        safe_search: Option<bool>,
    },

    /// X (Twitter) search source.
    X {
        /// List of X handles to exclude from search.
        #[serde(skip_serializing_if = "Option::is_none")]
        excluded_x_handles: Option<Vec<String>>,

        /// List of X handles to include in search.
        #[serde(skip_serializing_if = "Option::is_none")]
        included_x_handles: Option<Vec<String>>,

        /// Minimum favorite count for posts.
        #[serde(skip_serializing_if = "Option::is_none")]
        post_favorite_count: Option<u64>,

        /// Minimum view count for posts.
        #[serde(skip_serializing_if = "Option::is_none")]
        post_view_count: Option<u64>,
    },

    /// News search source.
    News {
        /// Two-letter country code (e.g., "US", "GB").
        #[serde(skip_serializing_if = "Option::is_none")]
        country: Option<String>,

        /// List of news websites to exclude from search (max 5).
        #[serde(skip_serializing_if = "Option::is_none")]
        excluded_websites: Option<Vec<String>>,

        /// Enable safe search filtering.
        #[serde(skip_serializing_if = "Option::is_none")]
        safe_search: Option<bool>,
    },

    /// RSS feed source.
    Rss {
        /// RSS feed URLs (currently supports max 1).
        links: Vec<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_options_serialization() {
        let options = XaiProviderOptions {
            reasoning_effort: Some("high".to_string()),
            parallel_function_calling: Some(true),
            search_parameters: None,
        };

        let json = serde_json::to_string(&options).unwrap();
        assert!(json.contains("reasoningEffort"));
        assert!(json.contains("high"));
    }

    #[test]
    fn test_search_parameters_serialization() {
        let params = SearchParameters {
            mode: "on".to_string(),
            return_citations: Some(true),
            from_date: Some("2024-01-01".to_string()),
            to_date: Some("2024-12-31".to_string()),
            max_search_results: Some(10),
            sources: Some(vec![SearchSource::Web {
                country: Some("US".to_string()),
                excluded_websites: None,
                allowed_websites: None,
                safe_search: Some(true),
            }]),
        };

        let json = serde_json::to_string(&params).unwrap();
        assert!(json.contains("mode"));
        assert!(json.contains("\"on\""));
        assert!(json.contains("return_citations"));
    }

    #[test]
    fn test_search_source_web() {
        let source = SearchSource::Web {
            country: Some("US".to_string()),
            excluded_websites: Some(vec!["example.com".to_string()]),
            allowed_websites: None,
            safe_search: Some(true),
        };

        let json = serde_json::to_value(&source).unwrap();
        assert_eq!(json["type"], "web");
        assert_eq!(json["country"], "US");
    }

    #[test]
    fn test_search_source_x() {
        let source = SearchSource::X {
            excluded_x_handles: None,
            included_x_handles: Some(vec!["@example".to_string()]),
            post_favorite_count: Some(100),
            post_view_count: Some(1000),
        };

        let json = serde_json::to_value(&source).unwrap();
        assert_eq!(json["type"], "x");
        assert_eq!(json["post_favorite_count"], 100);
    }

    #[test]
    fn test_search_source_rss() {
        let source = SearchSource::Rss {
            links: vec!["https://example.com/feed.xml".to_string()],
        };

        let json = serde_json::to_value(&source).unwrap();
        assert_eq!(json["type"], "rss");
        assert!(json["links"].is_array());
    }
}
