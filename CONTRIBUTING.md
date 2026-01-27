# Contributing to impersonate-rs

Thank you for your interest in contributing to `impersonate-rs`! We welcome contributions from the community.

## Prerequisites

- Rust (latest stable)
- `libcurl-impersonate` installed on your system (for running integration tests)

## Getting Started

1.  Clone the repository.
2.  Install dependencies: `cargo build`.
3.  Run tests: `cargo test`.

## Style Guide

- Follow standard Rust formatting (`cargo fmt`).
- Ensure no warnings with `cargo clippy`.
- Document public APIs.

## Pull Requests

1.  Fork the repository.
2.  Create a feature branch.
3.  Commit your changes.
4.  Push to your fork and submit a Pull Request.
5.  Ensure all tests pass.

## Testing

If you do not have `libcurl-impersonate` installed, unit tests will run using a stubbed FFI function. Integration tests that require actual network spoofing will fail or behave like standard curl.

## License

By contributing, you agree that your contributions will be licensed under the project's license.
