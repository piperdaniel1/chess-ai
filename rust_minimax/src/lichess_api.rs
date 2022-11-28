use reqwest::Client;
use std::fs::File;
use hyper::header::{HeaderMap, HeaderValue};
use futures_util::StreamExt;
use std::io::Read;

#[derive(Debug)]
struct Lichess {
    client: reqwest::Client,
    token: String,
}

#[derive(Debug)]
struct Compat {
    bot: bool,
    board: bool,
}

#[derive(Debug)]
struct Challenger {
    id: String,
    name: String,
    title: String,
    rating: u32,
    patron: bool,
    online: bool,
    lag: u32,
}

#[derive(Debug)]
struct DestUser {
    id: String,
    name: String,
    title: String,
    rating: u32,
    provisional: bool,
    online: bool,
    lag: u32,
}

#[derive(Debug)]
struct Variant {
    key: String,
    name: String,
    short: String,
}

#[derive(Debug)]
struct TimeControl {
    type_: String,
    limit: u32,
    increment: u32,
    show: String,
}

#[derive(Debug)]
struct Perf {
    icon: String,
    name: String,
}

#[derive(Debug)]
struct Challenge {
    id: String,
    url: String,
    status: String,
    compat: Compat,
    challenger: Challenger,
    dest_user: DestUser,
    variant: Variant,
    rated: bool,
    time_control: TimeControl,
    color: String,
    speed: String,
    perf: Perf,
}

impl Lichess {
    fn new(token: String) -> Self {
        Self {
            client: Client::new(),
            token,
        }
    }

    async fn block_until_challenge(&self) -> Challenge {
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
                            "challenge" => {
                                let challenge_json = json_chunk["challenge"].to_string();

                                let challenge: Challenge = serde_json::from_str(&challenge_json).unwrap();
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
}

fn get_lichess_token() -> String {
    let mut file = File::open("/home/daniel/personal-projects/chess-ai/rust_minimax/src/.lichess-token").unwrap();
    let mut token = String::new();
    file.read_to_string(&mut token).unwrap();
    token.trim().to_string()
}

mod tests {
    use super::*;

    #[test]
    fn test_block_until_challenge() {
        let lichess = Lichess::new(get_lichess_token());
        let challenge = lichess.block_until_challenge();
        println!("{:?}", challenge);
    }
}