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
    // facing happens in the context of a game. a card is not facing anywhere by itself
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

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_card() -> Card {
        Card {
            value: 12,
            suite: Suite::Hearts,
            facing: Facing::Down,
        }
    }

    #[test]
    fn can_create_a_card() {
        let card = setup_card();
        assert_eq!(card.value, 12);
    }

    #[test]
    fn can_turn_around_card() {
        let mut first_card = setup_card();
        assert_eq!(first_card.to_string(), "a card");
        first_card.turn_around();
        assert_eq!(first_card.to_string(), "Queen of Hearts");
    }
}
