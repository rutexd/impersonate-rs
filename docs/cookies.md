# Cookies

## Using `Session` for Cookie Management

The most robust way to handle cookies is using the `Session` struct. `libcurl` handles the cookie jar internally in memory.

```rust
use impersonate_rs::{Client, Browser, Session, Result};

fn main() -> Result<()> {
    let client = Client::builder()
        .impersonate(Browser::Chrome124)
        .build();
    
    // Create a session which holds a cookie jar
    let session = Session::new(client);

    // 1. Visit a page that sets a cookie
    session.get("https://httpbin.org/cookies/set/session_id/12345")?;

    // 2. Visit another page, the cookie is sent automatically
    let resp = session.get("https://httpbin.org/cookies")?;
    println!("Cookies sent: {}", resp.text()?);
    
    Ok(())
}
```

## Exporting/Importing Cookies

Currently, `impersonate-rs` relies on `libcurl`'s internal cookie engine. 

- **Loading from file**: You can configure the underlying curl handle to read from a Netscape-formatted `cookies.txt` file using `CURLOPT_COOKIEFILE`.
- **Saving to file**: You can configure it to write to a file using `CURLOPT_COOKIEJAR`.

*Note: Direct API support for `load_cookies()` and `save_cookies()` similar to Python's `pickle` or `dict` methods is planned for a future release.*

## Discarding Cookies

To discard cookies (reset the session), simply drop the `Session` instance and create a new one.

```rust
// Session 1
let session1 = Session::new(client.clone());
session1.get("...")?;

// Session 2 (Fresh, no cookies from Session 1)
let session2 = Session::new(client.clone());
```
