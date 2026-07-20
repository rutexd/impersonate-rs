use crate::error::{Error, Result};
use crate::ffi;
use curl::easy::Easy;
use std::collections::HashSet;

// Maps based on curl_cffi/curl_cffi/requests/impersonate.py

pub fn set_ja3_options(easy: &mut Easy, ja3: &str, permute: bool) -> Result<()> {
    let parts: Vec<&str> = ja3.split(',').collect();
    if parts.len() != 5 {
        return Err(Error::Impersonate("Invalid JA3 string format".to_string()));
    }

    let tls_version = parts[0];
    let ciphers = parts[1];
    let extensions = parts[2];
    let curves = parts[3];
    let curve_formats = parts[4];

    // 1. TLS Version
    // JA3 uses 771 for TLS 1.2 (0x0303). curl-impersonate expects CURL_SSLVERSION_TLSv1_2
    // Currently only supporting TLS 1.2 as per curl_cffi implementation
    let tls_version_int = tls_version
        .parse::<u16>()
        .map_err(|_| Error::Impersonate("Invalid TLS version".to_string()))?;
    if tls_version_int != 771 { // 0x0303
         // warn but proceed? curl_cffi asserts it.
         // For now let's enforce 1.2 default logic or just set min version.
    }
    // Set min version to TLS 1.2
    easy.ssl_min_max_version(
        curl::easy::SslVersion::Tlsv12,
        curl::easy::SslVersion::Default,
    )?;

    // 2. Ciphers

    let cipher_list = parse_ciphers(ciphers)?;
    easy.ssl_cipher_list(&cipher_list)?;

    // 3. Extensions
    let extension_ids: HashSet<u16> = extensions
        .split('-')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u16>())
        .collect::<std::result::Result<_, _>>()
        .map_err(|_| Error::Impersonate("Invalid extension ID".to_string()))?;

    toggle_extensions(easy, &extension_ids)?;

    if !permute {
        // Set explicit extension order
        // raw string format "1-2-3" is what curl-impersonate expects for CURLOPT_TLS_EXTENSION_ORDER
        // curl-rs doesn't expose this custom option directly, need unsafe setopt
        set_option_str(easy, ffi::CURLOPT_TLS_EXTENSION_ORDER, extensions)?;
    } else {
        // Enable permutation
        set_option_long(easy, ffi::CURLOPT_SSL_PERMUTE_EXTENSIONS, 1)?;
    }

    // 4. Curves
    let curve_list = parse_curves(curves)?;
    // CURLOPT_SSL_EC_CURVES
    // curl-rs doesn't expose this, use unsafe
    set_option_str(easy, 20000 + 198, &curve_list)?; // CURL_CTRL_SET + 198 (CURLOPT_SSL_EC_CURVES) - CHECK VALUE

    // 5. Curve Formats
    if curve_formats != "0" {
        return Err(Error::Impersonate(
            "Only curve_formats == 0 is supported".to_string(),
        ));
    }

    Ok(())
}

pub fn set_akamai_options(easy: &mut Easy, akamai: &str) -> Result<()> {
    let parts: Vec<&str> = akamai.split('|').collect();
    if parts.len() != 4 {
        return Err(Error::Impersonate(
            "Invalid Akamai string format".to_string(),
        ));
    }

    let settings = parts[0];
    let window_update = parts[1];
    let streams = parts[2];
    let header_order = parts[3];

    // HTTP/2 settings: replace ',' with ';'
    let settings_str = settings.replace(',', ";");
    set_option_str(easy, ffi::CURLOPT_HTTP2_SETTINGS, &settings_str)?;

    // Window Update
    let window_update_val = window_update
        .parse::<i32>()
        .map_err(|_| Error::Impersonate("Invalid window update value".to_string()))?;
    set_option_long(
        easy,
        ffi::CURLOPT_HTTP2_WINDOW_UPDATE,
        window_update_val as i64,
    )?;

    // Streams
    if streams != "0" {
        let streams_val = streams
            .parse::<i32>()
            .map_err(|_| Error::Impersonate("Invalid streams value".to_string()))?;
        set_option_long(easy, ffi::CURLOPT_HTTP2_STREAMS, streams_val as i64)?;
    }

    // Header Order: remove commas
    let header_order_str = header_order.replace(',', "");
    set_option_str(
        easy,
        ffi::CURLOPT_HTTP2_PSEUDO_HEADERS_ORDER,
        &header_order_str,
    )?;

    Ok(())
}

