# WebSockets

## Current Status

**WebSockets are NOT currently supported** in `impersonate-rs`.

While `libcurl` has recently added WebSocket support (experimental in newer versions), `curl-impersonate` focuses on TLS fingerprinting for HTTPS.

If you need WebSocket impersonation, you might need to:
1.  Use `impersonate-rs` to perform the initial HTTP Handshake / Upgrade request to get the session cookies/tokens.
2.  Pass those headers/cookies to a specialized Rust WebSocket crate like `tungstenite` or `tokio-tungstenite`.

*Note: Standard WebSocket clients in Rust (like `tungstenite`) do NOT support JA3 fingerprinting spoofing out of the box. If the target server fingerprints the WebSocket TLS handshake, you will still be blocked.*

This feature is tracked for future development.
