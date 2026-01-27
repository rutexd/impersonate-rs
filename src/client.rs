use crate::browser::Browser;
use crate::error::{Error, Result};
use crate::ffi;
use curl::easy::{Easy, List};
use std::ffi::CString;
use std::time::Duration;

/// A synchronous HTTP client wrapper around `curl-impersonate`.
///
/// Use `Client::builder()` to configure it.
pub struct Client {
    impersonate: Option<Browser>,
    default_headers: bool,
    verify: bool,
    timeout: Option<Duration>,
}

impl Client {
    /// Creates a new default Client.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a ClientBuilder.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// Convenience method for GET requests.
    pub fn get(&self, url: &str) -> RequestBuilder {
        self.request("GET", url)
    }

    /// Convenience method for POST requests.
    pub fn post(&self, url: &str) -> RequestBuilder {
        self.request("POST", url)
    }

    /// Creates a RequestBuilder for the given method and URL.
    pub fn request(&self, method: &str, url: &str) -> RequestBuilder {
        let mut builder = RequestBuilder::new(url)
            .method(method)
            .default_headers(self.default_headers)
            .verify(self.verify);

        if let Some(browser) = self.impersonate {
            builder = builder.impersonate(browser);
        }

        if let Some(timeout) = self.timeout {
            builder = builder.timeout(timeout);
        }

        builder
    }
}

impl Default for Client {
    fn default() -> Self {
        Self {
            impersonate: None,
            default_headers: true,
            verify: true,
            timeout: Some(Duration::from_secs(30)),
        }
    }
}

/// Builder for constructing a `Client`.
pub struct ClientBuilder {
    client: Client,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            client: Client::default(),
        }
    }
}

impl ClientBuilder {
    /// Sets the browser profile to impersonate.
    pub fn impersonate(mut self, browser: Browser) -> Self {
        self.client.impersonate = Some(browser);
        self
    }

    /// Enables or disables default headers (default: true).
    pub fn default_headers(mut self, enable: bool) -> Self {
        self.client.default_headers = enable;
        self
    }

    /// Enables or disables SSL verification (default: true).
    pub fn verify(mut self, verify: bool) -> Self {
        self.client.verify = verify;
        self
    }

    /// Sets the request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.client.timeout = Some(timeout);
        self
    }

    /// Builds the Client.
    pub fn build(self) -> Client {
        self.client
    }
}

/// Builder for constructing an HTTP request.
pub struct RequestBuilder {
    url: String,
    method: String,
    headers: List,
    body: Option<Vec<u8>>,
    impersonate: Option<Browser>,
    default_headers: bool,
    verify: bool,
    timeout: Option<Duration>,
}