fn set_option_long(easy: &mut Easy, opt: i32, val: i64) -> Result<()> {
    // curl-rs doesn't expose generic setopt_long, so we use the raw handle
    let raw = easy.raw();
    unsafe {
        // The signature for setopt with long is (handle, opt, long_value)
        // Note: in C, it's varargs. In Rust bindings, we need to be careful.
        // curl-sys exposes curl_easy_setopt.
        // Cast opt to u32 (CURLoption is usually uint)
        let code = curl_sys::curl_easy_setopt(raw, opt, val);
        if code != curl_sys::CURLE_OK {
            return Err(Error::Curl(curl::Error::new(code)));
        }
    }
    Ok(())
}

fn set_option_str(easy: &mut Easy, opt: i32, val: &str) -> Result<()> {
    let raw = easy.raw();
    let c_str = std::ffi::CString::new(val)
        .map_err(|e| Error::Impersonate(format!("CString error: {}", e)))?;
    unsafe {
        let code = curl_sys::curl_easy_setopt(raw, opt, c_str.as_ptr());
        if code != curl_sys::CURLE_OK {
            return Err(Error::Curl(curl::Error::new(code)));
        }
    }
    Ok(())
}

fn parse_ciphers(ciphers: &str) -> Result<String> {
    let mut names = Vec::new();
    for id_str in ciphers.split('-') {
        let id = id_str
            .parse::<u16>()
            .or_else(|_| u16::from_str_radix(id_str, 16))
            .map_err(|_| Error::Impersonate(format!("Invalid cipher ID: {}", id_str)))?;

        let name = get_cipher_name(id)
            .ok_or_else(|| Error::Impersonate(format!("Unknown cipher ID: {}", id)))?;
        names.push(name);
    }
    Ok(names.join(":"))
}

fn parse_curves(curves: &str) -> Result<String> {
    let mut names = Vec::new();
    for id_str in curves.split('-') {
        let id = id_str
            .parse::<u16>()
            .or_else(|_| u16::from_str_radix(id_str, 16))
            .map_err(|_| Error::Impersonate(format!("Invalid curve ID: {}", id_str)))?;

        let name = get_curve_name(id)
            .ok_or_else(|| Error::Impersonate(format!("Unknown curve ID: {}", id)))?;
        names.push(name);
    }
    Ok(names.join(":"))
}

fn toggle_extensions(easy: &mut Easy, extension_ids: &HashSet<u16>) -> Result<()> {
    // Logic from curl_cffi `toggle_extensions_by_ids`
    // Default enabled in curl-impersonate (approximate list)
    let default_enabled: HashSet<u16> = [0, 10, 11, 13, 16, 23, 35, 43, 45, 51, 65281]
        .iter()
        .cloned()
        .collect();

    // Enable
    for id in extension_ids.difference(&default_enabled) {
        apply_extension_toggle(easy, *id, true)?;
    }

    // Disable
    for id in default_enabled.difference(extension_ids) {
        apply_extension_toggle(easy, *id, false)?;
    }

    Ok(())
}

fn apply_extension_toggle(easy: &mut Easy, id: u16, enable: bool) -> Result<()> {
    let val = if enable { 1 } else { 0 };
    match id {
        65037 => {
            // ECH
            if enable {
                set_option_str(easy, ffi::CURLOPT_ECH, "grease")?
            } else {
                set_option_str(easy, ffi::CURLOPT_ECH, "")?
            }
        }
        27 => {
            // Compress Cert
            if enable {
                set_option_str(easy, ffi::CURLOPT_SSL_CERT_COMPRESSION, "brotli")?
            } else {
                set_option_str(easy, ffi::CURLOPT_SSL_CERT_COMPRESSION, "")?
            }
        }
        17513 | 17613 => {
            // ALPS
            set_option_long(easy, ffi::CURLOPT_SSL_ENABLE_ALPS, val)?;
        }
        16 => set_option_long(easy, ffi::CURLOPT_SSL_ENABLE_ALPN, val)?,
        5 => set_option_long(easy, ffi::CURLOPT_TLS_STATUS_REQUEST, val)?,
        18 => set_option_long(easy, ffi::CURLOPT_TLS_SIGNED_CERT_TIMESTAMPS, val)?,
        35 => set_option_long(easy, ffi::CURLOPT_SSL_ENABLE_TICKET, val)?,
        _ => {} // Ignore unimplemented
    }
    Ok(())
}

