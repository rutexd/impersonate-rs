use clap::Parser;
use impersonate_rs::{Browser, Client};
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    impersonate: Option<String>,

    #[arg(long)]
    ja3: Option<String>,

    #[arg(long)]
    akamai: Option<String>,

    #[arg(required = true)]
    url: String,

    #[arg(short, long, default_value = "GET")]
    method: String,

    #[arg(short, long)]
    verbose: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut builder = Client::builder();

    if let Some(imp) = args.impersonate {
        let browser = Browser::from_str(&imp).map_err(|e| anyhow::anyhow!("{}", e))?;
        builder = builder.impersonate(browser);
    }

    if let Some(ja3) = args.ja3 {
        builder = builder.ja3(&ja3);
    }

    if let Some(akamai) = args.akamai {
        builder = builder.akamai(&akamai);
    }

    let client = builder.build();

    let resp = client.request(&args.method, &args.url).send()?;

    println!("Status: {}", resp.status());

    if args.verbose {
        // Headers are now available via the public API, let's print them properly
        println!("Headers: {:#?}", resp.headers());
    }

    if let Ok(text) = resp.text() {
        println!("Body: {}", text);
    } else {
        println!("Body: <binary>");
    }

    Ok(())
}
