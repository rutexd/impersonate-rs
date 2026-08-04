use crate::browser::Browser;
use crate::error::{Error, Result};
use crate::ffi;
use curl::easy::{Easy, List};
use std::ffi::CString;
use std::time::Duration;
use std::io::Read;

#[cfg(not(feature = "mock"))]
mod ca {
    pub static CACERT_PEM: &str =
        include_str!(concat!(env!("OUT_DIR"), "/cacert.pem"));
}

/// Request body variants for different upload strategies.
enum Body {
    /// Body stored in memory (for small payloads)
    Buffer(Vec<u8>),
    /// Body from a reader (for large files, streams data)
    Stream(Box<dyn Read + Send>),
}

/// A synchronous HTTP client wrapper around `curl-impersonate`.
///
/// Use `Client::builder()` to configure it.
#[derive(Debug, Clone)]
pub struct Client {
    impersonate: Option<Browser>,
    ja3: Option<String>,
    akamai: Option<String>,
    permute_extensions: bool,
    default_headers: bool,
    follow_redirects: bool,
    verify: bool,
    timeout: Option<Duration>,
    proxy: Option<String>,
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
            .follow_redirects(self.follow_redirects)
            .verify(self.verify);

        if let Some(browser) = self.impersonate {
            builder = builder.impersonate(browser);
        }

        if let Some(ja3) = &self.ja3 {
            builder = builder.ja3(ja3);
        }

        if let Some(akamai) = &self.akamai {
            builder = builder.akamai(akamai);
        }

        builder = builder.permute_extensions(self.permute_extensions);

        if let Some(timeout) = self.timeout {
            builder = builder.timeout(timeout);
        }

        if let Some(proxy) = &self.proxy {
            builder = builder.proxy(proxy);
        }

        builder
    }
}

impl Default for Client {
    fn default() -> Self {
        Self {
            impersonate: None,
            ja3: None,
            akamai: None,
            permute_extensions: true,
            default_headers: true,
            follow_redirects: true,
            verify: true,
            timeout: Some(Duration::from_secs(30)),
            proxy: None,
        }
    }
}

/// Builder for constructing a `Client`.
#[derive(Default, Debug, Clone)]
pub struct ClientBuilder {
    client: Client,
}

impl ClientBuilder {
    /// Sets the browser profile to impersonate.
    pub fn impersonate(mut self, browser: Browser) -> Self {
        self.client.impersonate = Some(browser);
        self
    }

    /// Sets the JA3 fingerprint string.
    pub fn ja3(mut self, ja3: &str) -> Self {
        self.client.ja3 = Some(ja3.to_string());
        self
    }

    /// Sets the Akamai fingerprint string.
    pub fn akamai(mut self, akamai: &str) -> Self {
        self.client.akamai = Some(akamai.to_string());
        self
    }

    /// Enables or disables TLS extension permutation (default: true).
    pub fn permute_extensions(mut self, enable: bool) -> Self {
        self.client.permute_extensions = enable;
        self
    }

    /// Enables or disables default headers (default: true).
    pub fn default_headers(mut self, enable: bool) -> Self {
        self.client.default_headers = enable;
        self
    }

    /// Sets whether to follow redirects (default: true).
    pub fn follow_redirects(mut self, enable: bool) -> Self {
        self.client.follow_redirects = enable;
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

    /// Sets a proxy URL (e.g. "http://127.0.0.1:8080").
    pub fn proxy(mut self, proxy: &str) -> Self {
        self.client.proxy = Some(proxy.to_string());
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
    body: Option<Body>,
    impersonate: Option<Browser>,
    ja3: Option<String>,
    akamai: Option<String>,
    permute_extensions: bool,
    default_headers: bool,
    follow_redirects: bool,
    verify: bool,
    timeout: Option<Duration>,
    proxy: Option<String>,
    auth: Option<(String, String)>,
}

impl RequestBuilder {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            method: "GET".to_string(),
            headers: List::new(),
            body: None,
            impersonate: None,
            ja3: None,
            akamai: None,
            permute_extensions: true,
            default_headers: true,
            follow_redirects: true,
            verify: true,
            timeout: None,
            proxy: None,
            auth: None,
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
        self.body = Some(Body::Buffer(body.into()));
        self
    }

    /// Sets the request body from a reader (for streaming large files).
    /// 
    /// This method allows uploading large files without loading them entirely into memory.
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use impersonate_rs::{Client, Result};
    /// use std::fs::File;
    /// 
    /// # fn main() -> Result<()> {
    /// let client = Client::new();
    /// let file = File::open("large_video.mp4").unwrap();
    /// 
    /// let response = client.post("https://example.com/upload")
    ///     .body_reader(file)
    ///     .send()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn body_reader<R: Read + Send + 'static>(mut self, reader: R) -> Self {
        self.body = Some(Body::Stream(Box::new(reader)));
        self
    }

