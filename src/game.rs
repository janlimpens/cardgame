#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use rand::seq::SliceRandom;
use std::fmt;
use std::string::ToString;

#[derive(Debug)]
pub struct Card {
    pub value: u8,
    pub suite: Suite,
    pub facing: Facing,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self.value {
            1 => String::from("Ace"),
            11 => String::from("Jack"),
            12 => String::from("Queen"),
            13 => String::from("King"),
            _ => self.value.to_string(),
        };
        match self.facing {
            Facing::Up => write!(f, "{} of {}", name, self.suite),
            _ => write!(f, "a card"),
        }
    }
}

impl Card {
    pub fn turn_around(&mut self) {
        match self.facing {
            Facing::Up => self.facing = Facing::Down,
            Facing::Down => self.facing = Facing::Up,
            Facing::TowardsPlayer => self.facing = Facing::AwayFromPlayer,
            Facing::AwayFromPlayer => self.facing = Facing::TowardsPlayer,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Suite {
    Hearts,
    Tiles,
    Clovers,
    Pikes,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Facing {
    Up,
    Down,
    TowardsPlayer,
    AwayFromPlayer,
}

impl fmt::Display for Suite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
    color: String
}

impl Deck {
    pub fn build(values: &[u8], color: String) -> Self {
        let mut deck = Deck { cards: Vec::new(), color };
        let suites = [Suite::Hearts, Suite::Tiles, Suite::Clovers, Suite::Pikes];
        for &value in values {
            for &suite in suites.iter() {
                deck.cards.push(Card {
                    value,
                    suite,
                    facing: Facing::Down,    
                });
            }
        }
        deck
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn take(&mut self, number: u8) -> Vec<Card> {
        let final_length = self.cards.len().saturating_sub(number.into());
        self.cards.split_off(final_length)
    }

    pub fn take_from_top(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn number_remaining_cards(&self) -> u8 {
        self.cards.len().try_into().unwrap()
    }
}

pub struct Stack {
    pub name: String,
    facing: Facing,
    cards: Vec<Card>,
}

impl Stack {
    pub fn new(name: String, facing: Facing) -> Self {
        Stack {
            name,
            facing,
            cards: Vec::new(),
        }
    }
}

pub struct Player {
    pub name: String,
    pub hand: Stack,
    pub heaps: Vec<Stack>,
}

impl Player {
    pub fn new(name: String) -> Self {
        let hand = Stack::new(format!("{}'s hand", name.as_str()), Facing::TowardsPlayer);
        Player {
            name,
            hand,
            heaps: Vec::new(),
        }
    }
}

pub struct Table {
    pub heaps: Vec<Stack>,
    players: Vec<Player>,
}

impl Table {
    pub fn new() -> Self {
        Table {
            heaps: Vec::new(),
            players: Vec::new(),
        }
    }

    pub fn add_player(&mut self, name: String) -> Result<&Player, String> {
        self.players.push(Player::new(name));
        return match self.players.last() {
            Some(p) => Ok(p),
            None => Err(String::from("This player has already been registered."))
        }
    }
}

fn vec_compare<T: std::cmp::PartialEq>(va: &[T], vb: &[T]) -> bool {
    (va.len() == vb.len()) &&  // zip stops at the shortest
     va.iter()
       .zip(vb)
       .all(|(a,b)| *a==*b)
}
#[test]
fn test() {
    let card =Card {
        value: 12,
        suite:Suite::Hearts,
        facing:Facing::Up,
    };
    assert_eq!(card.value, 12);

    let values = [11, 12, 13, 10, 1];
    let mut deck =Deck::build(&values, String::from("black"));

    assert_eq!(deck.number_remaining_cards(), 20);
    let first_card = deck.take_from_top();
    assert_eq!(deck.number_remaining_cards(), 19);
    if first_card.is_none() {
        return;
    }

    let mut first_card = first_card.unwrap();
    assert_eq!(first_card.value, 1);
    assert_eq!(first_card.to_string(), "a card");
    first_card.turn_around();
    assert_eq!(first_card.to_string(), "Ace of Pikes");
    assert_eq!(first_card.suite,Suite::Pikes);
    deck.shuffle();

    let some_cards = deck.take(5);
    assert_eq!(some_cards.len(), 5);

    let cmp_values: Vec<u8> = some_cards.iter().map(|c| c.value).collect();
    assert!(
        !vec_compare(&cmp_values, &values),
        "no shuffle happened in between"
    );

    let mut table =Table::new();
    let alice = table.add_player(String::from("alice"));
    let bob = table.add_player(String::from("bob"));
}
