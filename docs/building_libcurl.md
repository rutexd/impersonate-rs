# Building libcurl-impersonate (Advanced)

To support the latest browser profiles (e.g., Chrome 124+, Safari 17+), you may need to build `curl-impersonate` from source with the latest patches, as the system packages or older releases might not include them yet.

## Build Instructions (Linux)

1.  Clone the repository (or the `curl_cffi` fork which often has newer signatures):
    ```bash
    git clone https://github.com/lexiforest/curl-impersonate.git
    cd curl-impersonate
    ```

2.  Build the Chrome version:
    ```bash
    mkdir build && cd build
    ../configure
    make chrome-build
    ```

3.  Install (or point `LD_LIBRARY_PATH`):
    ```bash
    sudo make chrome-install
    sudo ldconfig
    ```

4.  Verify with `impersonate-rs`:
    ```bash
    cargo run --example verify_browsers
    ```
