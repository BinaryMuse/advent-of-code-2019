pub mod instruction;
pub mod memory;

use std::collections::VecDeque;

use instruction::{Instruction, Opcode};
use memory::Memory;

#[derive(Clone, Debug)]
pub(crate) struct Computer {
    pub memory: Memory,
    pointer: usize,
    input: VecDeque<isize>,
    output: VecDeque<isize>,
    yeild_on_output: bool,
    yielded: bool,
    halted: bool,
}

impl Computer {
    pub fn new(memory: Memory) -> Self {
        Self {
            memory,
            pointer: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            yeild_on_output: false,
            yielded: false,
            halted: false,
        }
    }

    pub fn set_yield_on_output(&mut self, val: bool) {
        self.yeild_on_output = val;
    }

    pub fn set_input(&mut self, input: Vec<isize>) {
        self.input = input.into();
    }

    pub fn push_input(&mut self, input: isize) {
        self.input.push_back(input);
    }

    pub fn get_output(&self) -> Vec<isize> {
        self.output.iter().map(|n| *n).collect()
    }

    pub fn next_output(&mut self) -> Option<isize> {
        self.output.pop_front()
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    pub fn run(&mut self) {
        self.yielded = false;
        loop {
            if self.yielded || self.halted {
                return;
            }

            let mem_val = *self.memory.get(self.pointer).unwrap() as usize;
            let opcode: Opcode = mem_val.into();
            let mem_slice = &self.memory[self.pointer..];
            let Some(instruction) = Instruction::from_opcode(opcode, mem_slice.iter()) else {
                panic!("Invalid opcode")
            };

            self.execute_instr(instruction);
        }
    }

    fn execute_instr(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Add(p1, p2, p3) => {
                let op1 = p1.value(&self.memory);
                let op2 = p2.value(&self.memory);
                let result = op1 + op2;
                self.memory[p3.as_pos()] = result;
                self.pointer += 4;
            }
            Instruction::Mult(p1, p2, p3) => {
                let op1 = p1.value(&self.memory);
                let op2 = p2.value(&self.memory);
                let result = op1 * op2;
                self.memory[p3.as_pos()] = result;
                self.pointer += 4;
            }
            Instruction::Input(p1) => {
                if let Some(value) = self.input.pop_front() {
                    self.memory[p1.as_pos()] = value;
                    self.pointer += 2;
                } else {
                    self.yielded = true;
                    return ();
                }
            }
            Instruction::Output(p1) => {
                let value = p1.value(&self.memory);
                self.output.push_back(value);
                self.pointer += 2;

                if self.yeild_on_output {
                    self.yielded = true;
                    return ();
                }
            }
            Instruction::JumpIfTrue(p1, p2) => {
                let value = p1.value(&self.memory);
                if value != 0 {
                    self.pointer = p2.value(&self.memory) as usize;
                } else {
                    self.pointer += 3
                }
            }
            Instruction::JumpIfFalse(p1, p2) => {
                let value = p1.value(&self.memory);
                if value == 0 {
                    self.pointer = p2.value(&self.memory) as usize;
                } else {
                    self.pointer += 3
                }
            }
            Instruction::LessThan(p1, p2, p3) => {
                let val1 = p1.value(&self.memory);
                let val2 = p2.value(&self.memory);
                let out = if val1 < val2 { 1 } else { 0 };
                self.memory[p3.as_pos()] = out;
                self.pointer += 4;
            }
            Instruction::Equals(p1, p2, p3) => {
                let val1 = p1.value(&self.memory);
                let val2 = p2.value(&self.memory);
                let out = if val1 == val2 { 1 } else { 0 };
                self.memory[p3.as_pos()] = out;
                self.pointer += 4;
            }
            Instruction::Stop => {
                self.halted = true;
                return ();
            }
        }
    }
}
