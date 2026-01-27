use impersonate_rs::{Browser, Client};
use std::str::FromStr;

fn main() {
    let browsers = [
        "chrome99",
        "chrome100",
        "chrome101",
        "chrome104",
        "chrome107",
        "chrome110",
        "chrome116",
        "chrome119",
        "chrome120",
        "chrome123",
        "chrome124",
        "chrome131",
        "chrome133a",
        "chrome136",
        "chrome142",
        "chrome99_android",
        "chrome131_android",
        "edge99",
        "edge101",
        "safari15_3",
        "safari15_5",
        "safari17_0",
        "safari17_2_ios",
        "safari18_0",
        "safari18_0_ios",
        "safari18_4",
        "safari18_4_ios",
        "safari260",
        "safari260_ios",
        "safari2601",
        "firefox133",
        "firefox135",
        "firefox144",
        "tor145",
    ];

    println!("{:<20} | {:<10} | {:<30}", "Browser", "Result", "Note");
    println!("{:-<20}-|-{:-<10}-|-{:-<30}", "", "", "");

    for browser_str in browsers.iter() {
        let browser = match Browser::from_str(browser_str) {
            Ok(b) => b,
            Err(_) => {
                println!(
                    "{:<20} | {:<10} | {:<30}",
                    browser_str, "SKIP", "Parsing failed"
                );
                continue;
            }
        };

        let client = Client::builder().impersonate(browser).build();

        // Use a lightweight target that doesn't rate limit easily, or just check connectivity
        // example.com is good for basic TLS handshake verification
        match client.get("https://example.com").send() {
            Ok(resp) => {
                println!(
                    "{:<20} | {:<10} | Status: {}",
                    browser_str,
                    "OK",
                    resp.status()
                );
            }
            Err(e) => {
                // Check if it's an impersonation error (libcurl error)
                let msg = e.to_string();
                let status = if msg.contains("Impersonation error") || msg.contains("Curl error") {
                    "FAIL"
                } else {
                    "ERROR"
                };
                // Truncate error message for display
                let short_msg = if msg.len() > 30 { &msg[0..27] } else { &msg };
                println!("{:<20} | {:<10} | {}", browser_str, status, short_msg);
            }
        }
    }
}
