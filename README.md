# impersonate-rs

A Rust wrapper for [curl-impersonate](https://github.com/lexiforest/curl-impersonate), providing browser fingerprinting capabilities.

## Prerequisites

This crate requires `libcurl-impersonate` to be installed and available to the linker. It uses `curl_easy_impersonate` which is specific to the forked version of curl.

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
impersonate-rs = { path = "." }
```

### Example

```rust
use impersonate_rs::{Client, Browser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .impersonate(Browser::Chrome100)
        .build();

    let resp = client.get("https://tls.browserleaks.com/json").send()?;
    
    println!("{}", resp.text()?);
    Ok(())
}
```

## CLI

A CLI tool is included:

```bash
cargo run --bin impersonate -- --url https://tls.browserleaks.com/json --impersonate chrome124
```

## Building

Ensure `libcurl-impersonate` is in your library path (`LD_LIBRARY_PATH` or `pkg-config`).

## Features

- Supports Chrome, Safari, Edge, Firefox, and Tor browser profiles.
- Synchronous API (Async planned).
- Custom `libcurl` options for exact JA3/JA4 matching.
