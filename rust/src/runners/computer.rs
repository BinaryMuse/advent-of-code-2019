use std::str::FromStr;
use std::num::ParseIntError;
use std::convert::{TryFrom, TryInto};
use regex::Regex;
use custom_error::custom_error;
use voca_rs::*;

custom_error!{pub InstructionError
  BadInput{input: String} = "Couldn't parse {input} into an instruction",
  BadOpcode{input: String} = "{input} does not contain a valid opcode",
  BadParam{input: String} = "{input} is not a valid parameter mode"
}

#[derive(Copy, Clone, Debug)]
pub enum Opcode {
  Add,        // 01
  Multiply,   // 02
  Input,      // 03
  Output,     // 04

  Exit        // 99
}

impl FromStr for Opcode {
  type Err = InstructionError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "01" => Ok(Opcode::Add),
      "02" => Ok(Opcode::Multiply),
      "03" => Ok(Opcode::Input),
      "04" => Ok(Opcode::Output),
      "99" => Ok(Opcode::Exit),
      _    => Err(InstructionError::BadOpcode{input: s.to_string()})
    }
  }
}

impl Opcode {
  pub fn param_spec(self) -> (usize, bool) {
    match self {
      Opcode::Add       => (3, true),
      Opcode::Multiply  => (3, true),
      Opcode::Input     => (1, false),
      Opcode::Output    => (1, false),
      Opcode::Exit      => (0, false),
    }
  }
}

#[derive(Copy, Clone, Debug)]
pub enum Param {
  Position, Immediate
}

impl Param {
  pub fn from_char(c: char) -> Result<Self, InstructionError> {
    match c {
      '0' => Ok(Param::Position),
      '1' => Ok(Param::Immediate),
      _   => Err(InstructionError::BadParam{input: c.to_string()})
    }
  }
}

#[derive(Clone, Debug)]
pub struct Instruction {
  opcode: Opcode,
  parameters: Vec<Param>
}

impl TryFrom<i32> for Instruction {
  type Error = InstructionError;

  fn try_from(value: i32) -> Result<Self, Self::Error> {
    value.to_string().parse()
  }
}

impl FromStr for Instruction {
  type Err = InstructionError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let re = Regex::new(r"^(\d*)(\d{2})$").unwrap();
    let padded = &manipulate::pad_left(s, 2, "0");
    let captures = re.captures(padded).unwrap();

    match (captures.get(1), captures.get(2)) {
      (Some(params_str), Some(opcode_str)) => {
        let opcode = opcode_str.as_str().parse::<Opcode>()?;
        let (param_count, force_last) = opcode.param_spec();
        let mut chars = params_str.as_str().chars().rev();
        let mut parameters = Vec::with_capacity(param_count);
        for _ in 0..param_count {
          parameters.push(Param::from_char(chars.next().unwrap_or('0'))?);
        }
        if force_last {
          parameters.pop();
          parameters.push(Param::Immediate);
        }

        Ok(Self { opcode, parameters })
      },
      _ => Err(InstructionError::BadInput{input: s.to_string()})
    }
  }
}

#[derive(Debug)]
pub struct Computer {
  position: usize,
  code: Vec<i32>
}

impl FromStr for Computer {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let code = s.split(',').map(|s| s.parse::<i32>()).collect::<Result<_, _>>()?;
    Ok(Self { position: 0, code })
  }
}

impl Computer {
  pub fn get_at(&self, param: Param, position: usize) -> i32 {
    match param {
      Param::Position  => self.code[self.code[position] as usize],
      Param::Immediate => self.code[position],
    }
  }

  pub fn set_at(&mut self, position: usize, value: i32) {
    self.code[position] = value;
  }

  fn params3(&self, params: &[Param], opcode_pos: usize) -> (i32, i32, i32) {
    assert!(params.len() == 3);

    let items = params.iter().enumerate().map(|(idx, p)| {
      match *p {
        Param::Immediate => self.code[opcode_pos + idx + 1],
        Param::Position  => {
          let pos = self.code[opcode_pos + idx + 1];
          self.code[pos as usize]
        }
      }
    }).collect::<Vec<_>>();

    (items[0], items[1], items[2])
  }

  // fn get_params3(params: &Vec<Param>) -> (i32, i32, i32) {

  // }

  pub fn run(&mut self) -> Result<(), InstructionError> {
    let mut running = true;
    while running {
      let instr: Instruction = self.get_at(Param::Immediate, self.position).try_into()?;
      match instr.opcode {
        Opcode::Add => {
          let (input_one, input_two, output) = self.params3(&instr.parameters, self.position);

          let num = input_one + input_two;
          self.set_at(output as usize, num);
        },
        Opcode::Multiply => {
          let (input_one, input_two, output) = self.params3(&instr.parameters, self.position);

          let num = input_one * input_two;
          self.set_at(output as usize, num);
        },
        Opcode::Exit => {
          running = false;
        },
        _ => panic!("unexpected opcode!")
      };
      self.position += instr.opcode.param_spec().0 + 1;
    };

    Ok(())
  }

  #[cfg(test)]
  fn as_str(&self) -> String {
    self.code.iter().map(|&n| n.to_string()).collect::<Vec<_>>().join(",")
  }
}

#[test]
fn test_computer() {
  let mut p1 = Computer::from_str("1,0,0,0,99").unwrap();
  p1.run().expect("p1 to run successfully");
  assert_eq!(p1.as_str(), "2,0,0,0,99");

  let mut p2 = Computer::from_str("2,3,0,3,99").unwrap();
  p2.run().expect("p2 to run successfully");
  assert_eq!(p2.as_str(), "2,3,0,6,99");

  let mut p3 = Computer::from_str("1,1,1,4,99,5,6,0,99").unwrap();
  p3.run().expect("p3 to run successfully");
  assert_eq!(p3.as_str(), "30,1,1,4,2,5,6,0,99");

  let mut p4 = Computer::from_str("2,4,4,5,99,0").unwrap();
  p4.run().expect("p4 to run successfully");
  assert_eq!(p4.as_str(), "2,4,4,5,99,9801");
}
