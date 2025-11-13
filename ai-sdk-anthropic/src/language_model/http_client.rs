use reqwest;
use serde_json::Value;
use std::collections::HashMap;

use crate::error::parse_anthropic_error;

/// Make an HTTP POST request to the Anthropic API
///
/// # Arguments
///
/// * `url` - The API endpoint URL
/// * `headers` - HTTP headers to include in the request
/// * `body` - Request body as JSON value
///
/// # Returns
///
/// The response body as a JSON value, or an error
pub async fn post_json(
    url: &str,
    headers: HashMap<String, String>,
    body: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    // Serialize body to string for logging and sending
    let body_string = serde_json::to_string(&body)?;

    // Create HTTP client
    let client = reqwest::Client::new();

    // Build request
    let mut request = client.post(url).header("Content-Type", "application/json");

    // Add custom headers
    for (key, value) in headers {
        request = request.header(key, value);
    }

    // Send request
    let response = request.body(body_string).send().await?;

    // Get status and response body
    let status = response.status();
    let response_body = response.text().await?;

    // Handle error responses
    if !status.is_success() {
        let provider_error = parse_anthropic_error(status.as_u16(), &response_body);
        return Err(Box::new(provider_error));
    }

    // Parse successful response
    let response_json: Value = serde_json::from_str(&response_body)?;

    Ok(response_json)
}

/// Make an HTTP POST request with streaming support
///
/// # Arguments
///
/// * `url` - The API endpoint URL
/// * `headers` - HTTP headers to include in the request
/// * `body` - Request body as JSON value
///
/// # Returns
///
/// A byte stream from the response
pub async fn post_stream(
    url: &str,
    headers: HashMap<String, String>,
    body: Value,
) -> Result<
    impl futures_util::Stream<Item = Result<bytes::Bytes, reqwest::Error>> + 'static,
    Box<dyn std::error::Error>,
> {
    // Serialize body
    let body_string = serde_json::to_string(&body)?;

    // Create HTTP client
    let client = reqwest::Client::new();

    // Build request
    let mut request = client.post(url).header("Content-Type", "application/json");

    // Add custom headers
    for (key, value) in headers {
        request = request.header(key, value);
    }

    // Send request
    let response = request.body(body_string).send().await?;

    // Get status
    let status = response.status();

    // Handle error responses
    if !status.is_success() {
        let response_body = response.text().await?;
        let provider_error = parse_anthropic_error(status.as_u16(), &response_body);
        return Err(Box::new(provider_error));
    }

    // Return byte stream
    Ok(response.bytes_stream())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_exists() {
        // Basic test to ensure module compiles
        // This is a placeholder test
    }
}
