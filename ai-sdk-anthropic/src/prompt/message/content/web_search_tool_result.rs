use serde::{Deserialize, Serialize};

use crate::prompt::message::cache_control::AnthropicCacheControl;

/// Web search result item.
///
/// Represents a single search result from a web search tool execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebSearchResultItem {
    /// URL of the search result
    pub url: String,

    /// Title of the search result
    pub title: String,

    /// Age of the page (e.g., "2 days ago", "1 week ago"), or null if unknown
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_age: Option<String>,

    /// Encrypted content from the search result
    pub encrypted_content: String,

    /// Type of the search result
    #[serde(rename = "type")]
    pub result_type: String,
}

impl WebSearchResultItem {
    /// Creates a new web search result item.
    ///
    /// # Arguments
    ///
    /// * `url` - URL of the search result
    /// * `title` - Title of the search result
    /// * `encrypted_content` - Encrypted content from the search result
    /// * `result_type` - Type of the search result
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::web_search_tool_result::WebSearchResultItem;
    ///
    /// let result = WebSearchResultItem::new(
    ///     "https://example.com",
    ///     "Example Page",
    ///     "encrypted_content_here",
    ///     "web_page"
    /// );
    /// ```
    pub fn new(
        url: impl Into<String>,
        title: impl Into<String>,
        encrypted_content: impl Into<String>,
        result_type: impl Into<String>,
    ) -> Self {
        Self {
            url: url.into(),
            title: title.into(),
            page_age: None,
            encrypted_content: encrypted_content.into(),
            result_type: result_type.into(),
        }
    }

    /// Sets the page age.
    ///
    /// # Arguments
    ///
    /// * `page_age` - Age of the page (e.g., "2 days ago")
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::web_search_tool_result::WebSearchResultItem;
    ///
    /// let result = WebSearchResultItem::new("url", "title", "content", "type")
    ///     .with_page_age("3 days ago");
    /// ```
    pub fn with_page_age(mut self, page_age: impl Into<String>) -> Self {
        self.page_age = Some(page_age.into());
        self
    }

    /// Removes the page age.
    pub fn without_page_age(mut self) -> Self {
        self.page_age = None;
        self
    }

    /// Updates the URL.
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }

    /// Updates the title.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Updates the encrypted content.
    pub fn with_encrypted_content(mut self, encrypted_content: impl Into<String>) -> Self {
        self.encrypted_content = encrypted_content.into();
        self
    }

    /// Updates the result type.
    pub fn with_result_type(mut self, result_type: impl Into<String>) -> Self {
        self.result_type = result_type.into();
        self
    }
}