    /// Sets the request body as JSON.
    pub fn json<T: serde::Serialize>(mut self, data: &T) -> Result<Self> {
        let body = serde_json::to_vec(data)?;
        self.headers
            .append("Content-Type: application/json")
            .map_err(Error::Curl)?;
        self.body = Some(Body::Buffer(body));
        Ok(self)
    }

    /// Sets the request body as URL-encoded form data.
    pub fn form<T: serde::Serialize>(mut self, data: &T) -> Result<Self> {
        let body = serde_urlencoded::to_string(data)
            .map_err(|e| Error::Io(std::io::Error::new(std::io::ErrorKind::InvalidInput, e)))?
            .into_bytes();
        self.headers
            .append("Content-Type: application/x-www-form-urlencoded")
            .map_err(Error::Curl)?;
        self.body = Some(Body::Buffer(body));
        Ok(self)
    }

    /// Overrides the browser profile for this request.
    pub fn impersonate(mut self, browser: Browser) -> Self {
        self.impersonate = Some(browser);
        self
    }

    pub fn ja3(mut self, ja3: &str) -> Self {
        self.ja3 = Some(ja3.to_string());
        self
    }

    pub fn akamai(mut self, akamai: &str) -> Self {
        self.akamai = Some(akamai.to_string());
        self
    }

    pub fn permute_extensions(mut self, enable: bool) -> Self {
        self.permute_extensions = enable;
        self
    }

    pub fn default_headers(mut self, enable: bool) -> Self {
        self.default_headers = enable;
        self
    }

