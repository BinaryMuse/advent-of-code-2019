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
    relative_base: isize,
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
            relative_base: 0,
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

    pub fn set_memory(&mut self, address: usize, value: isize) {
        if self.memory.len() <= address {
            self.memory.resize(address + 1, 0);
        }

        self.memory[address] = value;
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
                let op1 = p1.value(&self);
                let op2 = p2.value(&self);
                let result = op1 + op2;
                self.set_memory(p3.as_pos(&self), result);
                self.pointer += 4;
            }
            Instruction::Mult(p1, p2, p3) => {
                let op1 = p1.value(&self);
                let op2 = p2.value(&self);
                let result = op1 * op2;
                self.set_memory(p3.as_pos(&self), result);
                self.pointer += 4;
            }
            Instruction::Input(p1) => {
                if let Some(value) = self.input.pop_front() {
                    self.set_memory(p1.as_pos(&self), value);
                    self.pointer += 2;
                } else {
                    self.yielded = true;
                    return ();
                }
            }
            Instruction::Output(p1) => {
                let value = p1.value(&self);
                self.output.push_back(value);
                self.pointer += 2;

                if self.yeild_on_output {
                    self.yielded = true;
                    return ();
                }
            }
            Instruction::JumpIfTrue(p1, p2) => {
                let value = p1.value(&self);
                if value != 0 {
                    self.pointer = p2.value(&self) as usize;
                } else {
                    self.pointer += 3
                }
            }
            Instruction::JumpIfFalse(p1, p2) => {
                let value = p1.value(&self);
                if value == 0 {
                    self.pointer = p2.value(&self) as usize;
                } else {
                    self.pointer += 3
                }
            }
            Instruction::LessThan(p1, p2, p3) => {
                let val1 = p1.value(&self);
                let val2 = p2.value(&self);
                let out = if val1 < val2 { 1 } else { 0 };
                self.set_memory(p3.as_pos(&self), out);
                self.pointer += 4;
            }
            Instruction::Equals(p1, p2, p3) => {
                let val1 = p1.value(&self);
                let val2 = p2.value(&self);
                let out = if val1 == val2 { 1 } else { 0 };
                self.set_memory(p3.as_pos(&self), out);
                self.pointer += 4;
            }
            Instruction::RelativeBase(p1) => {
                let val = p1.value(&self);
                self.relative_base += val;
                self.pointer += 2;
            }
            Instruction::Stop => {
                self.halted = true;
                return ();
            }
        }
    }
}
