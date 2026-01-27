# Quick Start

## Installation

Add `impersonate-rs` to your `Cargo.toml`:

```toml
[dependencies]
impersonate-rs = "0.1.0"
```

## Basic Requests

Perform a simple GET request with browser impersonation:

```rust
use impersonate_rs::{Client, Browser, Result};

fn main() -> Result<()> {
    let client = Client::builder()
        .impersonate(Browser::Chrome124)
        .build();

    let response = client.get("https://tls.browserleaks.com/json").send()?;
    println!("{}", response.text()?);
    Ok(())
}
```

## Proxies

You can set a proxy for the client:

```rust
let client = Client::builder()
    .impersonate(Browser::Chrome124)
    .proxy("http://user:pass@127.0.0.1:8080")
    .build();
```

## Custom Headers

Override default headers:

```rust
let client = Client::builder()
    .impersonate(Browser::Chrome124)
    .default_headers(false) // Disable default headers if needed
    .build();

let response = client.get("https://httpbin.org/headers")
    .header("User-Agent", "Custom Agent")?
    .send()?;
```

## Form Data & JSON

Send URL-encoded forms or JSON payloads:

```rust
use serde::Serialize;

#[derive(Serialize)]
struct Data {
    foo: String,
}

let client = Client::new();

// Form (application/x-www-form-urlencoded)
client.post("https://httpbin.org/post")
    .form(&Data { foo: "bar".into() })?
    .send()?;

// JSON (application/json)
client.post("https://httpbin.org/post")
    .json(&Data { foo: "bar".into() })?
    .send()?;
```

## Sessions (Cookies)

Use `Session` to persist cookies across requests:

```rust
use impersonate_rs::{Client, Session};

let client = Client::builder()
    .impersonate(Browser::Chrome124)
    .build();

let session = Session::new(client);

// Cookies set here...
session.get("https://httpbin.org/cookies/set/foo/bar")?;

// ...are sent here
let resp = session.get("https://httpbin.org/cookies")?;
println!("{}", resp.text()?);
```

## Advanced Fingerprinting

For precise control over TLS (JA3) and HTTP/2 (Akamai) fingerprints:

```rust
let client = Client::builder()
    .ja3("771,4865-4866-4867-49195-49199-49196-49200-52393-52392-49171-49172-156-157-47-53,0-23-65281-10-11-35-16-5-13-18-51-45-43-27-17513,29-23-24,0")
    .akamai("1:65536,2:0,3:1000,4:6291456,6:262144|15663105|0|m,a,s,p")
    .build();
```
