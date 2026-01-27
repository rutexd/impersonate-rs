fn main() {
    // Only link if not mocking
    #[cfg(not(feature = "mock"))]
    {
        println!("cargo:rustc-link-search=/usr/local/lib");
        println!("cargo:rustc-link-lib=curl-impersonate-chrome");

        // We might also need to tell curl-sys to NOT link against system curl if possible,
        // or just ensure our symbols are loaded.
        // curl-sys usually links "curl". If we link "curl-impersonate-chrome" too, it might work
        // if the symbols don't conflict or if we prefer the latter.
        // However, curl-impersonate usually REPLACES curl.
    }
}
