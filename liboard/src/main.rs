use anyhow::Result;
use clap::Parser;
use futures_util::{pin_mut, StreamExt};
use lichess_client::LichessClient;

#[derive(Parser, Debug)]
enum Subcommand {
    /// Get your registered email address
    Email,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// A simple lichess client.
/// To get a token, go to https://lichess.org/account/oauth/token
struct Args {
    /// The token to use for the lichess API
    #[clap(short, long)]
    token: String,

    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[tokio::main]
async fn main() {
    main_err().await.unwrap();
}

async fn main_err() -> Result<()> {
    let args = Args::parse();

    let client = LichessClient::new(&args.token);

    match args.subcommand {
        Subcommand::Email => {
            let email = client.email().await?;
            println!("{}", email);
        }
    }

    // let stream = client.stream_events().await?;

    // pin_mut!(stream);

    // while let Some(event) = stream.next().await {
    //     println!("{:?}", event);
    // }

    Ok(())
}
