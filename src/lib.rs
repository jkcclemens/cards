//! cards is a library crate that implements playing cards, decks, and basic players.
//!
//! This crate also implements basic functions like dealing, drawing, and peeking.

#![feature(try_from)]

extern crate rand;

macro_rules! iterable_enum {
  ($(#[$struct_meta:meta])* enum $name:ident $iter_name:ident { $($variant:ident),* }) => (
    $(#[$struct_meta])*
    pub enum $name { $($variant),* }

    impl $name {
      pub fn iter() -> $iter_name {
        $iter_name(None)
      }
    }

    pub struct $iter_name(Option<$name>);

    impl Iterator for $iter_name {
      type Item = $name;

      fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
          None => $( { self.0 = Some($name::$variant); Some($name::$variant) },
          Some($name::$variant) => )* None,
        }
      }
    }
  );
}

pub mod players;
mod cards;

pub use cards::*;
