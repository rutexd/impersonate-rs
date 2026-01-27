pub mod browser;
pub mod client;
pub mod error;
pub mod ffi;
pub mod fingerprint;

pub use browser::Browser;
pub use client::{Client, ClientBuilder, Response, Session};
pub use error::{Error, Result};
pub use fingerprint::{set_akamai_options, set_ja3_options};

// Re-export docs modules for docs.rs visibility if we wanted to embed them,
// but for now we rely on the crate-level doc comment below.

//! impersonate-rs
//!
//! A Rust wrapper for [curl-impersonate](https://github.com/lexiforest/curl-impersonate),
//! providing browser fingerprinting capabilities.
//!
//! ## Documentation
//!
//! - [Quick Start](https://github.com/ajsb85/impersonate-rs/blob/main/docs/quickstart.md)
//! - [Advanced Topics](https://github.com/ajsb85/impersonate-rs/blob/main/docs/advanced/overview.md)
//! - [API Reference](https://github.com/ajsb85/impersonate-rs/blob/main/docs/api.md)
//! - [Comparison vs Reqwest](https://github.com/ajsb85/impersonate-rs/blob/main/docs/vs_reqwest.md)
//!
//! ## Quick Example
//!
//! ```rust,no_run
//! use impersonate_rs::{Client, Browser, Result};
//!
//! # fn main() -> Result<()> {
//! let client = Client::builder()
//!     .impersonate(Browser::Chrome124)
//!     .build();
//!
//! let response = client.get("https://tls.browserleaks.com/json").send()?;
//! println!("{}", response.text()?);
//! # Ok(())
//! # }
//! ```
