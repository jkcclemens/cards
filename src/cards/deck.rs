use cards::{Card, Cards, CardSuit, CardValue, JokerColor};
use players::HasHand;

/// A deck of cards
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Deck {
  pub cards: Vec<Card>
}

impl Deck {
  /// Creates a standard 52-card deck
  pub fn standard_52() -> Deck {
    let mut cards = Vec::new();
    for suit in CardSuit::iter() {
      for value in CardValue::iter() {
        cards.push(Card::new(suit.clone(), value.clone()));
      }
    }
    Deck {
      cards: cards
    }
  }

  /// Creates a standard 54-card deck (52-card deck + jokers)
  pub fn standard_54() -> Deck {
    let mut deck = Deck::standard_52();
    deck.cards.push(Card::joker(JokerColor::Red));
    deck.cards.push(Card::joker(JokerColor::Black));
    deck
  }
}

impl Cards for Deck {
  fn shuffle(&mut self) {
    self.cards.shuffle();
  }

  fn draw(&mut self) -> Option<Card> {
    self.cards.draw()
  }

  fn draw_many(&mut self, n: usize) -> Option<Vec<Card>> {
    self.cards.draw_many(n)
  }

  fn peek(&self) -> Option<&Card> {
    self.cards.peek()
  }

  fn peek_many(&self, n: usize) -> Option<Vec<&Card>> {
    self.cards.peek_many(n)
  }

  fn deal<P: HasHand>(&mut self, players: &mut [P]) -> bool {
    self.cards.deal(players)
  }

  fn deal_many<P: HasHand>(&mut self, players: &mut [P], n: usize) -> bool {
    self.cards.deal_many(players, n)
  }
}
