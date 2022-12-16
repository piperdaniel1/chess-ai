use reqwest::Client;
use std::fs::File;
use hyper::header::{HeaderMap, HeaderValue};
use futures_util::StreamExt;
use futures_util::Stream;
use std::io::Read;
use serde::Deserialize;
use core::fmt::Error;
use serde_json::json;

#[derive(Debug)]
pub struct Lichess {
    client: reqwest::Client,
    token: String,
}

/*
 * Structs for deserializing the /api/stream/event response if type is "challenge"
 */
#[derive(Debug, Deserialize)]
pub struct Challenger {
    pub id: String,
    pub name: String,
    pub title: Option<String>,
    pub rating: u32,
    pub online: Option<bool>,
}
#[derive(Debug, Deserialize)]
pub struct DestUser {
    pub id: String,
    pub name: String,
    pub title: String,
    pub rating: u32,
    pub provisional: bool,
    pub online: Option<bool>,
    pub lag: Option<u32>,
}
#[derive(Debug, Deserialize)]
pub struct Variant {
    pub key: Option<String>,
    pub name: Option<String>,
    pub short: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct TimeControl {
    #[serde(rename = "type")]
    pub type_: String,
    pub limit: Option<u32>,
    pub increment: Option<u32>,
    pub show: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct Perf {
    pub icon: Option<String>,
    pub name: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct Challenge {
    pub id: String,
    pub url: String,
    pub status: String,
    pub challenger: Challenger,
    #[serde(rename = "destUser")]
    pub dest_user: DestUser,
    pub variant: Variant,
    pub rated: bool,
    #[serde(rename = "timeControl")]
    pub time_control: TimeControl,
    pub color: String,
    pub speed: String,
    pub perf: Perf,
}
/*
 * Structs for deserializing the /api/stream/event response if type is "gameStart"
 */
#[derive(Debug, Deserialize)]
pub struct Opponent {
    pub id: String,
    pub rating: u32,
    pub username: String,
}
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Compat {
    bot: Option<bool>,
    board: Option<bool>,
}
#[derive(Debug, Deserialize)]
pub struct Game {
    #[serde(rename="gameId")]
    pub game_id: String,
    #[serde(rename="fullId")]
    pub full_id: String,
    pub color: String,
    pub fen: String,
    #[serde(rename="hasMoved")]
    pub has_moved: bool,
    #[serde(rename="isMyTurn")]
    pub is_my_turn: bool,
    #[serde(rename="lastMove")]
    pub last_move: String,
    pub opponent: Opponent,
    pub perf: String,
    pub rated: bool,
    #[serde(rename="secondsLeft")]
    pub seconds_left: Option<u32>,
    pub source: String,
    pub speed: String,
    pub variant: Variant,
    pub compat: Option<Compat>,
}

/*
 * Structs for deserializing the /api/bot/game/stream/{gameId} response if type is "gameFull"
 */
pub enum GameEvent {
    GameFull(FullGame),
    GameState(GameState)
}
#[derive(Debug, Deserialize)]
pub struct Clock {
    pub initial: u32,
    pub increment: u32,
}
#[derive(Debug, Deserialize)]
pub struct GameState {
    #[serde(rename="type")]
    pub _type: String,
    pub moves: String,
    pub wtime: u32,
    pub btime: u32,
    pub winc: u32,
    pub binc: u32,
    pub status: String,
}
#[derive(Debug, Deserialize)]
pub struct FullGame {
    pub clock: Option<Clock>,
    #[serde(rename="initialFen")]
    pub initial_fen: String,
    pub state: GameState,
}

impl Lichess {
    pub fn new(token: String) -> Self {
        Self {
            client: Client::new(),
            token,
        }
    }

    pub async fn block_until_challenge(&self) -> Challenge {
        let mut req_header = HeaderMap::new();
        let auth_header = HeaderValue::from_str(&format!("Bearer {}", self.token)).unwrap();
        req_header.insert("Authorization", auth_header);

        let mut res = self.client.get("https://lichess.org/api/stream/event")
            .headers(req_header)
            .send()
            .await
            .unwrap()
            .bytes_stream();

        // Parse the stream
        while let Some(item) = res.next().await {
            match item {
                Ok(bytes) => {
                    if bytes.len() > 1 {
                        let s = String::from_utf8(bytes.to_vec()).unwrap();
                        let json_chunk: serde_json::Value = serde_json::from_str(&s).unwrap();
                        let event_type = json_chunk["type"].as_str().unwrap();

                        match event_type {
                            "challenge" => {
                                let challenge_json = json_chunk["challenge"].to_string();

                                let challenge: Challenge = serde_json::from_str(challenge_json.as_str()).unwrap();
                                //let challenge_id = json_chunk["challenge"]["id"].as_str().unwrap();
                                //return challenge_id.to_string();
                                return challenge
                            },
                            _ => continue,
                        }
                    }
                },
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }

        panic!("No challenge received before stream ended");
    }

    pub async fn block_until_game_start(&self) -> Game {
        let mut req_header = HeaderMap::new();
        let auth_header = HeaderValue::from_str(&format!("Bearer {}", self.token)).unwrap();
        req_header.insert("Authorization", auth_header);

        let mut res = self.client.get("https://lichess.org/api/stream/event")
            .headers(req_header)
            .send()
            .await
            .unwrap()
            .bytes_stream();

        // Parse the stream
        while let Some(item) = res.next().await {
            match item {
                Ok(bytes) => {
                    if bytes.len() > 1 {
                        println!("Received {} bytes", bytes.len());
                        let s = String::from_utf8(bytes.to_vec()).unwrap();
                        let json_chunk: serde_json::Value = serde_json::from_str(&s).unwrap();
                        let event_type = json_chunk["type"].as_str().unwrap();

                        match event_type {
                            "gameStart" => {
                                let req_json = json_chunk["game"].clone();

                                let game: Game = serde_json::from_value(req_json).unwrap();
                                //let challenge_id = json_chunk["challenge"]["id"].as_str().unwrap();
                                //return challenge_id.to_string();
                                return game
                            },
                            _ => continue,
                        }
                    }
                },
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }

        panic!("No game start event received before stream ended");
    }

    pub async fn create_challenge(&self, username: &str) {
        let mut req_header = HeaderMap::new();
        let auth_header = HeaderValue::from_str(&format!("Bearer {}", self.token)).unwrap();
        req_header.insert("Authorization", auth_header);

        let json_body = json!(
            {
                "clock.limit": 180,
                "clock.increment": 2,
                "color": "white",
                "rated": true,
                "variant": "standard",
                "mode": "casual",
            }
        );

        let res = self.client.post(format!("https://lichess.org/api/challenge/{}", username))
            .headers(req_header)
            .json(&json_body)
            .send()
            .await
            .unwrap();

        println!("Response: {:?}", res.text().await);
    }

    pub async fn make_move(&self, game: &Game, move_str: &str) -> Result<(), Error>  {
        for i in 0..10 {
            let mut req_header = HeaderMap::new();
            let auth_header = HeaderValue::from_str(&format!("Bearer {}", self.token)).unwrap();
            req_header.insert("Authorization", auth_header);
            let res = self.client.post(format!("https://lichess.org/api/bot/game/{}/move/{}", game.game_id, move_str))
                .headers(req_header)
                .send()
                .await;

            match res {
                Ok(_) => return Ok(()),
                Err(e) => {
                    println!("Error making a move on attempt #{}: {}", i, e);
                },
            };
        }

        Err(Error)
    }

    pub async fn get_game_stream(&self, game: &Game) -> impl Stream<Item = Result<hyper::body::Bytes, reqwest::Error>> {
        let mut req_header = HeaderMap::new();
        let auth_header = HeaderValue::from_str(&format!("Bearer {}", self.token)).unwrap();
        req_header.insert("Authorization", auth_header);

        let res = self.client.get(format!("https://lichess.org/api/bot/game/stream/{}", game.game_id))
            .headers(req_header)
            .send()
            .await
            .unwrap()
            .bytes_stream();
        
        res
    }

    pub async fn accept_challenge(&self, challenge: Challenge) -> Result<(), Error> {
        let res = self.client.post(format!("https://lichess.org/api/challenge/{}/accept", challenge.id))
            .bearer_auth(&self.token)
            .send()
            .await;

        let res = match res {
            Ok(res) => res,
            Err(_) => {
                return Err(Error);
            }
        };

        if res.status().is_success() {
            Ok(())
        } else {
            Err(Error)
        }
    }

    pub fn parse_game_event(&self, bytes: hyper::body::Bytes) -> Result<GameEvent, Error> {
        let s = String::from_utf8(bytes.to_vec()).unwrap();
        if s.len() <= 1 {
            return Err(Error);
        }
        let json_chunk: serde_json::Value = serde_json::from_str(&s).unwrap();

        if json_chunk["type"] == "gameFull" {
            let game_full_json = json_chunk.to_string();
            let game_full: FullGame = serde_json::from_str(game_full_json.as_str()).unwrap();
            Ok(GameEvent::GameFull(game_full))
        } else if json_chunk["type"] == "gameState" {
            let game_state_json = json_chunk.to_string();
            let game_state: GameState = serde_json::from_str(game_state_json.as_str()).unwrap();
            Ok(GameEvent::GameState(game_state))
        } else {
            Err(Error)
        }
    }

    pub async fn get_current_game(&self) -> Option<Game> {
        let res = self.client.get("https://lichess.org/api/account/playing")
            .bearer_auth(&self.token)
            .send()
            .await;
        
        let res = match res {
            Ok(res) => res,
            Err(_) => {
                panic!("Error getting current game");
            }
        };

        assert!(res.status().is_success());

        let json = res.json::<serde_json::Value>().await.unwrap();
        
        // get length of first_game
        let len = json["nowPlaying"].as_array().unwrap().len();

        if len == 0 {
            return None;
        }

        let game_json = json["nowPlaying"][0].clone();
        let game: Game = serde_json::from_value(game_json).unwrap();

        Some(game)
    }

    pub async fn get_bot_to_challenge(&self) -> String {
        // let nb = json!({"nb": 50});

        let mut res = self.client.get("https://lichess.org/api/bot/online")
            .send()
            .await
            .unwrap()
            .bytes_stream();

        while let Some(chunk) = res.next().await {
            let chunk = chunk.unwrap();
            let s = String::from_utf8(chunk.to_vec()).unwrap();

            match serde_json::from_str::<serde_json::Value>(&s) {
                Ok(json_chunk) => {
                    println!("json_chunk: {:#?}", json_chunk);
                },
                Err(e) => {
                    println!("Error: {}", e);
                }
            }

        }

        todo!()
    }
}

#[allow(dead_code)]
fn get_lichess_token() -> String {
    let mut file = File::open("/home/daniel/personal-projects/chess-ai/rust_minimax/src/.lichess-token").unwrap();
    let mut token = String::new();
    file.read_to_string(&mut token).unwrap();
    token.trim().to_string()
}

mod tests {
    #[allow(dead_code)]
    fn test_block_until_challenge() {
        use tokio::runtime::Runtime;
        use crate::lichess_api::Lichess;
        use crate::lichess_api::get_lichess_token;

        let rt = Runtime::new().unwrap();
        let lichess = Lichess::new(get_lichess_token());

        let challenge = rt.block_on(async {lichess.block_until_challenge().await});
        println!("{:#?}", challenge);
    }
}
