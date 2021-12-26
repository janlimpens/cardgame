#![allow(dead_code)]
#![allow(unused_variables)]

use rand::seq::SliceRandom;
// use std::collections::HashMap;

use crate::card::{Card, Facing, Stack, Suite};

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
    color: String,
}

impl Deck {
    pub fn build(values: &[u8], color: String) -> Self {
        let mut deck = Deck {
            cards: Vec::new(),
            color,
        };
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
        println!("A {color} deck has been created.");
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
    pub stacks: Vec<Stack>,
    players: Vec<Player>,
}

impl Table {
    
    pub fn new() -> Self {
        Table {
            stacks: Vec::new(),
            players: Vec::new(),
        }
    }

    pub fn add_player(&mut self, name: String) -> Result<&Player, String> {
        self.players.push(Player::new(name));
        return match self.players.last() {
            Some(p) => Ok(p),
            None => Err(String::from("This player has already been registered.")),
        };
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    const values: [u8; 5] = [11, 12, 13, 10, 1];
    
    fn vec_compare<T: std::cmp::PartialEq>(va: &[T], vb: &[T]) -> bool {
        (va.len() == vb.len()) &&  // zip stops at the shortest
         va.iter()
           .zip(vb)
           .all(|(a,b)| *a==*b)
    }
    
    fn setup_deck() -> Deck {
        Deck::build(&values, String::from("black"))
    }
    
    #[test]
    fn can_create_a_deck() {
        let deck = setup_deck();
        assert_eq!(deck.number_remaining_cards(), 20);
    }

    #[test]
    fn can_take_a_card_from_top() {
        let mut deck = setup_deck();
        let mut top_card = deck.take_from_top().unwrap();
        assert_eq!(deck.number_remaining_cards(), 19);
        top_card.turn_around();
        assert!(top_card.to_string() == String::from("Ace of Pikes"));
    }

    #[test]
    fn can_remove_a_few_cards_from_the_deck() {
        let mut deck = setup_deck();
        let some_cards = deck.take(5);
        assert_eq!(some_cards.len(), 5);
        assert_eq!(deck.number_remaining_cards(), 15);
    }

    #[test]
    fn can_shuffle_deck() {
        let mut deck = setup_deck();
        deck.shuffle();
        let some_cards = deck.take(5);
        let cmp_values: Vec<u8> = some_cards.iter().map(|c| c.value).collect();
        assert!(
            !vec_compare(&cmp_values, &values),
            "no shuffle happened in between"
        );

        let mut table = Table::new();
        let alice = table.add_player(String::from("alice"));
        let bob = table.add_player(String::from("bob"));
    }
}
