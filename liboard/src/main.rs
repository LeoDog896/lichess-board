use anyhow::Result;
use lichess_board::LichessClient;
use futures_util::{pin_mut, StreamExt};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The token to use for the lichess API
    #[clap(short, long)]
    token: String,
}

#[tokio::main]
async fn main() {
    main_err().await.unwrap();
}

async fn main_err() -> Result<()> {
    let args = Args::parse();

    let client = LichessClient::new(&args.token);
    let stream = client.stream().await?;

    pin_mut!(stream);

    while let Some(event) = stream.next().await {
        println!("{:?}", event);
    }

    Ok(())
}
