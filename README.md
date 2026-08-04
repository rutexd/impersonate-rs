# impersonate-rs

[![Crates.io](https://img.shields.io/crates/v/impersonate-rs.svg)](https://crates.io/crates/impersonate-rs)
[![Documentation](https://docs.rs/impersonate-rs/badge.svg)](https://docs.rs/impersonate-rs)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A high-performance Rust wrapper for [curl-impersonate](https://github.com/lexiforest/curl-impersonate). This crate provides a safe, ergonomic, and idiomatic Rust interface for performing HTTP requests that mimic specific browser TLS fingerprints (Chrome, Firefox, Safari, Edge) to bypass sophisticated anti-bot protections.

## 🚀 Features

-   **Browser Impersonation**: Built-in support for mimicking modern browsers (Chrome, Edge, Safari, Firefox).
-   **Custom Fingerprinting**: Low-level control over JA3 (TLS) and Akamai (HTTP/2) fingerprints.
-   **High Level API**: Ergonomic `Client` and `RequestBuilder` similar to `reqwest`.
-   **Strongly Typed**: `Browser` enum ensures valid profile selection.
-   **Header Consistency**: Automatically manages headers to match the impersonated browser.

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
impersonate-rs = "0.1.0"
```

### System Requirements

This crate links against `libcurl-impersonate`. You must have the shared library installed on your system.

**Linux (Debian/Ubuntu):**
```bash
# Example for installing curl-impersonate-chrome
sudo apt install build-essential pkg-config cmake ninja-build curl autoconf automake libtool
# Follow build instructions from https://github.com/lexiforest/curl-impersonate
```

**Development Mode:**
If you don't have the library installed yet, you can build with the `mock` feature to stub the FFI calls:
```toml
[dependencies]
impersonate-rs = { version = "0.1.0", features = ["mock"] }
```

## ⚡ Usage

### Basic Browser Impersonation

```rust
use impersonate_rs::{Client, Browser, Result};

fn main() -> Result<()> {
    // Create a client that impersonates Chrome 124
    let client = Client::builder()
        .impersonate(Browser::Chrome124)
        .build();

    // Perform a GET request
    let response = client.get("https://tls.browserleaks.com/json").send()?;

    // Print the response body
    println!("Response: {}", response.text()?);
    
    Ok(())
}
```

### Streaming Downloads (for Large Files)

```rust
use impersonate_rs::{Client, Browser, Result};
use std::fs::File;
use std::io::Write;

fn main() -> Result<()> {
    let client = Client::builder()
        .impersonate(Browser::Chrome124)
        .build();

    let mut file = File::create("large_file.zip").unwrap();
    
    // Stream the response directly to a file without loading it into memory
    let response = client
        .get("https://example.com/large_file.zip")
        .send_with_callback(|chunk| {
            file.write_all(chunk).unwrap();
            Ok(())
        })?;

    println!("Downloaded {} bytes", response.bytes_received());
    Ok(())
}
```

### Streaming Uploads (for Large Files)

```rust
use impersonate_rs::{Client, Browser, Result};
use std::fs::File;

fn main() -> Result<()> {
    let client = Client::builder()
        .impersonate(Browser::Chrome124)
        .build();

    // Upload a large file without loading it entirely into memory
    let file = File::open("large_video.mp4").unwrap();
    
    let response = client
        .post("https://example.com/upload")
        .body_reader(file)  // Streams the file in chunks
        .send()?;

    println!("Upload status: {}", response.status());
    Ok(())
}
```

### Async Streaming Downloads

```rust
use impersonate_rs::{Client, Browser, Result};
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .impersonate(Browser::Chrome124)
        .build();

    let file = Arc::new(Mutex::new(File::create("file.zip").unwrap()));
    let file_clone = file.clone();
    
    let response = client
        .get("https://example.com/file.zip")
        .send_with_callback_async(move |chunk| {
            file_clone.lock().unwrap().write_all(chunk).unwrap();
            Ok(())
        })
        .await?;

    println!("Downloaded {} bytes", response.bytes_received());
    Ok(())
}
```

### Custom JA3/Akamai Fingerprints

For advanced users who need to rotate fingerprints dynamically or use custom signatures.

```rust
use impersonate_rs::{Client, Result};

fn main() -> Result<()> {
    let client = Client::builder()
        // Set a custom JA3 string (TLS fingerprint)
        .ja3("771,4865-4866-4867-49195-49199-49196-49200-52393-52392-49171-49172-156-157-47-53,0-23-65281-10-11-35-16-5-13-18-51-45-43-27-17513,29-23-24,0")
        // Set a custom Akamai string (HTTP/2 fingerprint)
        .akamai("1:65536,2:0,3:1000,4:6291456,6:262144|15663105|0|m,a,s,p")
        .build();

    let response = client.get("https://example.com").send()?;
    println!("{}", response.status());
    
    Ok(())
}
```

## 🛠️ CLI Tool

This crate includes a CLI tool for quick testing and verification.

```bash
# Clone the repo
git clone https://github.com/ajsb85/impersonate-rs.git
cd impersonate-rs

# Run against a target
cargo run --bin impersonate -- https://tls.browserleaks.com/json --impersonate chrome124
```

## 🤝 Contributing

Contributions are welcome! Please check out the [CONTRIBUTING.md](CONTRIBUTING.md) guide.

1.  Fork it
2.  Create your feature branch (`git checkout -b feature/amazing-feature`)
3.  Commit your changes (`git commit -am 'Add some amazing feature'`)
4.  Push to the branch (`git push origin feature/amazing-feature`)
5.  Create a new Pull Request

## 📄 License

This project is licensed under the [MIT License](LICENSE).

## ⚠️ Disclaimer

This library is intended for testing, security research, and interoperability purposes. Users are responsible for ensuring their use of this software complies with all applicable laws and terms of service of the websites they access.
