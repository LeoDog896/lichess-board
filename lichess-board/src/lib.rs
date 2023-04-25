use async_stream::stream;

use futures_core::stream::Stream;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

pub struct LichessClient {
    token: String,
    base: String,
}

enum PlayerType {
    White,
    Black,
}

enum EventSource {
    Lobby,
    Friend,
    AI,
    API,
    Tournament,
    Position,
    Import,
    ImportLive,
    Simul,
    Relay,
    Pool,
    Swiss,
}

enum GameStatus {
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

struct Compat {
    bot: bool,
    board: bool,
}

struct ChallengeUser {
    rating: i32,
    provisional: bool,
    online: bool,
    id: String,
    name: String,
    title: String,
    patron: bool,
}

enum ChallengeStatus {
    Created,
    Offline,
    Canceled,
    Declined,
    Accepted,
}

enum TimeControl {
    Speed {
        limit: i32,
        increment: i32,
        show: String,
    },
    Unlimited,
    Correspondence {
        daysPerTurn: i32,
    },
}

struct Perf {
    icon: String,
    name: String,
}

struct Challenge {
    id: String,
    url: String,
    status: String,
    challenger: ChallengeUser,
    destUser: ChallengeUser,
    variant: String,
    rated: bool,
    timeControl: TimeControl,
    color: String,
    perf: Perf,
    direction: String,
    initialFen: String,
    declineReason: String,
    declineReasonKey: String,
}

enum UserEvent {
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

impl LichessClient {
    /// Instantiates a new lichess client.
    /// You can generate a token at https://lichess.org/account/oauth/token
    fn new(token: &str) -> LichessClient {
        LichessClient {
            token: token.to_string(),
            base: "https://lichess.org".to_string(),
        }
    }

    /// Stream events from the user (e.g. challenges)
    /// This uses the `/api/stream/event` endpoint
    async fn stream() -> impl Stream<Item = UserEvent> {
        stream! {
            yield UserEvent::ChallengeDenied { id: "1".to_string() }
        }
    }
}
