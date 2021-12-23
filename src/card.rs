use rand::seq::SliceRandom;

#[derive(Debug)]
struct Card {
    value: u8,
    suite: Suite,
}
#[derive(Debug, PartialEq, Clone, Copy)]
enum Suite {
    Hearts,
    Tiles,
	Clovers,
	Pikes,
}
#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }
    
    fn build(values: &[u8]) -> Deck{
        let mut deck = Deck {cards: Vec::new()};
        let suites = [Suite::Hearts, Suite::Tiles, Suite::Clovers, Suite::Pikes];
        for &value in values {
            for &suite in suites.iter() {
                deck.cards.push(Card { value, suite });
            }
        }
        deck
    }

    fn take(&mut self, number: u8) -> Vec<Card> {
        let final_length = self.cards.len().saturating_sub(number.into());
        self.cards.split_off(final_length)
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
    let card = Card { value: 12, suite: Suite::Hearts };
    assert_eq!(card.value, 12);

    let values = [11, 12, 13, 10, 1];
    let mut deck = Deck::build(&values);

    assert_eq!(deck.cards.len(), 20);
    let first_card = &deck.cards[0];
    assert_eq!(first_card.value, 11);
    assert_eq!(first_card.suite, Suite::Hearts);
    deck.shuffle();

    let some_cards = deck.take(5);
    assert_eq!(some_cards.len(), 5);

    let cmp_values: Vec<u8> = some_cards.iter().map(|c| c.value).collect();
    assert!(!vec_compare(&cmp_values, &values), "no shuffle happened in between");
    
}
