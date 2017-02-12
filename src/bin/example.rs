extern crate cards;

use cards::{Card, Cards, Deck};
use cards::players::Player;

fn main() {
  let mut deck = Deck::standard_54();
  deck.shuffle();

  let mut players: Vec<Player> = (0..2).map(|_| Player::new()).collect();

  if !deck.deal_many(&mut players, 10) {
    println!("Not enough cards.");
    return;
  }

  for mut player in &mut players {
    player.hand.sort();
  }

  for (i, player) in players.iter().enumerate() {
    let cards = player.hand.iter().map(Card::to_string).collect::<Vec<_>>().join(" ");
    println!("player {}: {}", i + 1, cards);
  }
  println!("deck size: {}", deck.cards.len());
}
