mod deck;
mod parts;

pub use self::deck::*;
pub use self::parts::*;

use players::Player;

use rand::{Rng, thread_rng};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
  Normal { suit: CardSuit, value: CardValue },
  Joker(JokerColor)
}

impl Card {
  pub fn new(suit: CardSuit, value: CardValue) -> Card {
    Card::Normal {
      value: value,
      suit: suit
    }
  }

  pub fn joker(color: JokerColor) -> Card {
    Card::Joker(color)
  }

  pub fn to_string(&self) -> String {
    match *self {
      Card::Normal { ref suit, ref value } => format!("{}{}", value, suit),
      Card::Joker(ref color) => format!("Jo{}", color)
    }
  }
}

pub trait Cards {
  /// Draws a card from the deck, removing it from the deck.
  ///
  /// If the deck is empty, this will return `None`. Otherwise, it will return the card at the top
  /// of the deck.
  fn draw(&mut self) -> Option<Card>;

  /// Draws `n` cards from the top of the deck, removing them from the deck.
  ///
  /// This will not draw cards unless there are enough to be drawn. This will return None if there
  /// are not enough cards, removing no cards.
  fn draw_many(&mut self, n: usize) -> Option<Vec<Card>>;

  /// Peeks at the card on top of the deck, leaving it in place.
  ///
  /// If the deck is empty, this will return `None`. Otherwise, it will return a reference to the
  /// card at the top of the deck.
  fn peek(&self) -> Option<&Card>;

  /// Peeks at `n` cards on top of the deck, leaving them in place.
  ///
  /// If the deck does not have enough cards, this will return `None`. Otherwise, it will return a
  /// vector of references to the first `n` cards on top of the deck.
  fn peek_many(&self, n: usize) -> Option<Vec<&Card>>;

  /// Shuffles the deck in place.
  fn shuffle(&mut self);

  /// Deals one card to every player in `players`, removing them from the deck.
  ///
  /// Returns `true` if there were enough cards. Returns `false` and does not distribute any cards
  /// if there were not enough.
  fn deal(&mut self, players: &mut [Player]) -> bool;

  /// Deals `n` cards to every player in `players`, removing them from the deck.
  ///
  /// Returns `true` if there were enough cards. Returns `false` and does not distribute any cards
  /// if there were not enough.
  fn deal_many(&mut self, players: &mut [Player], n: usize) -> bool;
}

impl Cards for Vec<Card> {
  fn draw(&mut self) -> Option<Card> {
    if self.is_empty() {
      return None;
    }
    Some(self.remove(0))
  }

  fn draw_many(&mut self, n: usize) -> Option<Vec<Card>> {
    if self.len() < n {
      return None;
    }
    Some((0..n).map(|_| self.draw().unwrap()).collect())
  }

  fn peek(&self) -> Option<&Card> {
    self.get(0)
  }

  fn peek_many(&self, n: usize) -> Option<Vec<&Card>> {
    (0..n).map(|i| self.get(i)).collect()
  }

  fn shuffle(&mut self) {
    thread_rng().shuffle(self);
  }

  fn deal(&mut self, players: &mut [Player]) -> bool {
    if self.len() < players.len() {
      return false;
    }
    for player in players {
      player.hand.push(self.draw().unwrap());
    }
    true
  }

  fn deal_many(&mut self, players: &mut [Player], n: usize) -> bool {
    if self.len() < players.len() * n {
      return false;
    }
    for player in players {
      player.hand.append(&mut self.draw_many(n).unwrap());
    }
    true
  }
}
