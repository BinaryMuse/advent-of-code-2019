use super::computer::{Computer, Param};
use std::str::FromStr;

pub fn run(input: String, _args: &[String]) {
  let mut computer = Computer::from_str(&input).unwrap();
  computer.set_at(1, 12);
  computer.set_at(2, 2);
  computer.run().expect("expected computer to finish running");

  println!("Part 1: Position 0 is {}", computer.get_at(Param::Position, 0));

  for noun in 1..=99 {
    for verb in 1..=99 {
      let mut computer = Computer::from_str(&input).unwrap();
      computer.set_at(1, noun);
      computer.set_at(2, verb);
      computer.run().expect("expected computer to finish running");
      if computer.get_at(Param::Immediate, 0) == 19_690_720 {
        println!("Part 2: 100 * noun + verb = {}", 100 * noun + verb);
        return;
      }
    }
  }
}
