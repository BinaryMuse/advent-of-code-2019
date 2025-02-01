mod instruction;
mod memory;

use instruction::Instruction;
use memory::Memory;

#[derive(Clone, Debug)]
pub(crate) struct Computer {
    pub memory: Memory,
    pointer: usize,
}

impl Computer {
    pub fn new(memory: Memory) -> Self {
        Self { memory, pointer: 0 }
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.memory.get(self.pointer).unwrap();
            let mem_slice = &self.memory[self.pointer..];
            let Some(instruction) = Instruction::from_opcode(*opcode, mem_slice.iter()) else {
                panic!("Invalid opcode")
            };

            if instruction == Instruction::Stop {
                return ();
            }

            self.execute_instr(instruction);
        }
    }

    fn execute_instr(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Add(loc1, loc2, loc3) => {
                let op1 = self.memory.get(loc1).unwrap();
                let op2 = self.memory.get(loc2).unwrap();
                let result = *op1 + *op2;
                self.memory[loc3] = result;
                self.pointer += 4;
            }
            Instruction::Mult(loc1, loc2, loc3) => {
                let op1 = self.memory.get(loc1).unwrap();
                let op2 = self.memory.get(loc2).unwrap();
                let result = *op1 * *op2;
                self.memory[loc3] = result;
                self.pointer += 4;
            }
            Instruction::Stop => return (),
        }
    }
}
