use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;
use std::convert::TryFrom;

iterable_enum! {
  JokerColor JokerColorIter, (Debug, PartialEq, Eq, PartialOrd, Ord) {
    Red,
    Black
  }
}

impl<'a> TryFrom<&'a str> for JokerColor {
  type Err = String;

  fn try_from(ch: &str) -> Result<Self, Self::Err> {
    let lower = ch.to_lowercase();
    let jc = match lower.as_ref() {
      "r" => JokerColor::Red,
      "b" => JokerColor::Black,
      _ => return Err(String::from("Invalid joker color"))
    };
    Ok(jc)
  }
}

impl<'a, 'b> From<&'b JokerColor> for &'a str {
  fn from(cv: &'b JokerColor) -> &'a str {
    match *cv {
      JokerColor::Red => "R",
      JokerColor::Black => "B"
    }
  }
}

impl Display for JokerColor {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    let s: &str = self.into();
    write!(f, "{}", s)
  }
}
