extern crate cards;

use cards::{Cards, Card, Deck, CardSuit};
use cards::players::HasHand;

#[derive(Debug, Default)]
struct CustomPlayer {
  hand: Vec<Card>,
  score: usize
}

impl CustomPlayer {
  fn score(&self) -> usize {
    self.score
  }

  fn set_score(&mut self, score: usize) {
    self.score = score;
  }
}

impl HasHand for CustomPlayer {
  fn hand(&self) -> &[Card] {
    &self.hand
  }

  fn hand_mut(&mut self) -> &mut Vec<Card> {
    &mut self.hand
  }
}

fn main() {
  let mut deck = Deck::standard_52();
  deck.shuffle();

  let mut players: Vec<CustomPlayer> = (0..2).map(|_| CustomPlayer::default()).collect();

  deck.deal_many(&mut players, 10);

  for mut player in &mut players {
    let score = player.hand().iter()
      .map(|c| if let Card::Normal { suit: CardSuit::Spades, .. } = *c { 1 } else { 0 })
      .sum();
    player.set_score(score);
  }

  for (i, player) in players.iter().enumerate() {
    println!("player {}'s score: {}", i + 1, player.score());
  }
}
