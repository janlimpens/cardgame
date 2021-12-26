use std::collections::HashMap;

pub struct Server {
    players: HashMap<string, Player>,
    games: HashMap<string, Game>
}

pub struct Game {
    title: String,
}

pub struct Player {
    name: String,
}
