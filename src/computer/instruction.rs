#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Instruction {
    Add(usize, usize, usize),
    Mult(usize, usize, usize),
    Stop,
}

impl Instruction {
    pub fn from_opcode(code: usize, mut mem: std::slice::Iter<'_, usize>) -> Option<Instruction> {
        // consume the opcode
        mem.next().unwrap();
        if code == 1 {
            let loc1 = mem.next().unwrap();
            let loc2 = mem.next().unwrap();
            let loc3 = mem.next().unwrap();
            Some(Self::Add(*loc1, *loc2, *loc3))
        } else if code == 2 {
            let loc1 = mem.next().unwrap();
            let loc2 = mem.next().unwrap();
            let loc3 = mem.next().unwrap();
            Some(Self::Mult(*loc1, *loc2, *loc3))
        } else if code == 99 {
            Some(Self::Stop)
        } else {
            None
        }
    }
}