/// Anthropic web search tool result content block.
///
/// Represents the result of a web search tool execution. Contains an array of
/// search result items with URLs, titles, encrypted content, and metadata.
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::prompt::message::content::web_search_tool_result::{
///     AnthropicWebSearchToolResultContent, WebSearchResultItem
/// };
///
/// let results = vec![
///     WebSearchResultItem::new(
///         "https://example.com/1",
///         "First Result",
///         "encrypted_content_1",
///         "web_page"
///     ).with_page_age("1 day ago"),
///     WebSearchResultItem::new(
///         "https://example.com/2",
///         "Second Result",
///         "encrypted_content_2",
///         "web_page"
///     )
/// ];
///
/// let web_search_result = AnthropicWebSearchToolResultContent::new(
///     "server_toolu_123",
///     results
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicWebSearchToolResultContent {
    /// The type of content (always "web_search_tool_result")
    #[serde(rename = "type")]
    pub content_type: String,

    /// ID of the tool use this is a result for
    pub tool_use_id: String,

    /// Array of search result items
    pub content: Vec<WebSearchResultItem>,

    /// Optional cache control configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<AnthropicCacheControl>,
}

impl AnthropicWebSearchToolResultContent {
    /// Creates a new web search tool result content block.
    ///
    /// # Arguments
    ///
    /// * `tool_use_id` - ID of the tool use this is a result for
    /// * `content` - Array of search result items
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::web_search_tool_result::{
    ///     AnthropicWebSearchToolResultContent, WebSearchResultItem
    /// };
    ///
    /// let results = vec![
    ///     WebSearchResultItem::new("url", "title", "content", "type")
    /// ];
    ///
    /// let web_search = AnthropicWebSearchToolResultContent::new(
    ///     "server_toolu_abc123",
    ///     results
    /// );
    /// ```
    pub fn new(tool_use_id: impl Into<String>, content: Vec<WebSearchResultItem>) -> Self {
        Self {
            content_type: "web_search_tool_result".to_string(),
            tool_use_id: tool_use_id.into(),
            content,
            cache_control: None,
        }
    }

    /// Sets the cache control for this web search tool result.
    ///
    /// # Arguments
    ///
    /// * `cache_control` - The cache control configuration
    pub fn with_cache_control(mut self, cache_control: AnthropicCacheControl) -> Self {
        self.cache_control = Some(cache_control);
        self
    }

    /// Removes cache control from this web search tool result.
    pub fn without_cache_control(mut self) -> Self {
        self.cache_control = None;
        self
    }

    /// Updates the tool use ID.
    pub fn with_tool_use_id(mut self, tool_use_id: impl Into<String>) -> Self {
        self.tool_use_id = tool_use_id.into();
        self
    }

    /// Updates the content (search results).
    pub fn with_content(mut self, content: Vec<WebSearchResultItem>) -> Self {
        self.content = content;
        self
    }

    /// Adds a single search result item to the content.
    pub fn add_result(mut self, result: WebSearchResultItem) -> Self {
        self.content.push(result);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    use serde_json::json;

    #[test]
    fn test_web_search_result_item_new() {
        let result =
            WebSearchResultItem::new("https://example.com", "Example", "encrypted", "web_page");

        assert_eq!(result.url, "https://example.com");
        assert_eq!(result.title, "Example");
        assert!(result.page_age.is_none());
        assert_eq!(result.encrypted_content, "encrypted");
        assert_eq!(result.result_type, "web_page");
    }

    #[test]
    fn test_web_search_result_item_with_page_age() {
        let result =
            WebSearchResultItem::new("url", "title", "content", "type").with_page_age("2 days ago");

        assert_eq!(result.page_age, Some("2 days ago".to_string()));
    }

    #[test]
    fn test_web_search_result_item_without_page_age() {
        let result = WebSearchResultItem::new("url", "title", "content", "type")
            .with_page_age("1 day ago")
            .without_page_age();

        assert!(result.page_age.is_none());
    }

    #[test]
    fn test_web_search_result_item_builder() {
        let result = WebSearchResultItem::new("url1", "title1", "content1", "type1")
            .with_url("url2")
            .with_title("title2")
            .with_encrypted_content("content2")
            .with_result_type("type2")
            .with_page_age("3 days ago");

        assert_eq!(result.url, "url2");
        assert_eq!(result.title, "title2");
        assert_eq!(result.encrypted_content, "content2");
        assert_eq!(result.result_type, "type2");
        assert_eq!(result.page_age, Some("3 days ago".to_string()));
    }

    #[test]
    fn test_web_search_result_item_serialize_without_page_age() {
        let result = WebSearchResultItem::new("https://example.com", "Title", "encrypted", "page");
        let json = serde_json::to_value(&result).unwrap();

        assert_eq!(
            json,
            json!({
                "url": "https://example.com",
                "title": "Title",
                "encrypted_content": "encrypted",
                "type": "page"
            })
        );
    }

    #[test]
    fn test_web_search_result_item_serialize_with_page_age() {
        let result = WebSearchResultItem::new("https://example.com", "Title", "encrypted", "page")
            .with_page_age("1 week ago");
        let json = serde_json::to_value(&result).unwrap();

        assert_eq!(
            json,
            json!({
                "url": "https://example.com",
                "title": "Title",
                "page_age": "1 week ago",
                "encrypted_content": "encrypted",
                "type": "page"
            })
        );
    }

    #[test]
    fn test_web_search_result_item_deserialize() {
        let json = json!({
            "url": "https://example.com",
            "title": "Example",
            "page_age": "2 days ago",
            "encrypted_content": "content",
            "type": "article"
        });

        let result: WebSearchResultItem = serde_json::from_value(json).unwrap();

        assert_eq!(result.url, "https://example.com");
        assert_eq!(result.title, "Example");
        assert_eq!(result.page_age, Some("2 days ago".to_string()));
        assert_eq!(result.encrypted_content, "content");
        assert_eq!(result.result_type, "article");
    }

    #[test]
    fn test_web_search_result_item_deserialize_null_page_age() {
        let json = json!({
            "url": "https://example.com",
            "title": "Example",
            "page_age": null,
            "encrypted_content": "content",
            "type": "page"
        });

        let result: WebSearchResultItem = serde_json::from_value(json).unwrap();

        assert!(result.page_age.is_none());
    }

    #[test]
    fn test_web_search_tool_result_new() {
        let results = vec![
            WebSearchResultItem::new("url1", "title1", "content1", "type1"),
            WebSearchResultItem::new("url2", "title2", "content2", "type2"),
        ];

        let web_search = AnthropicWebSearchToolResultContent::new("toolu_123", results);

        assert_eq!(web_search.content_type, "web_search_tool_result");
        assert_eq!(web_search.tool_use_id, "toolu_123");
        assert_eq!(web_search.content.len(), 2);
        assert!(web_search.cache_control.is_none());
    }

    #[test]
    fn test_web_search_tool_result_with_cache_control() {
        let cache = AnthropicCacheControl::new(CacheTTL::FiveMinutes);
        let web_search = AnthropicWebSearchToolResultContent::new("toolu_123", vec![])
            .with_cache_control(cache.clone());

        assert_eq!(web_search.cache_control, Some(cache));
    }

    #[test]
    fn test_web_search_tool_result_without_cache_control() {
        let web_search = AnthropicWebSearchToolResultContent::new("toolu_123", vec![])
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour))
            .without_cache_control();

        assert!(web_search.cache_control.is_none());
    }

    #[test]
    fn test_web_search_tool_result_with_tool_use_id() {
        let web_search =
            AnthropicWebSearchToolResultContent::new("old_id", vec![]).with_tool_use_id("new_id");

        assert_eq!(web_search.tool_use_id, "new_id");
    }

    #[test]
    fn test_web_search_tool_result_with_content() {
        let results1 = vec![WebSearchResultItem::new(
            "url1", "title1", "content1", "type1",
        )];
        let results2 = vec![WebSearchResultItem::new(
            "url2", "title2", "content2", "type2",
        )];

        let web_search =
            AnthropicWebSearchToolResultContent::new("toolu_123", results1).with_content(results2);

        assert_eq!(web_search.content.len(), 1);
        assert_eq!(web_search.content[0].url, "url2");
    }

    #[test]
    fn test_web_search_tool_result_add_result() {
        let result1 = WebSearchResultItem::new("url1", "title1", "content1", "type1");
        let result2 = WebSearchResultItem::new("url2", "title2", "content2", "type2");

        let web_search = AnthropicWebSearchToolResultContent::new("toolu_123", vec![result1])
            .add_result(result2);

        assert_eq!(web_search.content.len(), 2);
        assert_eq!(web_search.content[0].url, "url1");
        assert_eq!(web_search.content[1].url, "url2");
    }

    #[test]
    fn test_web_search_tool_result_serialize() {
        let results = vec![
            WebSearchResultItem::new(
                "https://example.com/1",
                "First Result",
                "encrypted1",
                "web_page",
            )
            .with_page_age("1 day ago"),
            WebSearchResultItem::new(
                "https://example.com/2",
                "Second Result",
                "encrypted2",
                "article",
            ),
        ];

        let web_search = AnthropicWebSearchToolResultContent::new("toolu_123", results);
        let json = serde_json::to_value(&web_search).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "web_search_tool_result",
                "tool_use_id": "toolu_123",
                "content": [
                    {
                        "url": "https://example.com/1",
                        "title": "First Result",
                        "page_age": "1 day ago",
                        "encrypted_content": "encrypted1",
                        "type": "web_page"
                    },
                    {
                        "url": "https://example.com/2",
                        "title": "Second Result",
                        "encrypted_content": "encrypted2",
                        "type": "article"
                    }
                ]
            })
        );
    }

    #[test]
    fn test_web_search_tool_result_serialize_with_cache() {
        let results = vec![WebSearchResultItem::new("url", "title", "content", "type")];
        let web_search = AnthropicWebSearchToolResultContent::new("toolu_123", results)
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let json = serde_json::to_value(&web_search).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "web_search_tool_result",
                "tool_use_id": "toolu_123",
                "content": [
                    {
                        "url": "url",
                        "title": "title",
                        "encrypted_content": "content",
                        "type": "type"
                    }
                ],
                "cache_control": {
                    "type": "ephemeral",
                    "ttl": "1h"
                }
            })
        );
    }

    #[test]
    fn test_web_search_tool_result_deserialize() {
        let json = json!({
            "type": "web_search_tool_result",
            "tool_use_id": "toolu_456",
            "content": [
                {
                    "url": "https://example.com",
                    "title": "Example",
                    "page_age": "3 days ago",
                    "encrypted_content": "encrypted",
                    "type": "page"
                }
            ]
        });

        let web_search: AnthropicWebSearchToolResultContent = serde_json::from_value(json).unwrap();

        assert_eq!(web_search.content_type, "web_search_tool_result");
        assert_eq!(web_search.tool_use_id, "toolu_456");
        assert_eq!(web_search.content.len(), 1);
        assert_eq!(web_search.content[0].url, "https://example.com");
        assert_eq!(
            web_search.content[0].page_age,
            Some("3 days ago".to_string())
        );
        assert!(web_search.cache_control.is_none());
    }

    #[test]
    fn test_web_search_tool_result_deserialize_with_cache() {
        let json = json!({
            "type": "web_search_tool_result",
            "tool_use_id": "toolu_789",
            "content": [],
            "cache_control": {
                "type": "ephemeral",
                "ttl": "5m"
            }
        });

        let web_search: AnthropicWebSearchToolResultContent = serde_json::from_value(json).unwrap();

        assert_eq!(
            web_search.cache_control,
            Some(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
        );
    }

    #[test]
    fn test_equality() {
        let results1 = vec![WebSearchResultItem::new("url", "title", "content", "type")];
        let results2 = vec![WebSearchResultItem::new("url", "title", "content", "type")];
        let results3 = vec![WebSearchResultItem::new("url2", "title", "content", "type")];

        let ws1 = AnthropicWebSearchToolResultContent::new("id", results1);
        let ws2 = AnthropicWebSearchToolResultContent::new("id", results2);
        let ws3 = AnthropicWebSearchToolResultContent::new("id", results3);

        assert_eq!(ws1, ws2);
        assert_ne!(ws1, ws3);
    }

    #[test]
    fn test_clone() {
        let results = vec![WebSearchResultItem::new("url", "title", "content", "type")];
        let ws1 = AnthropicWebSearchToolResultContent::new("id", results)
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let ws2 = ws1.clone();

        assert_eq!(ws1, ws2);
    }

    #[test]
    fn test_builder_chaining() {
        let result1 = WebSearchResultItem::new("url1", "title1", "content1", "type1");
        let result2 = WebSearchResultItem::new("url2", "title2", "content2", "type2");
        let result3 = WebSearchResultItem::new("url3", "title3", "content3", "type3");

        let web_search = AnthropicWebSearchToolResultContent::new("id1", vec![result1])
            .with_tool_use_id("id2")
            .add_result(result2)
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
            .add_result(result3)
            .without_cache_control();

        assert_eq!(web_search.tool_use_id, "id2");
        assert_eq!(web_search.content.len(), 3);
        assert!(web_search.cache_control.is_none());
    }

    #[test]
    fn test_roundtrip_serialization() {
        let results = vec![
            WebSearchResultItem::new("url1", "title1", "content1", "type1")
                .with_page_age("1 hour ago"),
            WebSearchResultItem::new("url2", "title2", "content2", "type2"),
        ];
        let original = AnthropicWebSearchToolResultContent::new("toolu_roundtrip", results)
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicWebSearchToolResultContent =
            serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_empty_results() {
        let web_search = AnthropicWebSearchToolResultContent::new("toolu_123", vec![]);

        assert_eq!(web_search.content.len(), 0);
    }

    #[test]
    fn test_multiple_results_with_various_page_ages() {
        let results = vec![
            WebSearchResultItem::new("url1", "title1", "content1", "type1")
                .with_page_age("1 minute ago"),
            WebSearchResultItem::new("url2", "title2", "content2", "type2")
                .with_page_age("1 hour ago"),
            WebSearchResultItem::new("url3", "title3", "content3", "type3")
                .with_page_age("1 day ago"),
            WebSearchResultItem::new("url4", "title4", "content4", "type4")
                .with_page_age("1 week ago"),
            WebSearchResultItem::new("url5", "title5", "content5", "type5"), // No page age
        ];

        let web_search = AnthropicWebSearchToolResultContent::new("toolu_123", results);

        assert_eq!(web_search.content.len(), 5);
        assert_eq!(
            web_search.content[0].page_age,
            Some("1 minute ago".to_string())
        );
        assert_eq!(
            web_search.content[1].page_age,
            Some("1 hour ago".to_string())
        );
        assert_eq!(
            web_search.content[2].page_age,
            Some("1 day ago".to_string())
        );
        assert_eq!(
            web_search.content[3].page_age,
            Some("1 week ago".to_string())
        );
        assert!(web_search.content[4].page_age.is_none());
    }
}
