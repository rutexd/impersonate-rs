use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const CURL_IMPERSONATE_VERSION: &str = "v1.5.6";
const GITHUB_RELEASE_URL: &str =
    "https://github.com/lexiforest/curl-impersonate/releases/download";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CURL_IMPERSONATE_VERSION");

    #[cfg(not(feature = "mock"))]
    {
        let target = env::var("TARGET").expect("TARGET env var not set");
        let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR env var not set"));

        let version = env::var("CURL_IMPERSONATE_VERSION")
            .unwrap_or_else(|_| CURL_IMPERSONATE_VERSION.to_string());

        let archive_name = resolve_archive_name(&target);
        let extract_dir = out_dir.join("curl-impersonate");

        if !extract_dir.join("lib").exists() && !extract_dir.join("libcurl-impersonate.a").exists()
        {
            let url = format!(
                "{}/{}/libcurl-impersonate-{}.{}.tar.gz",
                GITHUB_RELEASE_URL, version, version, archive_name
            );

            println!(
                "cargo:warning=Downloading libcurl-impersonate {} for {}...",
                version, target
            );

            let archive_path =
                out_dir.join(format!("libcurl-impersonate-{}.tar.gz", archive_name));
            download(&url, &archive_path);
            extract(&archive_path, &extract_dir);
            let _ = fs::remove_file(&archive_path);

            println!("cargo:warning=Extracted to {}", extract_dir.display());
        } else {
            println!(
                "cargo:warning=Using cached libcurl-impersonate at {}",
                extract_dir.display()
            );
        }

        emit_link_directives(&extract_dir, &target);
    }
}

fn resolve_archive_name(target: &str) -> String {
    match target {
        "x86_64-pc-windows-msvc" | "x86_64-pc-windows-gnu" => "x86_64-win32".into(),
        "i686-pc-windows-msvc" | "i686-pc-windows-gnu" => "i686-win32".into(),
        "aarch64-pc-windows-msvc" | "aarch64-pc-windows-gnu" => "arm64-win32".into(),
        "x86_64-unknown-linux-gnu" => "x86_64-linux-gnu".into(),
        "x86_64-unknown-linux-musl" => "x86_64-linux-musl".into(),
        "aarch64-unknown-linux-gnu" => "aarch64-linux-gnu".into(),
        "aarch64-unknown-linux-musl" => "aarch64-linux-musl".into(),
        "x86_64-apple-darwin" => "x86_64-macos".into(),
        "aarch64-apple-darwin" => "arm64-macos".into(),
        other => panic!(
            "Unsupported target: {}. Supported: x86_64/i686/aarch64-pc-windows-msvc, \
             x86_64/aarch64-unknown-linux-gnu, x86_64/aarch64-unknown-linux-musl, \
             x86_64/aarch64-apple-darwin",
            other
        ),
    }
}

fn download(url: &str, dest: &Path) {
    let response = ureq::get(url)
        .call()
        .unwrap_or_else(|e| panic!("Failed to download {}: {}", url, e));

    let mut reader = response.into_body().into_reader();
    let mut file = fs::File::create(dest).expect("Failed to create download file");
    std::io::copy(&mut reader, &mut file).expect("Failed to write download");
}

fn extract(archive: &Path, dest: &Path) {
    use flate2::read::GzDecoder;
    use tar::Archive;

    fs::create_dir_all(dest).expect("Failed to create extraction directory");

    let file = fs::File::open(archive).expect("Failed to open archive");
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);

    archive
        .unpack(dest)
        .unwrap_or_else(|e| panic!("Failed to extract archive: {}", e));
}

fn emit_link_directives(impersonate_dir: &Path, target: &str) {
    let is_windows = target.contains("windows");

    if is_windows {
        let lib_dir = impersonate_dir.join("lib");
        println!("cargo:rustc-link-search=native={}", lib_dir.display());

        // Link the static lib which contains all curl symbols + curl_easy_impersonate.
        // curl-sys embeds vanilla curl objects in its rlib, so we'll have duplicates.
        // /WHOLEARCHIVE forces the linker to include all objects from the static lib.
        // /FORCE:MULTIPLE suppresses the duplicate symbol errors.
        let static_lib = lib_dir.join("libcurl-impersonate.lib");
        println!(
            "cargo:rustc-link-arg=/WHOLEARCHIVE:{}",
            static_lib.display()
        );
        println!("cargo:rustc-link-arg=/FORCE:MULTIPLE");

        // Transitive dependencies.
        println!("cargo:rustc-link-lib=ssl");
        println!("cargo:rustc-link-lib=crypto");
        println!("cargo:rustc-link-lib=nghttp2");
        println!("cargo:rustc-link-lib=nghttp3");
        println!("cargo:rustc-link-lib=ngtcp2");
        println!("cargo:rustc-link-lib=ngtcp2_crypto_boringssl");
        println!("cargo:rustc-link-lib=brotlidec");
        println!("cargo:rustc-link-lib=brotlienc");
        println!("cargo:rustc-link-lib=brotlicommon");
        println!("cargo:rustc-link-lib=zstd");
        println!("cargo:rustc-link-lib=zlib");
        println!("cargo:rustc-link-lib=ws2_32");
        println!("cargo:rustc-link-lib=crypt32");
        println!("cargo:rustc-link-lib=normaliz");
        println!("cargo:rustc-link-lib=advapi32");
        println!("cargo:rustc-link-lib=wldap32");

        copy_windows_dlls(impersonate_dir);
    } else {
        println!("cargo:rustc-link-lib=static=curl-impersonate");
        println!("cargo:rustc-link-lib=z");
        println!("cargo:rustc-link-lib=pthread");
        println!("cargo:rustc-link-lib=dl");
        println!("cargo:rustc-link-lib=ssl");
        println!("cargo:rustc-link-lib=crypto");
    }
}

fn copy_windows_dlls(impersonate_dir: &Path) {
    let bin_dir = impersonate_dir.join("bin");
    let lib_dir = impersonate_dir.join("lib");

    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".into());
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let target_dir = manifest_dir.join("target").join(&profile);

    let _ = fs::create_dir_all(&target_dir);

    for dir in [&bin_dir, &lib_dir] {
        if !dir.exists() {
            continue;
        }
        for entry in fs::read_dir(dir).expect("Failed to read dir") {
            let entry = entry.expect("Failed to read dir entry");
            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            if name_str.ends_with(".dll") {
                let dest = target_dir.join(&name);
                let _ = fs::copy(entry.path(), &dest);
                println!("cargo:warning=Copied {} to {}", name_str, dest.display());
            }
        }
    }
}
