//! impersonate-rs
//!
//! A Rust wrapper for [curl-impersonate](https://github.com/lexiforest/curl-impersonate),
//! providing browser fingerprinting capabilities.
//!
//! ## Quick Start
//!
//! ### Installation
//!
//! Add `impersonate-rs` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! impersonate-rs = "0.1.1"
//! ```
//!
//! ### Basic Requests
//!
//! Perform a simple GET request with browser impersonation:
//!
//! ```rust,no_run
//! use impersonate_rs::{Client, Browser, Result};
//!
//! # fn main() -> impersonate_rs::Result<()> {
//! let client = Client::builder()
//!     .impersonate(Browser::Chrome100)
//!     .build();
//!
//! let response = client.get("https://tls.browserleaks.com/json").send()?;
//! println!("{}", response.text()?);
//! # Ok(())
//! # }
//! ```
//!
//! ## Modules
//!
//! - [`browser`]: Supported browser profiles.
//! - [`client`]: HTTP client and request builder.
//! - [`error`]: Error types.
//! - [`fingerprint`]: Low-level JA3/Akamai configuration.
//!
//! ## Guides
//!
//! <details>
//! <summary><strong>Cookies & Sessions</strong></summary>
//!
//! ### Using `Session` for Cookie Management
//!
//! The most robust way to handle cookies is using the [`Session`] struct. `libcurl` handles the cookie jar internally in memory.
//!
//! ```rust,no_run
//! use impersonate_rs::{Client, Browser, Session, Result};
//!
//! # fn main() -> Result<()> {
//! let client = Client::builder()
//!     .impersonate(Browser::Chrome124)
//!     .build();
//!
//! // Create a session which holds a cookie jar
//! let session = Session::new(client);
//!
//! // 1. Visit a page that sets a cookie
//! session.get("https://httpbin.org/cookies/set/session_id/12345")?;
//!
//! // 2. Visit another page, the cookie is sent automatically
//! let resp = session.get("https://httpbin.org/cookies")?;
//! println!("Cookies sent: {}", resp.text()?);
//! # Ok(())
//! # }
//! ```
//!
//! </details>
//!
//! <details>
//! <summary><strong>Async/Tokio Usage</strong></summary>
//!
//! `impersonate-rs` primarily exposes a **synchronous (blocking)** API because `libcurl` is fundamentally blocking.
//! However, you can easily use it in an `async` context (like Tokio) by wrapping requests in `task::spawn_blocking`.
//!
//! ```rust,no_run
//! use impersonate_rs::{Client, Browser};
//! use tokio::task;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::builder()
//!         .impersonate(Browser::Chrome124)
//!         .build();
//!
//!     // Spawn blocking task
//!     let response_text = task::spawn_blocking(move || {
//!         let resp = client.get("https://tls.browserleaks.com/json").send()?;
//!         resp.text()
//!     }).await??;
//!
//!     println!("Response: {}", response_text);
//!     Ok(())
//! }
//! ```
//! </details>
//!
//! <details>
//! <summary><strong>Proxies & Auth</strong></summary>
//!
//! ### Proxies
//!
//! ```rust,no_run
//! # use impersonate_rs::{Client, Browser};
//! # fn main() {
//! let client = Client::builder()
//!     .impersonate(Browser::Chrome124)
//!     .proxy("http://user:pass@127.0.0.1:8080")
//!     .build();
//! # }
//! ```
//!
//! ### Basic Auth
//!
//! ```rust,no_run
//! # use impersonate_rs::Client;
//! # fn main() {
//! let client = Client::new();
//! let req = client.get("https://httpbin.org/basic-auth/user/pass")
//!     .basic_auth("user", "pass");
//! # }
//! ```
//! </details>
//!
//! <details>
//! <summary><strong>Advanced Fingerprinting</strong></summary>
//!
//! For precise control over TLS (JA3) and HTTP/2 (Akamai) fingerprints:
//!
//! ```rust,no_run
//! # use impersonate_rs::Client;
//! # fn main() {
//! let client = Client::builder()
//!     .ja3("771,4865-4866-4867-49195-49199-49196-49200-52393-52392-49171-49172-156-157-47-53,0-23-65281-10-11-35-16-5-13-18-51-45-43-27-17513,29-23-24,0")
//!     .akamai("1:65536,2:0,3:1000,4:6291456,6:262144|15663105|0|m,a,s,p")
//!     .build();
//! # }
//! ```
//! </details>
//!
//! ## External Documentation
//!
//! - [Building libcurl (Advanced)](https://github.com/ajsb85/impersonate-rs/blob/main/docs/building_libcurl.md)
//! - [Full Comparison vs Reqwest](https://github.com/ajsb85/impersonate-rs/blob/main/docs/vs_reqwest.md)
//! - [WebSockets Status](https://github.com/ajsb85/impersonate-rs/blob/main/docs/websockets.md)

pub mod browser;
pub mod client;
pub mod error;
pub mod ffi;
pub mod fingerprint;

pub use browser::Browser;
pub use client::{Client, ClientBuilder, Response, Session};
pub use error::{Error, Result};
pub use fingerprint::{set_akamai_options, set_ja3_options};
