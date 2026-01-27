//! FFI bindings for curl-impersonate specifics.

use curl_sys::{CURLcode, CURL};
use std::os::raw::{c_char, c_int};

extern "C" {
    /// The core impersonation function added by curl-impersonate.
    ///
    /// # Safety
    /// This function is unsafe because it modifies the internal state of the CURL handle
    /// based on C pointers.
    #[cfg(not(test))]
    pub fn curl_easy_impersonate(
        curl: *mut CURL,
        target: *const c_char,
        default_headers: c_int,
    ) -> CURLcode;
}

#[cfg(test)]
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub unsafe extern "C" fn curl_easy_impersonate(
    curl: *mut CURL,
    target: *const c_char,
    default_headers: c_int,
) -> CURLcode {
    // Stub implementation for tests
    println!("Called curl_easy_impersonate stub");
    curl_sys::CURLE_OK
}

// Custom CURLOPT values for curl-impersonate
// Based on curl_cffi/const.py
pub const CURLOPT_HTTP_VERSION: c_int = 84;
pub const CURLOPT_SSLVERSION: c_int = 32;
pub const CURLOPT_SSL_CIPHER_LIST: c_int = 10000 + 83;
pub const CURLOPT_HTTP2_SETTINGS: c_int = 10000 + 1006;
pub const CURLOPT_HTTP2_WINDOW_UPDATE: c_int = 10008;
pub const CURLOPT_HTTP2_STREAMS: c_int = 10000 + 1010;
pub const CURLOPT_HTTP2_PSEUDO_HEADERS_ORDER: c_int = 10000 + 1005;
pub const CURLOPT_SSL_SIG_HASH_ALGS: c_int = 10000 + 1001;
pub const CURLOPT_SSL_PERMUTE_EXTENSIONS: c_int = 1007;
pub const CURLOPT_SSL_CERT_COMPRESSION: c_int = 10000 + 1003;
pub const CURLOPT_TLS_GREASE: c_int = 1011;
pub const CURLOPT_TLS_EXTENSION_ORDER: c_int = 10000 + 1012;
pub const CURLOPT_STREAM_WEIGHT: c_int = 239;
pub const CURLOPT_STREAM_EXCLUSIVE: c_int = 1013;
pub const CURLOPT_TLS_DELEGATED_CREDENTIALS: c_int = 10000 + 1017;
pub const CURLOPT_TLS_RECORD_SIZE_LIMIT: c_int = 1018;
pub const CURLOPT_HTTP2_NO_PRIORITY: c_int = 1021;
pub const CURLOPT_ECH: c_int = 10000 + 325;

// Constants for behavior
pub const CURL_HTTP_VERSION_2_0: c_int = 3;
pub const CURL_SSLVERSION_TLSV1_2: c_int = 6;
pub const CURL_SSLVERSION_MAX_DEFAULT: c_int = 1 << 16;
