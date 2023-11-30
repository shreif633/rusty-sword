use clap::Parser;
use dotenvy::dotenv;

mod framework;
mod requests;
mod responses;
mod plugins;

mod sniffer;
mod server;
mod sandbox;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    mode: String,
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let args = Args::parse();
    dotenv().expect(".env file not found");

    match args.mode.as_str() {
        "sandbox" => sandbox::start().await,
        "sniffer" => sniffer::start("0.0.0.0:30002", "25.1.195.206:30001", false, true, true).await,
        _ => server::start("0.0.0.0:30002").await,
    }
}