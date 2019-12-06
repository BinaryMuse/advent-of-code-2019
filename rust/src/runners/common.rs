use std::fs::File;
use std::io::{Read, Result};
use std::ops::Add;

pub fn get_input(basename: &str) -> Result<String> {
  let filepath = format!("./inputs/{}.txt", basename);
  let mut f = File::open(filepath)?;
  let mut contents = String::new();
  f.read_to_string(&mut contents)?;

  Ok(contents)
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub struct Coord(pub i32, pub i32);

impl Coord {
  pub fn distance(self, other: Self) -> i32 {
    (self.0 - other.0).abs() + (self.1 - other.1).abs()
  }

  pub fn origin() -> Self {
    Coord(0, 0)
  }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Coord(self.0 + other.0, self.1 + other.1)
    }
}

impl From<(i32, i32)> for Coord {
    fn from(tuple: (i32, i32)) -> Self {
        Coord(tuple.0, tuple.1)
    }
}
