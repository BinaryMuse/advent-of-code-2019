use std::str::FromStr;
use std::num::ParseIntError;
use std::convert::{TryFrom, TryInto};
use std::collections::LinkedList;
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
  Add,          // 01
  Multiply,     // 02
  Input,        // 03
  Output,       // 04
  JumpIfTrue,   // 05
  JumpIfFalse,  // 06
  LessThan,     // 07
  Equals,       // 08

  Exit          // 99
}

impl FromStr for Opcode {
  type Err = InstructionError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "01" => Ok(Opcode::Add),
      "02" => Ok(Opcode::Multiply),
      "03" => Ok(Opcode::Input),
      "04" => Ok(Opcode::Output),
      "05" => Ok(Opcode::JumpIfTrue),
      "06" => Ok(Opcode::JumpIfFalse),
      "07" => Ok(Opcode::LessThan),
      "08" => Ok(Opcode::Equals),
      "99" => Ok(Opcode::Exit),
      _    => Err(InstructionError::BadOpcode{input: s.to_string()})
    }
  }
}

impl Opcode {
  pub fn param_spec(self) -> (usize, bool) {
    match self {
      Opcode::Add         => (3, true),
      Opcode::Multiply    => (3, true),
      Opcode::Input       => (1, true),
      Opcode::Output      => (1, false),
      Opcode::JumpIfTrue  => (2, false),
      Opcode::JumpIfFalse => (2, false),
      Opcode::LessThan    => (3, true),
      Opcode::Equals      => (3, true),
      Opcode::Exit        => (0, false),
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
    let re = Regex::new(r"^(-?\d*)(-?\d{2})$").unwrap();
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


#[derive(Clone, Debug)]
pub struct Computer {
  position: usize,
  code: Vec<i32>,
  input_queue: LinkedList<i32>,
  output: LinkedList<i32>
}

impl FromStr for Computer {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let code = s.split(',').map(|s| s.parse::<i32>()).collect::<Result<_, _>>()?;
    Ok(Self { position: 0, code, input_queue: LinkedList::new(), output: LinkedList::new() })
  }
}

impl Computer {
  pub fn input(&mut self, input: i32) {
    self.input_queue.push_back(input);
  }

  pub fn get_output(&self) -> &LinkedList<i32> {
    &self.output
  }

  pub fn get_at(&self, param: Param, position: usize) -> i32 {
    match param {
      Param::Position  => self.code[self.code[position] as usize],
      Param::Immediate => self.code[position],
    }
  }

  pub fn set_at(&mut self, position: usize, value: i32) {
    self.code[position] = value;
  }

  fn params_vec(&self, params: &[Param], opcode_pos: usize) -> Vec<i32> {
    params.iter().enumerate().map(|(idx, p)| {
      match *p {
        Param::Immediate => self.code[opcode_pos + idx + 1],
        Param::Position  => {
          let pos = self.code[opcode_pos + idx + 1];
          self.code[pos as usize]
        }
      }
    }).collect::<Vec<_>>()
  }

  fn params1(&self, params: &[Param], opcode_pos: usize) -> i32 {
    assert!(params.len() == 1);
    let items = self.params_vec(params, opcode_pos);
    items[0]
  }

  fn params2(&self, params: &[Param], opcode_pos: usize) -> (i32, i32) {
    assert!(params.len() == 2);
    let items = self.params_vec(params, opcode_pos);
    (items[0], items[1])
  }

  fn params3(&self, params: &[Param], opcode_pos: usize) -> (i32, i32, i32) {
    assert!(params.len() == 3);
    let items = self.params_vec(params, opcode_pos);
    (items[0], items[1], items[2])
  }

  pub fn run(&mut self) -> Result<(), InstructionError> {
    let mut running = true;
    while running {
      let mut auto_pos = true;
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
        Opcode::Input => {
          let input = self.input_queue.pop_front().expect("Asked for an input but nothing in the queue!");
          let pos = self.params1(&instr.parameters, self.position);
          self.set_at(pos as usize, input);
        },
        Opcode::Output => {
          let output = self.params1(&instr.parameters, self.position);
          self.output.push_back(output);
        },
        Opcode::JumpIfTrue => {
          let (check, new_pointer) = self.params2(&instr.parameters, self.position);
          if check != 0 {
            self.position = new_pointer as usize;
            auto_pos = false;
          }
        },
        Opcode::JumpIfFalse => {
          let (check, new_pointer) = self.params2(&instr.parameters, self.position);
          if check == 0 {
            self.position = new_pointer as usize;
            auto_pos = false;
          }
        },
        Opcode::LessThan => {
          let (first, second, pos) = self.params3(&instr.parameters, self.position);
          let value = if first < second { 1 } else { 0 };
          self.set_at(pos as usize, value);
        },
        Opcode::Equals => {
          let (first, second, pos) = self.params3(&instr.parameters, self.position);
          let value = if first == second { 1 } else { 0 };
          self.set_at(pos as usize, value);
        },
        Opcode::Exit => {
          running = false;
        }
      };
      if auto_pos { self.position += instr.opcode.param_spec().0 + 1 }
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

#[test]
fn test_parameter_modes() {
  let mut c1 = Computer::from_str("1002,4,3,4,33").unwrap();
  c1.run().expect("p4 to run successfully");
  assert_eq!(c1.as_str(), "1002,4,3,4,99");
}

#[test]
fn test_input_output() {
  let mut c1 = Computer::from_str("3,0,4,0,99").unwrap();
  c1.input(42);
  c1.run().expect("p4 to run successfully");
  assert_eq!(c1.as_str(), "42,0,4,0,99");
  assert_eq!(c1.output.pop_front(), Some(42));
  assert_eq!(c1.output.pop_front(), None);
}

#[test]
fn test_jumps_compares() {
  let mut c1 = Computer::from_str(
    &vec![
      "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,".to_string(),
      "1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,".to_string(),
      "999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99".to_string()
    ].join("")
  ).unwrap();
  let mut c2 = c1.clone();
  let mut c3 = c1.clone();

  c1.input(7);
  c2.input(8);
  c3.input(9);

  c1.run().expect("Expected computer one to run");
  c2.run().expect("Expected computer two to run");
  c3.run().expect("Expected computer three to run");

  assert_eq!(c1.get_output().front(), Some(&999));
  assert_eq!(c2.get_output().front(), Some(&1000));
  assert_eq!(c3.get_output().front(), Some(&1001));
}
