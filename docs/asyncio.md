# Asyncio (Tokio)

## Current Status

`impersonate-rs` primarily exposes a **synchronous (blocking)** API because `libcurl` is fundamentally blocking.

However, you can easily use it in an `async` context (like Tokio) by wrapping requests in `task::spawn_blocking`.

## Example with Tokio

```rust
use impersonate_rs::{Client, Browser};
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .impersonate(Browser::Chrome124)
        .build();

    // Spawn blocking task
    let response_text = task::spawn_blocking(move || {
        let resp = client.get("https://tls.browserleaks.com/json").send()?;
        resp.text()
    }).await??;

    println!("Response: {}", response_text);
    Ok(())
}
```

## High Concurrency

For high concurrency (hundreds of requests), be aware that `spawn_blocking` uses a thread pool. While efficient for CPU-bound tasks, `libcurl` handles are strictly single-threaded.

**Best Practice**: Create a separate `Client` (or `Session`) for each thread/task if you need isolation, or use a pool of clients. Since `Client` is `Clone` and lightweight (mostly configuration), cloning is cheap. `Session` holds a `RefCell<Easy>`, so it cannot be shared across threads directly (it is `!Sync`).

If you need to share a session across async tasks, you would need to wrap it in a `Mutex` (blocking the thread while in use) or pass it via message passing.

## Future Plans

We are evaluating `curl-multi` integration or `async-curl` crate usage to provide a true `async/.await` API in the future, similar to `reqwest`.
