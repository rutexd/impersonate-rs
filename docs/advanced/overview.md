# Advanced Topics

## Low-Level API

`impersonate-rs` is built on top of `curl-sys` and `libcurl-impersonate`. While the high-level `Client` API covers most use cases, you can access the underlying `curl::easy::Easy` handle if you need to set specific libcurl options that are not exposed by the wrapper.

This is generally done by creating a `Session` (which holds the `Easy` handle) or modifying the code to expose the handle. Currently, the `Session` struct keeps the `Easy` handle internal to ensure safety and state consistency.

If you need raw access, you might need to fork the crate or submit a PR to expose the handle safely.

## Custom CURL Options

The `ClientBuilder` allows setting many common options. However, if you need to set arbitrary `CURLOPT_*` values, you currently need to use the `impersonate-rs` library in a way that allows access to the internal `Easy` handle, or suggest a new feature.

## HTTP Versions

By default, `curl-impersonate` (and thus `impersonate-rs`) attempts to negotiate the best available HTTP version (often HTTP/2 or HTTP/3 depending on the build).

To force a specific HTTP version (e.g. for testing or bypass):

```rust
// Not yet exposed in the public API of ClientBuilder.
// Planned for future release:
// .http_version(HttpVersion::V2)
```

## TLS Fingerprinting Details

When you use `.impersonate(Browser::Chrome124)`, the library applies a complex set of configurations to `libcurl`, including:

1.  **Cipher Suite List**: Exact order and selection of ciphers.
2.  **TLS Extensions**: Permutation and selection of extensions (ALPN, supported_versions, etc.).
3.  **Curve Preference**: Order of elliptic curves.
4.  **HTTP/2 Settings**: Window size, frame size, header table size.
5.  **Pseudo-Header Order**: The order of `:method`, `:scheme`, etc.

If you use `.ja3()` or `.akamai()`, you are overriding these presets with your own raw strings.

### JA3 Format
The JA3 string format used here is the standard:
`SSLVersion,Ciphers,Extensions,EllipticCurves,EllipticCurvePointFormats`

### Akamai HTTP/2 Fingerprint
The format is:
`Settings|WindowUpdate|StreamPriority|HeaderOrder`

## Proxy Support

The `proxy` parameter supports various schemes:
- `http://user:pass@host:port`
- `https://user:pass@host:port`
- `socks5://user:pass@host:port`

Note that `libcurl` handles the proxy connection. For `https` proxies, `libcurl` establishes a TLS tunnel to the proxy.

## Troubleshooting

### "Please enable JS" (403 Forbidden)
If you see this from Cloudflare/Datadome targets (like Idealista), it means:
1.  **TLS Handshake Passed**: The WAF accepted your Client Hello.
2.  **App Layer Failed**: The WAF served an interstitial page requiring JavaScript execution.

`impersonate-rs` handles the **Network Layer** (TLS/HTTP). It does **not** execute JavaScript. To bypass these pages, you need a solution that can run the challenge JS (e.g., a headless browser or a specialized solver), or you need to find an endpoint that is less protected.

### "Symbol not found"
If you get linker errors about missing `curl_easy_impersonate` symbols:
1.  Ensure `libcurl-impersonate` is installed in your library path (`/usr/local/lib`, etc.).
2.  Run `sudo ldconfig`.
3.  Check `LD_LIBRARY_PATH`.
