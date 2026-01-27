//! impersonate-rs
//!
//! A Rust wrapper for [curl-impersonate](https://github.com/lexiforest/curl-impersonate),
//! providing browser fingerprinting capabilities.
//!
//! This crate allows you to perform HTTP requests that mimic specific browser fingerprints
//! (TLS handshake, HTTP/2 settings, headers), helping to bypass bot protections that rely on TLS fingerprinting.
//!
//! ## Prerequisites
//!
//! This crate requires `libcurl-impersonate` to be installed and available to the linker.
//!
//! ## Example
//!
//! ```rust,no_run
//! use impersonate_rs::{Client, Browser};
//!
//! # fn main() -> impersonate_rs::Result<()> {
//! let client = Client::builder()
//!     .impersonate(Browser::Chrome100)
//!     .build();
//!
//! let resp = client.get("https://tls.browserleaks.com/json").send()?;
//! println!("{}", resp.text()?);
//! # Ok(())
//! # }
//! ```

pub mod browser;
pub mod client;
pub mod error;
pub mod ffi;
pub mod fingerprint;

pub use browser::Browser;
pub use client::{Client, ClientBuilder, Response};
pub use error::{Error, Result};
pub use fingerprint::{set_akamai_options, set_ja3_options};