    pub fn follow_redirects(mut self, enable: bool) -> Self {
        self.follow_redirects = enable;
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

    pub fn proxy(mut self, proxy: &str) -> Self {
        self.proxy = Some(proxy.to_string());
        self
    }

    pub fn basic_auth(mut self, user: &str, pass: &str) -> Self {
        self.auth = Some((user.to_string(), pass.to_string()));
        self
    }

    /// Sends the request and returns a Response.
    /// 
    /// # Examples
    /// 
    /// Basic usage (loads response into memory):
    /// ```no_run
    /// use impersonate_rs::{Client, Result};
    /// 
    /// fn main() -> Result<()> {
    ///     let client = Client::new();
    ///     let response = client.get("https://example.com").send()?;
    ///     println!("{}", response.text()?);
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// Streaming large files:
    /// ```no_run
    /// use impersonate_rs::{Client, Result};
    /// use std::fs::File;
    /// use std::io::Write;
    /// 
    /// fn main() -> Result<()> {
    ///     let client = Client::new();
    ///     let mut file = File::create("download.bin").unwrap();
    ///     
    ///     let response = client.get("https://example.com/largefile.zip")
    ///         .send_with_callback(|chunk| {
    ///             file.write_all(chunk).unwrap();
    ///             Ok(())
    ///         })?;
    ///     
    ///     println!("Downloaded {} bytes", response.bytes_received());
    ///     Ok(())
    /// }
    /// ```
    pub fn send(self) -> Result<Response> {
        let mut easy = Easy::new();
        self.send_with_easy(&mut easy)
    }

    /// Sends the request and streams the response body to a callback.
    /// 
    /// Returns `StreamResponse` which does not contain the body (since it was streamed).
    /// Use this for downloading large files without loading them entirely into memory.
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use impersonate_rs::{Client, Result};
    /// use std::fs::File;
    /// use std::io::Write;
    /// 
    /// fn main() -> Result<()> {
    ///     let client = Client::new();
    ///     let mut file = File::create("download.bin").unwrap();
    ///     
    ///     let response = client.get("https://example.com/largefile.zip")
    ///         .send_with_callback(|chunk| {
    ///             file.write_all(chunk).unwrap();
    ///             Ok(())
    ///         })?;
    ///     
    ///     println!("Status: {}, Downloaded: {} bytes", 
    ///              response.status(), response.bytes_received());
    ///     Ok(())
    /// }
    /// ```
    pub fn send_with_callback<F>(self, mut callback: F) -> Result<StreamResponse>
    where
        F: FnMut(&[u8]) -> Result<()>,
    {
        let mut easy = Easy::new();
        self.send_stream_with_easy(&mut easy, &mut callback)
    }

    /// Async version: Sends the request and streams the response body to a callback.
    /// 
    /// This spawns a blocking task to perform the streaming operation.
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use impersonate_rs::{Client, Result};
    /// use std::io::Write;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client = Client::new();
    ///     
    ///     let response = client.get("https://example.com/largefile.zip")
    ///         .send_with_callback_async(move |chunk| {
    ///             // Note: This callback runs in a blocking context
    ///             // For true async I/O, collect chunks and process them outside
    ///             std::fs::OpenOptions::new()
    ///                 .write(true)
    ///                 .append(true)
    ///                 .open("download.bin")
    ///                 .unwrap()
    ///                 .write_all(chunk)
    ///                 .map_err(|e| impersonate_rs::Error::Io(e))?;
    ///             Ok(())
    ///         })
    ///         .await?;
    ///     
    ///     println!("Downloaded {} bytes", response.bytes_received());
    ///     Ok(())
    /// }
    /// ```
    pub async fn send_with_callback_async<F>(self, callback: F) -> Result<StreamResponse>
    where
        F: FnMut(&[u8]) -> Result<()> + Send + 'static,
    {
        tokio::task::spawn_blocking(move || self.send_with_callback(callback))
            .await
            .map_err(|e| Error::Impersonate(format!("Task join error: {}", e)))?
    }

    // Common setup logic for both send methods
    fn setup_easy(&self, easy: &mut Easy) -> Result<()> {
        // Basic options
        easy.url(&self.url)?;
        easy.ssl_verify_peer(self.verify)?;
        easy.ssl_verify_host(self.verify)?;

        // BoringSSL (used by curl-impersonate) does not integrate with the
        // Windows certificate store. Supply the bundled Mozilla CA certificates
        // so that SSL verification works out of the box on Windows.
        #[cfg(all(target_os = "windows", not(feature = "mock")))]
        if self.verify {
            easy.ssl_cainfo_blob(ca::CACERT_PEM.as_bytes())
                .map_err(|e| Error::Curl(e))?;
        }
        easy.follow_location(self.follow_redirects)?;

        if let Some(timeout) = self.timeout {
            easy.timeout(timeout)?;
        } else {
            easy.timeout(Duration::from_secs(0))?;
        }

        if let Some(proxy) = &self.proxy {
            easy.proxy(proxy)?;
        } else {
            easy.proxy("")?;
        }

        if let Some((user, pass)) = &self.auth {
            easy.username(user)?;
            easy.password(pass)?;
        } else {
            easy.username("")?;
            easy.password("")?;
        }

        // Method
        match self.method.as_str() {
            "GET" => easy.get(true)?,
            "POST" => easy.post(true)?,
            "PUT" => easy.put(true)?,
            "HEAD" => easy.nobody(true)?,
            m => easy.custom_request(m)?,
        }

        // curl_easy_impersonate() sets Accept-Encoding as a raw HTTP header
        // (CURLOPT_HTTPBASEHEADER), but curl only auto-decompresses when
        // CURLOPT_ACCEPT_ENCODING is set. This is what curl_cffi does in
        // set_curl_options() — it calls ACCEPT_ENCODING separately.
        easy.accept_encoding("")?;

        // Impersonation Logic
        // Priority:
        // 1. Explicit JA3/Akamai (Custom)
        // 2. Browser Profile (Preset)

        // Important: Impersonate first if set, as it might reset some options
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

        // Apply JA3 overrides if present
        if let Some(ja3) = &self.ja3 {
            crate::fingerprint::set_ja3_options(easy, ja3, self.permute_extensions)?;
        }

        // Apply Akamai overrides if present
        if let Some(akamai) = &self.akamai {
            crate::fingerprint::set_akamai_options(easy, akamai)?;
        }

        Ok(())
    }

    fn send_with_easy(mut self, easy: &mut Easy) -> Result<Response> {
        self.setup_easy(easy)?;

        // Headers (must be set after setup_easy because it consumes self.headers)
        easy.http_headers(self.headers)?;

        // Perform request and capture response
        let mut response_body = Vec::new();
        let mut response_headers = Vec::new();

        // Handle body upload - prepare before creating transfer
        let body_opt = self.body.take();
        
        // Setup body before transfer if it's a buffer
        if let Some(Body::Buffer(ref buf)) = body_opt {
            easy.post_field_size(buf.len() as u64)?;
            easy.post_fields_copy(buf)?;
        }

        {
            let mut transfer = easy.transfer();
            
            // Setup write callback for response
            transfer.write_function(|data| {
                response_body.extend_from_slice(data);
                Ok(data.len())
            })?;

            transfer.header_function(|header| {
                response_headers.extend_from_slice(header);
                true
            })?;

            // Setup read callback for request body (if streaming)
            if let Some(Body::Stream(mut reader)) = body_opt {
                transfer.read_function(move |buf| {
                    reader.read(buf).map_err(|_| curl::easy::ReadError::Abort)
                })?;
            }

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

    fn send_stream_with_easy<F>(mut self, easy: &mut Easy, callback: &mut F) -> Result<StreamResponse>
    where
        F: FnMut(&[u8]) -> Result<()>,
    {
        self.setup_easy(easy)?;

        // Headers (must be set after setup_easy because it consumes self.headers)
        easy.http_headers(self.headers)?;

        // Perform streaming request
        let mut response_headers = Vec::new();
        let mut total_bytes: u64 = 0;

        // Handle body upload - prepare before creating transfer
        let body_opt = self.body.take();
        
        // Setup body before transfer if it's a buffer
        if let Some(Body::Buffer(ref buf)) = body_opt {
            easy.post_field_size(buf.len() as u64)?;
            easy.post_fields_copy(buf)?;
        }

        {
            let mut transfer = easy.transfer();
            
            // Setup write callback for streaming response
            transfer.write_function(|data| {
                total_bytes += data.len() as u64;
                match callback(data) {
                    Ok(()) => Ok(data.len()),
                    Err(_) => Ok(0), // Signal error by returning 0
                }
            })?;

            transfer.header_function(|header| {
                response_headers.extend_from_slice(header);
                true
            })?;

            // Setup read callback for request body (if streaming)
            if let Some(Body::Stream(mut reader)) = body_opt {
                transfer.read_function(move |buf| {
                    reader.read(buf).map_err(|_| curl::easy::ReadError::Abort)
                })?;
            }

            transfer.perform()?;
        }

        let status_code = easy.response_code()?;

        Ok(StreamResponse {
            status_code,
            headers: parse_headers(&response_headers),
            url: self.url,
            bytes_received: total_bytes,
        })
    }
}

/// A Session that persists cookies across requests.
pub struct Session {
    client: Client,
    easy: std::cell::RefCell<Easy>,
}

impl Session {
    /// Creates a new Session with the given Client configuration.
    pub fn new(client: Client) -> Self {
        let mut easy = Easy::new();
        // Enable cookie engine with in-memory storage
        let _ = easy.cookie_file("");
        Self {
            client,
            easy: std::cell::RefCell::new(easy),
        }
    }

    /// Convenience method for GET requests.
    pub fn get(&self, url: &str) -> Result<Response> {
        self.request("GET", url)
            .send_with_easy(&mut self.easy.borrow_mut())
    }

    /// Convenience method for POST requests.
    pub fn post(&self, url: &str) -> Result<Response> {
        self.request("POST", url)
            .send_with_easy(&mut self.easy.borrow_mut())
    }

    /// Creates a RequestBuilder that will use this session's handle.
    /// Note: The actual execution happens when you call `send_with_easy` manually currently,
    /// or use the convenience methods above.
    /// To allow builder pattern with Session, we need a SessionRequestBuilder.
    /// For simplicity, we just reuse Client's builder but we need to execute it on *our* handle.
    pub fn request(&self, method: &str, url: &str) -> RequestBuilder {
        self.client.request(method, url)
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new(Client::default())
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

impl Clone for Response {
    fn clone(&self) -> Self {
        Self {
            status_code: self.status_code,
            body: self.body.clone(),
            headers: self.headers.clone(),
            url: self.url.clone(),
        }
    }
}

impl Response {
    /// Returns the HTTP status code.
    pub fn status(&self) -> u32 {
        self.status_code
    }

    /// Returns the response body as a String (UTF-8).
    ///
    /// Note: If the response is not valid UTF-8, this will return an error.
    pub fn text(&self) -> Result<String> {
        String::from_utf8(self.body.clone())
            .map_err(|e| Error::Impersonate(format!("UTF-8 Error: {}", e)))
    }

    /// Returns the response body as a String (lossy).
    ///
    /// This will replace invalid UTF-8 sequences with the replacement character ().
    pub fn text_lossy(&self) -> std::borrow::Cow<'_, str> {
        String::from_utf8_lossy(&self.body)
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

/// HTTP Streaming Response object.
/// 
/// Returned by `send_stream()` methods. Does not contain the body,
/// as it was streamed to the callback.
#[derive(Debug)]
pub struct StreamResponse {
    status_code: u32,
    headers: std::collections::HashMap<String, String>,
    url: String,
    bytes_received: u64,
}

impl Clone for StreamResponse {
    fn clone(&self) -> Self {
        Self {
            status_code: self.status_code,
            headers: self.headers.clone(),
            url: self.url.clone(),
            bytes_received: self.bytes_received,
        }
    }
}

impl StreamResponse {
    /// Returns the HTTP status code.
    pub fn status(&self) -> u32 {
        self.status_code
    }

    /// Returns the response headers.
    pub fn headers(&self) -> &std::collections::HashMap<String, String> {
        &self.headers
    }

    /// Returns the effective URL.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Returns the total number of bytes received.
    pub fn bytes_received(&self) -> u64 {
        self.bytes_received
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