// Maps
fn get_cipher_name(id: u16) -> Option<&'static str> {
    match id {
        0x000A => Some("TLS_RSA_WITH_3DES_EDE_CBC_SHA"),
        0x002F => Some("TLS_RSA_WITH_AES_128_CBC_SHA"),
        0x0033 => Some("TLS_DHE_RSA_WITH_AES_128_CBC_SHA"),
        0x0035 => Some("TLS_RSA_WITH_AES_256_CBC_SHA"),
        0x0039 => Some("TLS_DHE_RSA_WITH_AES_256_CBC_SHA"),
        0x003C => Some("TLS_RSA_WITH_AES_128_CBC_SHA256"),
        0x003D => Some("TLS_RSA_WITH_AES_256_CBC_SHA256"),
        0x0067 => Some("TLS_DHE_RSA_WITH_AES_128_CBC_SHA256"),
        0x006B => Some("TLS_DHE_RSA_WITH_AES_256_CBC_SHA256"),
        0x008C => Some("TLS_PSK_WITH_AES_128_CBC_SHA"),
        0x008D => Some("TLS_PSK_WITH_AES_256_CBC_SHA"),
        0x009C => Some("TLS_RSA_WITH_AES_128_GCM_SHA256"),
        0x009D => Some("TLS_RSA_WITH_AES_256_GCM_SHA384"),
        0x009E => Some("TLS_DHE_RSA_WITH_AES_128_GCM_SHA256"),
        0x009F => Some("TLS_DHE_RSA_WITH_AES_256_GCM_SHA384"),
        0x1301 => Some("TLS_AES_128_GCM_SHA256"),
        0x1302 => Some("TLS_AES_256_GCM_SHA384"),
        0x1303 => Some("TLS_CHACHA20_POLY1305_SHA256"),
        0xC008 => Some("TLS_ECDHE_ECDSA_WITH_3DES_EDE_CBC_SHA"),
        0xC009 => Some("TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA"),
        0xC00A => Some("TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA"),
        0xC012 => Some("TLS_ECDHE_RSA_WITH_3DES_EDE_CBC_SHA"),
        0xC013 => Some("TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA"),
        0xC014 => Some("TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA"),
        0xC023 => Some("TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA256"),
        0xC024 => Some("TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA384"),
        0xC027 => Some("TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA256"),
        0xC028 => Some("TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA384"),
        0xC02B => Some("TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256"),
        0xC02C => Some("TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384"),
        0xC02F => Some("TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256"),
        0xC030 => Some("TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384"),
        0xC035 => Some("TLS_ECDHE_PSK_WITH_AES_128_CBC_SHA"),
        0xC036 => Some("TLS_ECDHE_PSK_WITH_AES_256_CBC_SHA"),
        0xCCA8 => Some("TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256"),
        0xCCA9 => Some("TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256"),
        0xCCAC => Some("TLS_ECDHE_PSK_WITH_CHACHA20_POLY1305_SHA256"),
        _ => None,
    }
}

fn get_curve_name(id: u16) -> Option<&'static str> {
    match id {
        19 => Some("P-192"),
        21 => Some("P-224"),
        23 => Some("P-256"),
        24 => Some("P-384"),
        25 => Some("P-521"),
        29 => Some("X25519"),
        256 => Some("ffdhe2048"),
        257 => Some("ffdhe3072"),
        4588 => Some("X25519MLKEM768"),
        25497 => Some("X25519Kyber768Draft00"),
        _ => None,
    }
}
