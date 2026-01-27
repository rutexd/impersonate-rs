# Comparison with other Libraries

## vs `reqwest`

| Feature | `impersonate-rs` | `reqwest` |
| :--- | :--- | :--- |
| **TLS Fingerprinting** | ✅ Built-in (Chrome, Firefox, Safari) | ❌ Standard OpenSSL/Rustls |
| **HTTP/2 Fingerprinting** | ✅ Built-in (Akamai) | ❌ Standard |
| **API Style** | Synchronous (Blocking) | Async (Tokio) & Blocking |
| **Backend** | `libcurl-impersonate` (C) | `hyper` (Rust) |
| **Use Case** | Scraping, Bot Bypass, Security Research | General Purpose HTTP Client |

**Choose `impersonate-rs` if:**
- You are getting blocked by Cloudflare, Akamai, or Datadome TLS fingerprinting.
- You need to mimic a specific browser version exactly.

**Choose `reqwest` if:**
- You are building a standard API client.
- You need pure Rust (no C dependencies).
- You need extreme async performance for thousands of concurrent requests (though `impersonate-rs` can be used with `tokio::task::spawn_blocking`).

## vs `curl_cffi` (Python)

`impersonate-rs` is essentially the Rust equivalent of `curl_cffi`.

| Feature | `impersonate-rs` | `curl_cffi` |
| :--- | :--- | :--- |
| **Language** | Rust | Python |
| **Core Lib** | `libcurl-impersonate` | `libcurl-impersonate` |
| **Performance** | Native (No GC, Zero Cost Abstractions) | Python Overhead |
| **Safety** | Rust Memory Safety | Python/C binding safety |
| **Ecosystem** | Cargo | PyPI |

If you are rewriting a Python scraper in Rust for performance or deployment reasons, `impersonate-rs` is the direct drop-in replacement for the network layer.
