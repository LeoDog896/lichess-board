use async_stream::try_stream;
use anyhow::Result;
use futures_util::TryStreamExt;
use reqwest::{Url, Response};
use tokio::io::AsyncBufReadExt;
use tokio_stream::Stream;
use tokio_stream::wrappers::LinesStream;
use tokio_util::io::StreamReader;

pub struct LichessClient {
    token: String,
    base: Url,
    client: reqwest::Client,
}

#[derive(Debug)]
pub enum PlayerType {
    White,
    Black,
}

#[derive(Debug)]
pub enum EventSource {
    Lobby,
    Friend,
    Ai,
    Api,
    Tournament,
    Position,
    Import,
    ImportLive,
    Simul,
    Relay,
    Pool,
    Swiss,
}

#[derive(Debug)]
pub enum GameStatus {
    Created,
    Started,
    Aborted,
    Mate,
    Resign,
    Stalemate,
    Timeout,
    Draw,
    OutOfTime,
    Cheat,
    NoStart,
    UnknownFinish,
    VariantEnd,
}

#[derive(Debug)]
pub struct Compat {
    bot: bool,
    board: bool,
}

#[derive(Debug)]
pub struct ChallengeUser {
    rating: i32,
    provisional: bool,
    online: bool,
    id: String,
    name: String,
    title: String,
    patron: bool,
}

#[derive(Debug)]
pub enum ChallengeStatus {
    Created,
    Offline,
    Canceled,
    Declined,
    Accepted,
}

#[derive(Debug)]
pub enum TimeControl {
    Speed {
        limit: i32,
        increment: i32,
        show: String,
    },
    Unlimited,
    Correspondence {
        days_per_turn: i32,
    },
}

#[derive(Debug)]
pub struct Perf {
    icon: String,
    name: String,
}

#[derive(Debug)]
pub struct Challenge {
    id: String,
    url: String,
    status: String,
    challenger: ChallengeUser,
    dest_user: ChallengeUser,
    variant: String,
    rated: bool,
    time_control: TimeControl,
    color: String,
    perf: Perf,
    direction: String,
    initial_fen: String,
    decline_reason: String,
    decline_reason_key: String,
}

#[derive(Debug)]
pub enum UserEvent {
    GameStart {
        id: String,
        source: EventSource,
        status: GameStatus,
        winner: PlayerType,
        compat: Compat,
    },
    GameFinish {
        id: String,
        source: EventSource,
        status: GameStatus,
        winner: PlayerType,
        compat: Compat,
    },
    Challenge(Challenge),
    ChallengeCancelled(Challenge),
    ChallengeDenied {
        id: String,
    },
}

/// The low-level lichess client.
/// Sends and receives directly from the lichess API.
impl LichessClient {
    /// Instantiates a new lichess client.
    /// You can generate a token at https://lichess.org/account/oauth/token
    pub fn new(token: &str) -> LichessClient {
        LichessClient {
            token: token.to_string(),
            base: Url::parse("https://lichess.org").expect("Could not parse base URL"),
            client: reqwest::Client::new(),
        }
    }

    async fn get(&self, path: &str) -> Result<Response, reqwest::Error> {
        self.client.get(self.base.join(path).expect("Could not add API endpoint!"))
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await
    }

    pub async fn email(&self) -> Result<String> {
        let req = self.get("/api/account/email").await?;

        Ok(req.text().await?)
    }

    /// Stream events from the user (e.g. challenges)
    /// This uses the `/api/stream/event` endpoint
    pub async fn stream_events(&self) -> Result<impl Stream<Item = Result<UserEvent>>> {
        let req = self.get("/api/stream/event").await?;

        let bytes_stream = StreamReader::new(req.bytes_stream().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)));
        let lines = bytes_stream.lines();
        let lines_stream = LinesStream::new(lines);

        let stream = try_stream! {
            for await line in lines_stream {
                let line = line?;
                println!("{}", line);
                yield UserEvent::ChallengeDenied { id: "1".to_string() };
            }
        };

        Ok(stream)
    }
}
