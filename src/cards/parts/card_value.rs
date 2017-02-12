use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;
use std::convert::TryFrom;

iterable_enum! {
  CardValue CardValueIter, (Debug, Clone, PartialEq, Eq, PartialOrd, Ord) {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King
  }
}

impl<'a> TryFrom<&'a str> for CardValue {
  type Err = String;

  fn try_from(ch: &str) -> Result<Self, Self::Err> {
    let lower = ch.to_lowercase();
    let cv = match lower.as_ref() {
      "a" => CardValue::Ace,
      "2" => CardValue::Two,
      "3" => CardValue::Three,
      "4" => CardValue::Four,
      "5" => CardValue::Five,
      "6" => CardValue::Six,
      "7" => CardValue::Seven,
      "8" => CardValue::Eight,
      "9" => CardValue::Nine,
      "10" => CardValue::Ten,
      "j" => CardValue::Jack,
      "q" => CardValue::Queen,
      "k" => CardValue::King,
      _ => return Err(String::from("Invalid card value"))
    };
    Ok(cv)
  }
}

impl<'a, 'b> From<&'b CardValue> for &'a str {
  fn from(cv: &'b CardValue) -> &'a str {
    match *cv {
      CardValue::Ace => "A",
      CardValue::Two => "2",
      CardValue::Three => "3",
      CardValue::Four => "4",
      CardValue::Five => "5",
      CardValue::Six => "6",
      CardValue::Seven => "7",
      CardValue::Eight => "8",
      CardValue::Nine => "9",
      CardValue::Ten => "10",
      CardValue::Jack => "J",
      CardValue::Queen => "Q",
      CardValue::King => "K"
    }
  }
}

impl Display for CardValue {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    let s: &str = self.into();
    write!(f, "{}", s)
  }
}
