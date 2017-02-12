use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;
use std::convert::TryFrom;

iterable_enum! {
  CardSuit CardSuitIter, (Debug, Clone, PartialEq, Eq, PartialOrd, Ord) {
    Hearts,
    Diamonds,
    Clubs,
    Spades
  }
}

impl<'a> TryFrom<&'a str> for CardSuit {
  type Err = String;

  fn try_from(ch: &str) -> Result<Self, Self::Err> {
    let lower = ch.to_lowercase();
    let cs = match lower.as_ref() {
      "♥" | "h" => CardSuit::Hearts,
      "♦" | "d" => CardSuit::Diamonds,
      "♣" | "c" => CardSuit::Clubs,
      "♠" | "s" => CardSuit::Spades,
      _ => return Err(String::from("Invalid card suit"))
    };
    Ok(cs)
  }
}

impl<'a, 'b> From<&'b CardSuit> for &'a str {
  fn from(cv: &'b CardSuit) -> &'a str {
    match *cv {
      CardSuit::Hearts => "♥",
      CardSuit::Diamonds => "♦",
      CardSuit::Clubs => "♣",
      CardSuit::Spades => "♠"
    }
  }
}

impl Display for CardSuit {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    let s: &str = self.into();
    write!(f, "{}", s)
  }
}
