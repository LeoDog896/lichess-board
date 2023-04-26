use anyhow::Result;
use lichess_board::LichessClient;
use futures_util::{pin_mut, StreamExt};

#[tokio::main]
async fn main() {
    main_err().await.unwrap();
}

async fn main_err() -> Result<()> {
    let client = LichessClient::new("token");
    let stream = client.stream().await?;

    pin_mut!(stream);

    while let Some(event) = stream.next().await {
        println!("{:?}", event);
    }

    Ok(())
}