impl RequestBuilder {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            method: "GET".to_string(),
            headers: List::new(),
            body: None,
            impersonate: None,
            default_headers: true,
            verify: true,
            timeout: None,
        }
    }

    pub fn method(mut self, method: &str) -> Self {
        self.method = method.to_uppercase();
        self
    }

    /// Adds a header to the request.
    pub fn header(mut self, key: &str, value: &str) -> Result<Self> {
        self.headers
            .append(&format!("{}: {}", key, value))
            .map_err(Error::Curl)?;
        Ok(self)
    }

    /// Sets the request body.
    pub fn body<T: Into<Vec<u8>>>(mut self, body: T) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Overrides the browser profile for this request.
    pub fn impersonate(mut self, browser: Browser) -> Self {
        self.impersonate = Some(browser);
        self
    }

    pub fn default_headers(mut self, enable: bool) -> Self {
        self.default_headers = enable;
        self
    }

    pub fn verify(mut self, verify: bool) -> Self {
        self.verify = verify;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Sends the request and returns a Response.
    pub fn send(self) -> Result<Response> {
        let mut easy = Easy::new();

        // Basic options
        easy.url(&self.url)?;
        easy.ssl_verify_peer(self.verify)?;
        easy.ssl_verify_host(self.verify)?;

        if let Some(timeout) = self.timeout {
            easy.timeout(timeout)?;
        }

        // Method and Body
        match self.method.as_str() {
            "GET" => easy.get(true)?,
            "POST" => {
                easy.post(true)?;
                if let Some(body) = &self.body {
                    easy.post_field_size(body.len() as u64)?;
                    easy.post_fields_copy(body)?;
                }
            }
            "PUT" => {
                easy.put(true)?;
                if let Some(body) = &self.body {
                    easy.post_field_size(body.len() as u64)?;
                    easy.post_fields_copy(body)?;
                }
            }
            "HEAD" => easy.nobody(true)?,
            m => easy.custom_request(m)?,
        }

        // Headers
        easy.http_headers(self.headers)?;

        // Impersonation
        // IMPORTANT: Must be called before connection
        if let Some(browser) = self.impersonate {
            let browser_str = CString::new(browser.as_str())
                .map_err(|e| Error::Impersonate(format!("Invalid browser string: {}", e)))?;

            let raw = easy.raw();
            unsafe {
                let code = ffi::curl_easy_impersonate(
                    raw,
                    browser_str.as_ptr(),
                    if self.default_headers { 1 } else { 0 },
                );

                if code != 0 {
                    return Err(Error::Impersonate(format!(
                        "Failed to impersonate {}: code {}",
                        browser.as_str(),
                        code
                    )));
                }
            }
        }

        // Perform request and capture response
        let mut response_body = Vec::new();
        let mut response_headers = Vec::new();

        {
            let mut transfer = easy.transfer();
            transfer.write_function(|data| {
                response_body.extend_from_slice(data);
                Ok(data.len())
            })?;

            transfer.header_function(|header| {
                response_headers.extend_from_slice(header);
                true
            })?;

            transfer.perform()?;
        }

        let status_code = easy.response_code()?;

        Ok(Response {
            status_code,
            body: response_body,
            headers: parse_headers(&response_headers),
            url: self.url,
        })
    }
}

/// HTTP Response object.
#[derive(Debug)]
pub struct Response {
    status_code: u32,
    body: Vec<u8>,
    headers: std::collections::HashMap<String, String>,
    url: String,
}

impl Response {
    /// Returns the HTTP status code.
    pub fn status(&self) -> u32 {
        self.status_code
    }

    /// Returns the response body as a String (UTF-8).
    pub fn text(&self) -> Result<String> {
        String::from_utf8(self.body.clone())
            .map_err(|e| Error::Impersonate(format!("UTF-8 Error: {}", e)))
    }

    /// Deserializes the response body as JSON.
    pub fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T> {
        serde_json::from_slice(&self.body).map_err(Error::Json)
    }

    /// Returns the response body as raw bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.body
    }

    /// Returns the response headers.
    pub fn headers(&self) -> &std::collections::HashMap<String, String> {
        &self.headers
    }

    /// Returns the effective URL.
    pub fn url(&self) -> &str {
        &self.url
    }
}

fn parse_headers(raw: &[u8]) -> std::collections::HashMap<String, String> {
    let mut headers = std::collections::HashMap::new();
    if let Ok(s) = std::str::from_utf8(raw) {
        for line in s.lines() {
            if let Some((key, value)) = line.split_once(':') {
                headers.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
    }
    headers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_builder() {
        let client = Client::builder()
            .impersonate(Browser::Chrome100)
            .default_headers(false)
            .build();

        assert_eq!(client.impersonate, Some(Browser::Chrome100));
        assert!(!client.default_headers);
    }

    #[test]
    fn test_request_builder() {
        let client = Client::new();
        let req = client.get("https://example.com");
        assert_eq!(req.url, "https://example.com");
        assert_eq!(req.method, "GET");
    }

    // NOTE: We cannot easily test `send()` without a real curl implementation
    // unless we mock `curl::easy::Easy`. But since we stubbed the FFI,
    // if `curl-sys` works, it might just run and return 0 status or fail to connect.
    // In this environment, `curl` crate likely links to system curl, but
    // `curl_easy_impersonate` is stubbed.
    // So we can try to run a request.

    #[test]
    fn test_ffi_stub_call() {
        // This verifies that we can call the function without linking error
        // The real behavior depends on the stub in ffi.rs
        let client = Client::builder().impersonate(Browser::Chrome100).build();

        // This might fail due to network, but shouldn't fail due to symbol lookup
        let _ = client.get("http://localhost:12345").send();
    }
}
