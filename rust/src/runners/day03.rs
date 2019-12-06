use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::str::FromStr;
use custom_error::custom_error;
use super::common::Coord;

pub fn run(input: String, _args: &[String]) {
  let wires: Vec<_> = input.lines().map(Wire::from_str).collect::<Result<_, _>>().unwrap();
  let first_wire = wires.first().unwrap();
  let iter = first_wire.visited.iter();

  let intersections: Vec<_> = iter.filter(|coord| {
    wires.iter().all(|wire| wire.visited.contains(coord))
  }).filter(|coord| **coord != Coord::origin()).collect();
  let distances: Vec<_> = intersections.iter().map(|coord| (coord, coord.distance(Coord(0, 0)))).collect();
  let shortest = distances.iter().min_by_key(|(_, distance)| distance).unwrap();
  println!("Part 1: Closest intersection to origin is {:?}, distance is {}", shortest.0, shortest.1);

  let least_steps = intersections.iter().map(|coord| {
    wires.iter().map(move |wire| *(wire.history.get(coord).unwrap())).sum::<usize>()
  }).min_by_key(|distance| *distance).unwrap();
  println!("Part 2: Smallest combined steps: {}", least_steps);
}

custom_error!{WireParseError
  BadFormat{input: String, source: ParseIntError} = "Couldn't parse {input} to a direction; doesn't contain a valid number",
  UnknownDirection{input: String} = "Couldn't parse {} into a direction; unknown direction code"
}

#[derive(Clone, Copy, Debug)]
enum Direction {
  R(usize),
  D(usize),
  U(usize),
  L(usize)
}

impl FromStr for Direction {
  type Err = WireParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let letter = s.chars().take(1).collect::<String>();
    let digits = s.chars().skip(1).collect::<String>().parse::<usize>()
      .map_err(|e| WireParseError::BadFormat{input: s.to_string(), source: e})?;

    match letter.as_str() {
      "R" => Ok(Direction::R(digits)),
      "L" => Ok(Direction::L(digits)),
      "U" => Ok(Direction::U(digits)),
      "D" => Ok(Direction::D(digits)),
      _   => Err(WireParseError::UnknownDirection{ input: s.to_string() })
    }
  }
}

struct Wire {
  visited: HashSet<Coord>,
  history: HashMap<Coord, usize>
}

impl FromStr for Wire {
  type Err = WireParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut visited = HashSet::new();
    let mut history = HashMap::new();
    let mut steps = 0;

    let mut current = Coord(0, 0);
    visited.insert(current);
    history.insert(current, steps);

    for piece in s.split(',') {
      let dir: Direction = piece.parse()?;
      let (delta, count) = match dir {
        Direction::R(n) => (Coord( 1,  0), n),
        Direction::L(n) => (Coord(-1,  0), n),
        Direction::U(n) => (Coord( 0,  1), n),
        Direction::D(n) => (Coord( 0, -1), n),
      };

      for _ in 1..=count {
        steps += 1;
        current = current + delta;
        visited.insert(current);
        history.entry(current).or_insert(steps);
      }
    }

    Ok(Wire { visited, history })
  }
}
