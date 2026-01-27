use clap::Parser;
use impersonate_rs::{Browser, Client};
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "chrome124")]
    impersonate: String,

    #[arg(required = true)]
    url: String,

    #[arg(short, long, default_value = "GET")]
    method: String,

    #[arg(short, long)]
    verbose: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Parse browser
    let browser = Browser::from_str(&args.impersonate).map_err(|e| anyhow::anyhow!("{}", e))?;

    let client = Client::builder().impersonate(browser).build();

    let resp = client.request(&args.method, &args.url).send()?;

    println!("Status: {}", resp.status());

    if args.verbose {
        println!("Headers: {:#?}", resp.text()?);
        // Just printing body as headers placeholder for now in verbose
    }

    if let Ok(text) = resp.text() {
        println!("Body: {}", text);
    } else {
        println!("Body: <binary>");
    }

    Ok(())
}
