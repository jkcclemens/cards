use cards::Card;

#[derive(Debug, Default)]
pub struct Player {
  pub hand: Vec<Card>
}

impl Player {
  pub fn new() -> Self {
    Player::default()
  }
}
