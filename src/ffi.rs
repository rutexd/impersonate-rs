//! FFI bindings for curl-impersonate specifics.

use curl_sys::{CURLcode, CURL, CURLoption};
use std::os::raw::{c_char, c_int};

#[cfg(not(feature = "mock"))]
extern "C" {
    /// The core impersonation function added by curl-impersonate.
    ///
    /// # Safety
    /// This function is unsafe because it modifies the internal state of the CURL handle
    /// based on C pointers.
    pub fn curl_easy_impersonate(
        curl: *mut CURL,
        target: *const c_char,
        default_headers: c_int,
    ) -> CURLcode;
}

#[cfg(feature = "mock")]
#[allow(unused_variables)]
/// Mock implementation of curl_easy_impersonate for testing.
///
/// # Safety
/// This function is unsafe because it mimics the C FFI function signature.
pub unsafe fn curl_easy_impersonate(
    curl: *mut CURL,
    target: *const c_char,
    default_headers: c_int,
) -> CURLcode {
    println!("[MOCK] curl_easy_impersonate called. Impersonation skipped.");
    curl_sys::CURLE_OK
}

// Custom CURLOPT values for curl-impersonate
// Based on curl_cffi/const.py
// CURLoption is i32 in curl-sys < 0.4.90 and u32 in >= 0.4.90, so follow the alias.
pub const CURLOPT_HTTP_VERSION: CURLoption = 84;
pub const CURLOPT_SSLVERSION: CURLoption = 32;
pub const CURLOPT_SSL_CIPHER_LIST: CURLoption = 10000 + 83;
pub const CURLOPT_HTTP2_SETTINGS: CURLoption = 10000 + 1006;
pub const CURLOPT_HTTP2_WINDOW_UPDATE: CURLoption = 10008;
pub const CURLOPT_HTTP2_STREAMS: CURLoption = 10000 + 1010;
pub const CURLOPT_HTTP2_PSEUDO_HEADERS_ORDER: CURLoption = 10000 + 1005;
pub const CURLOPT_SSL_SIG_HASH_ALGS: CURLoption = 10000 + 1001;
pub const CURLOPT_SSL_ENABLE_ALPS: CURLoption = 1002;
pub const CURLOPT_SSL_PERMUTE_EXTENSIONS: CURLoption = 1007;
pub const CURLOPT_SSL_CERT_COMPRESSION: CURLoption = 10000 + 1003;
pub const CURLOPT_SSL_ENABLE_TICKET: CURLoption = 1004;
pub const CURLOPT_TLS_GREASE: CURLoption = 1011;

pub const CURLOPT_TLS_EXTENSION_ORDER: CURLoption = 10000 + 1012;
pub const CURLOPT_STREAM_WEIGHT: CURLoption = 239;
pub const CURLOPT_STREAM_EXCLUSIVE: CURLoption = 1013;
pub const CURLOPT_TLS_STATUS_REQUEST: CURLoption = 1016;
pub const CURLOPT_TLS_SIGNED_CERT_TIMESTAMPS: CURLoption = 1015;
pub const CURLOPT_TLS_DELEGATED_CREDENTIALS: CURLoption = 10000 + 1017;
pub const CURLOPT_TLS_RECORD_SIZE_LIMIT: CURLoption = 1018;
pub const CURLOPT_HTTP2_NO_PRIORITY: CURLoption = 1021;
pub const CURLOPT_ECH: CURLoption = 10000 + 325;
pub const CURLOPT_SSL_ENABLE_ALPN: CURLoption = 226;

// Constants for behavior
pub const CURL_HTTP_VERSION_2_0: c_int = 3;
pub const CURL_SSLVERSION_TLSV1_2: c_int = 6;
pub const CURL_SSLVERSION_MAX_DEFAULT: c_int = 1 << 16;
