# API Reference

## Browser Profiles

The `Browser` enum supports a wide range of browser versions.

### Chrome
- `Browser::Chrome99`
- `Browser::Chrome100` ... `Browser::Chrome124`
- `Browser::Chrome131Android` (Android Mobile)

### Safari
- `Browser::Safari15_3`
- `Browser::Safari18_0`
- `Browser::Safari17_2Ios` (iPhone)

### Firefox
- `Browser::Firefox133`
- `Browser::Firefox135`

### Edge
- `Browser::Edge101`

### Tor
- `Browser::Tor145`

## ClientBuilder

| Method | Description |
| :--- | :--- |
| `.impersonate(Browser)` | Sets the browser profile. |
| `.ja3(str)` | Sets a custom JA3 TLS fingerprint. |
| `.akamai(str)` | Sets a custom Akamai HTTP/2 fingerprint. |
| `.permute_extensions(bool)` | Randomize TLS extension order (default: true). |
| `.verify(bool)` | Verify SSL certificates (default: true). |
| `.timeout(Duration)` | Request timeout. |
| `.proxy(str)` | Proxy URL. |
| `.follow_redirects(bool)` | Follow HTTP 3xx redirects (default: true). |

## RequestBuilder

| Method | Description |
| :--- | :--- |
| `.header(key, val)` | Add a custom header. |
| `.body(vec)` | Set raw body bytes. |
| `.json(obj)` | Serialize object to JSON body. |
| `.form(obj)` | Serialize object to Form URL Encoded body. |
| `.basic_auth(u, p)` | Set HTTP Basic Auth. |

## Response

| Method | Description |
| :--- | :--- |
| `.status()` | HTTP status code (`u32`). |
| `.text()` | Body as `String`. |
| `.bytes()` | Body as `Vec<u8>`. |
| `.json()` | Deserialize body to struct. |
| `.headers()` | Response headers map. |
