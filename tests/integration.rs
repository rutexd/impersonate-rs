use impersonate_rs::{Browser, Client};

#[test]
fn test_public_api() {
    let _client = Client::builder().impersonate(Browser::Chrome100).build();
    // we can't easily assert on client internals as fields are private
    // but if this compiles, the API is public
}
