#![allow(dead_code)]
#![allow(unused_variables)]

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub struct Server {
    players: HashMap<String, Player>,
    games: HashMap<String, Game>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            players: HashMap::new(),
            games: HashMap::new(),
        }
    }

    pub fn register_player(&mut self, name: &String) -> Result<&Player, &str> {
        let player = Player {
            name: name.to_string(),
        };
        match self.players.insert(name.clone(), player) {
            None => Ok(self.players.get(&*name).unwrap()),
            Some(_) => Err("Player already inserted"),
        }
    }

    pub fn register_game(&mut self, player_name: &String, game_title: &String) -> Result<&Game, &str> {
        let game_id = game_title.clone(); // maybe some better id
        if self.games.get(&game_id).is_some() {
            return Err("Game already registered.");
        }
        if self.players.get(player_name).is_none() {
            return Err("Player unknown.");
        }
        let mut new_game = Game {
            id: game_id.clone(),
            title: game_title.clone(),
            players: HashSet::new(),
            current_player: player_name.clone(),
        };
        new_game.players.insert(player_name.clone());
        let _ = self.games.insert(game_id.to_string(), new_game); // can't just unwrap this
        self.set_current_player(&game_id, &player_name).unwrap();
        Ok(self.games.get(&game_id).unwrap())
    }

    pub fn set_current_player(&mut self, game_id: &String, player_name: &String) -> Result<&Player, &str> {
        if !self.players.contains_key(player_name) {
            return Err("Player unknown");
        }
        if !self.games.contains_key(game_id) {
            return Err("Game unknown");
        }
        let game = self.games.get_mut(game_id).unwrap();
        if !game.players.contains(player_name) {
            return Err("Player isn't registered in the game.");
        }
        game.current_player = player_name.clone();
        return Ok(self.players.get(player_name).unwrap());
    }

    pub fn get_current_player(&self, game_id: &String) -> Option<&Player> {
        if !self.games.contains_key(game_id) {
            return None;
        }
        let game = self.games.get(game_id).unwrap();
        self.players.get(&game.current_player)
    }

    pub fn add_player_to_game(&mut self, game_id: &str, player_name: &str) -> Result<&Player, &str> {
        if !self.players.contains_key(player_name) {
            return Err("Player unknown");
        }
        if !self.games.contains_key(game_id) {
            return Err("Game unknown");
        }
        let game = self.games.get_mut(game_id).unwrap();
        game.players.insert(player_name.to_string());
        Ok(self.players.get(player_name).unwrap())
    }

    pub fn handle(&mut self, request: String) -> String {
        let request: Request = serde_json::from_str(&request).unwrap();
        let response = match request {
            Request::RegisterPlayer { player_name } => {
                let result = self.register_player(&player_name);
                let message = match result {
                    Ok(_) => "Player registered sucessfully.",
                    Err(e) => e
                }.to_string();
                Response::RegisterPlayer {player_name, success: result.is_ok(), message }
            }
        };
        serde_json::to_string(&response).unwrap()
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub(crate) enum Request {
    RegisterPlayer { player_name: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub(crate) enum Response {
    RegisterPlayer { 
        success: bool,
        message: String,
        player_name: String 
    },
}
pub struct Game {
    id: String,
    title: String,
    players: HashSet<String>,
    current_player: String,
}

impl Game {}

pub struct Player {
    name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_server() {
        let s = Server::new();
    }
    #[test]
    fn registers_valid_player() {
        let mut s = Server::new();
        let reg = s.register_player(&"Hansi".to_string());
        assert!(reg.is_ok());
    }
    #[test]
    fn register_player_twice_fails() {
        let mut s = Server::new();
        let _ = s.register_player(&"Hansi".to_string());
        let reg = s.register_player(&"Hansi".to_string());
        assert!(reg.is_err());
    }
    #[test]
    fn registering_a_new_game_without_a_valid_user_fails() {
        let mut s = Server::new();
        let player_name = String::from("Hansi");
        let game_title = String::from("Schnapsen01");
        let reg = s.register_game(&player_name, &game_title);
        assert!(reg.is_err());
    }
    #[test]
    fn registering_a_new_game_with_a_valid_user_succeeds() {
        let mut s = Server::new();
        let player_name = String::from("Hansi");
        let game_title = String::from("Schnapsen01");
        let _ = s.register_player(&"Hansi".to_string());
        let reg = s.register_game(&player_name, &game_title);
        assert!(reg.is_ok());
    }
    #[test]
    fn can_set_current_player() {
        let mut s = Server::new();
        let hansi = String::from("Hansi");
        let fritzi = String::from("Fritzi");
        let game_title = String::from("Schnapsen01");
        s.register_player(&hansi).unwrap();
        s.register_player(&fritzi).unwrap();
        let game = s.register_game(&hansi, &game_title).unwrap();
        assert!(s.get_current_player(&game_title).unwrap().name == hansi);
        s.add_player_to_game(&game_title, &fritzi).unwrap();
        s.set_current_player(&game_title, &fritzi).unwrap();
        assert!(s.get_current_player(&game_title).unwrap().name == fritzi);
    }
    #[test]
    fn cant_set_invalid_current_player() {
        let mut s = Server::new();
        let hansi = String::from("Hansi");
        let fritzi = String::from("Fritzi");
        let game_title = String::from("Schnapsen01");
        s.register_player(&hansi).unwrap();
        s.register_player(&fritzi).unwrap();
        let game = s.register_game(&hansi, &game_title).unwrap();
        assert!(s.get_current_player(&game_title).unwrap().name == hansi);    
        assert!(s.set_current_player(&game_title, &fritzi).is_err());
    }
    #[test]
    fn fails_adding_unregistered_player() {
        let mut s = Server::new();
        let player_name = &"pn".to_string();
        let game_id = &"gid".to_string();
        s.register_player(player_name).unwrap();
        s.register_game(player_name, game_id).unwrap();
        assert!(s.add_player_to_game(game_id, &"xxx".to_string()).is_err());
    }

    #[test]
    fn handles_register_player_request() {
        let req = Request::RegisterPlayer {player_name: "Hansi".to_string()};
        let ser = serde_json::to_string(&req).unwrap();
        let mut s = Server::new();
        let response = s.handle(ser);
        let response:Response = serde_json::from_str(&response).unwrap();
        match response {
            Response::RegisterPlayer{ success, message, player_name } => {
                assert!(success);
                assert!(player_name == "Hansi".to_string());
                assert!(s.players.contains_key(&player_name));
            }
        }       
    }
}
