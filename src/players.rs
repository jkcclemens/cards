//! Card players

use cards::Card;

/// A card player.
///
/// Has a hand of cards.
#[derive(Debug, Default)]
pub struct Player {
  /// The player's hand
  pub hand: Vec<Card>
}

impl Player {
  /// Creates a new player with an empty hand
  pub fn new() -> Self {
    Player::default()
  }
}
