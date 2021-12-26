use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Server<'a> {
    players: HashMap<&'a str, Player<'a>>,
    games: HashMap<&'a str, Game<'a>>,
}

impl<'a> Server<'a> {
    pub fn new() -> Self {
        Server {
            players: HashMap::new(),
            games: HashMap::new(),
        }
    }

    pub fn register_player(&mut self, name: &'a str) -> Result<&str, &str> {
        match self.players.insert(name, Player { name }) {
            Some(p) => Ok("Player inserted"),
            None => Err("Player already inserted"),
        }
    }

    pub fn handle(&mut self, msg: &'a str) -> Result<&str, &str> {
        let message: Message = serde_json::from_str(&msg).unwrap();
        match message {
            Message::RegisterPlayerRequest { player_name  } => self.register_player(&player_name),
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum Message<'a> {
    RegisterPlayerRequest { player_name: &'a str },
}
pub struct Game<'a> {
    title: &'a str,
}

pub struct Player<'a> {
    name: &'a str,
}
