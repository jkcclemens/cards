# cards

cards is a library crate for playing cards and their players.

It strives to only provide basic support, the rest being implemented by other libraries and
binaries.

```rust
extern crate cards;

use cards::{Card, Cards, Deck};
use cards::players::Player;

fn main() {
  // Create a standard 52-card deck. It comes unshuffled.
  let mut deck = Deck::standard_52();
  // Shuffle the deck.
  deck.shuffle();

  // Create two players with empty hands.
  let mut players: Vec<Player> = (0..2).map(|_| Player::new()).collect();

  // Deal out 10 cards to each player. This returns false if there are not enough cards.
  if !deck.deal_many(&mut players, 10) {
    println!("Not enough cards.");
    return;
  }

  // Sort each player's hand.
  for mut player in &mut players {
    player.hand.sort();
  }

  // Print out each player's hand.
  // player 1: A♥ 6♥ A♦ 2♣ 9♣ 10♣ J♣ 3♠ 7♠ J♠
  // player 2: 9♥ J♥ K♥ 3♦ 3♣ 4♣ 5♣ 7♣ A♠ 5♠
  for (i, player) in players.iter().enumerate() {
    let cards = player.hand.iter().map(Card::to_string).collect::<Vec<_>>().join(" ");
    println!("player {}: {}", i + 1, cards);
  }
  // Print out the size of the deck after having dealt.
  // deck size: 32
  println!("deck size: {}", deck.cards.len());
}
```
