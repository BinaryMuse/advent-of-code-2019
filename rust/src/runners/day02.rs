use std::str::FromStr;
use std::num::ParseIntError;
use std::convert::TryInto;

pub fn run(input: String, _args: &[String]) {
  let mut program = Program::from_str(&input).unwrap();
  program.replace_at(1, 12);
  program.replace_at(2, 2);
  program.run();

  println!("Part 1: Position 0 is {}", program.get_at(0));

  for noun in 1..=99 {
    for verb in 1..=99 {
      let mut program = Program::from_str(&input).unwrap();
      program.replace_at(1, noun);
      program.replace_at(2, verb);
      program.run();
      if program.get_at(0) == 19_690_720 {
        println!("Part 2: 100 * noun + verb = {}", 100 * noun + verb);
        return;
      }
    }
  }
}

struct Program {
  position: usize,
  code: Vec<i32>
}

impl FromStr for Program {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let code: Result<_, _> = s.split(',').map(|s| s.parse::<i32>()).collect();
    code.map(|c| Self { position: 0, code: c })
  }
}

impl Program {
  fn get_at(&self, position: usize) -> i32 {
    self.code[position]
  }

  fn get_pos_at(&self, position: usize) -> usize {
    self.get_at(position).try_into().unwrap()
  }

  fn replace_at(&mut self, position: usize, value: i32) {
    self.code[position] = value;
  }

  fn run(&mut self) {
    let mut running = true;
    while running {
      let code = self.get_at(self.position);
      match code {
        1 => {
          let input_one = self.get_pos_at(self.position + 1);
          let input_two = self.get_pos_at(self.position + 2);
          let output = self.get_pos_at(self.position + 3);

          let num = self.get_at(input_one) + self.get_at(input_two);
          self.replace_at(output, num);
        },
        2 => {
          let input_one = self.get_pos_at(self.position + 1);
          let input_two = self.get_pos_at(self.position + 2);
          let output = self.get_pos_at(self.position + 3);

          let num = self.get_at(input_one) * self.get_at(input_two);
          self.replace_at(output, num);
        },
        99 => {
          running = false;
        },
        _ => panic!("unexpected opcode!")
      };
      self.position += 4;
    }
  }

  fn as_str(&self) -> String {
    self.code.iter().map(|&n| n.to_string()).collect::<Vec<_>>().join(",")
  }
}

#[test]
fn test_program() {
  let mut p1 = Program::from_str("1,0,0,0,99").unwrap();
  p1.run();
  assert_eq!(p1.as_str(), "2,0,0,0,99");

  let mut p2 = Program::from_str("2,3,0,3,99").unwrap();
  p2.run();
  assert_eq!(p2.as_str(), "2,3,0,6,99");

  let mut p4 = Program::from_str("1,1,1,4,99,5,6,0,99").unwrap();
  p4.run();
  assert_eq!(p4.as_str(), "30,1,1,4,2,5,6,0,99");

  let mut p3 = Program::from_str("2,4,4,5,99,0").unwrap();
  p3.run();
  assert_eq!(p3.as_str(), "2,4,4,5,99,9801");
}
