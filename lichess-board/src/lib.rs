use async_stream::stream;
use anyhow::Result;

use futures_core::stream::Stream;

pub struct LichessClient {
    token: String,
    base: String,
}

pub enum PlayerType {
    White,
    Black,
}

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

pub struct Compat {
    bot: bool,
    board: bool,
}

pub struct ChallengeUser {
    rating: i32,
    provisional: bool,
    online: bool,
    id: String,
    name: String,
    title: String,
    patron: bool,
}

pub enum ChallengeStatus {
    Created,
    Offline,
    Canceled,
    Declined,
    Accepted,
}

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

pub struct Perf {
    icon: String,
    name: String,
}

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
            base: "https://lichess.org".to_string(),
        }
    }

    /// Stream events from the user (e.g. challenges)
    /// This uses the `/api/stream/event` endpoint
    pub async fn stream(&self) -> Result<impl Stream<Item = UserEvent>> {
        let req = reqwest::get(format!("{}/{}", self.base, "/api/stream/event")).await?;
        Ok(stream! {
            yield UserEvent::ChallengeDenied { id: "1".to_string() }
        })
    }
}
