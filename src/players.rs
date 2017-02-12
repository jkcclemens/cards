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

impl HasHand for Player {
  fn hand(&self) -> &[Card] {
    &self.hand
  }

  fn hand_mut(&mut self) -> &mut Vec<Card> {
    &mut self.hand
  }
}

/// Trait for structs containing hands of cards.
///
/// This is most useful for creating a custom `Player`, since all methods use this trait instead of
/// `Player` when operating on player-types.
pub trait HasHand: Sized {
  /// Gets the hand of this object, suitable for examination.
  fn hand(&self) -> &[Card];

  /// Gets the hand of this object, suitable for mutation.
  fn hand_mut(&mut self) -> &mut Vec<Card>;
}
