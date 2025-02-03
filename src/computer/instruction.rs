use super::Computer;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Instruction {
    Add(Param, Param, Param),
    Mult(Param, Param, Param),
    Input(Param),
    Output(Param),
    JumpIfTrue(Param, Param),
    JumpIfFalse(Param, Param),
    LessThan(Param, Param, Param),
    Equals(Param, Param, Param),
    RelativeBase(Param),
    Stop,
}

impl Instruction {
    pub fn from_opcode(
        opcode: Opcode,
        mut mem: std::slice::Iter<'_, isize>,
    ) -> Option<Instruction> {
        // consume the opcode
        mem.next().unwrap();
        if opcode.code == 1 {
            let (p1, p2, p3) = Self::get_params3(&opcode.param_modes, mem);
            Some(Self::Add(p1, p2, p3))
        } else if opcode.code == 2 {
            let (p1, p2, p3) = Self::get_params3(&opcode.param_modes, mem);
            Some(Self::Mult(p1, p2, p3))
        } else if opcode.code == 3 {
            let p1 = Self::get_params1(&opcode.param_modes, mem);
            Some(Self::Input(p1))
        } else if opcode.code == 4 {
            let p1 = Self::get_params1(&opcode.param_modes, mem);
            Some(Self::Output(p1))
        } else if opcode.code == 5 {
            let (p1, p2) = Self::get_params2(&opcode.param_modes, mem);
            Some(Self::JumpIfTrue(p1, p2))
        } else if opcode.code == 6 {
            let (p1, p2) = Self::get_params2(&opcode.param_modes, mem);
            Some(Self::JumpIfFalse(p1, p2))
        } else if opcode.code == 7 {
            let (p1, p2, p3) = Self::get_params3(&opcode.param_modes, mem);
            Some(Self::LessThan(p1, p2, p3))
        } else if opcode.code == 8 {
            let (p1, p2, p3) = Self::get_params3(&opcode.param_modes, mem);
            Some(Self::Equals(p1, p2, p3))
        } else if opcode.code == 9 {
            let p1 = Self::get_params1(&opcode.param_modes, mem);
            Some(Self::RelativeBase(p1))
        } else if opcode.code == 99 {
            Some(Self::Stop)
        } else {
            None
        }
    }

    fn get_params1(modes: &[usize], mut mem: std::slice::Iter<'_, isize>) -> Param {
        let mut modes = modes.iter();
        Self::get_param(modes.next(), mem.next())
    }

    fn get_params2(modes: &[usize], mut mem: std::slice::Iter<'_, isize>) -> (Param, Param) {
        let mut modes = modes.iter();
        let p1 = Self::get_param(modes.next(), mem.next());
        let p2 = Self::get_param(modes.next(), mem.next());
        (p1, p2)
    }

    fn get_params3(modes: &[usize], mut mem: std::slice::Iter<'_, isize>) -> (Param, Param, Param) {
        let mut modes = modes.iter();
        let p1 = Self::get_param(modes.next(), mem.next());
        let p2 = Self::get_param(modes.next(), mem.next());
        let p3 = Self::get_param(modes.next(), mem.next());
        (p1, p2, p3)
    }

    fn get_param(mode: Option<&usize>, mem: Option<&isize>) -> Param {
        let mode = *mode.unwrap_or(&0);
        if mode == 0 {
            Param::Pos(*mem.unwrap() as usize)
        } else if mode == 1 {
            Param::Imm(*mem.unwrap())
        } else {
            Param::Rel(*mem.unwrap())
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Opcode {
    code: usize,
    param_modes: Vec<usize>,
}

impl From<usize> for Opcode {
    fn from(value: usize) -> Self {
        let mut copy = value;
        let mut digits: Vec<usize> = Vec::with_capacity(10);

        while copy > 0 {
            let n = copy % 10;
            copy /= 10;
            digits.push(n);
        }

        let mut digits = digits.into_iter();
        let Some(ones_place) = digits.next() else {
            panic!("No ones place in op code")
        };
        let tens_place = digits.next().unwrap_or(0);
        let code = tens_place * 10 + ones_place;

        Self {
            code,
            param_modes: digits.collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Param {
    Pos(usize),
    Imm(isize),
    Rel(isize),
}

impl Param {
    pub fn value(&self, computer: &Computer) -> isize {
        match self {
            Self::Pos(idx) => *computer.memory.get(*idx).unwrap_or(&0),
            Self::Imm(num) => *num,
            Self::Rel(idx) => {
                let idx = computer.relative_base + idx;
                *computer.memory.get(idx as usize).unwrap_or(&0)
            }
        }
    }

    pub fn as_pos(&self, computer: &Computer) -> usize {
        match self {
            Self::Pos(idx) => *idx,
            Self::Imm(num) => *num as usize,
            Self::Rel(idx) => (computer.relative_base + idx) as usize,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::computer::instruction::Param;

    use super::{Instruction, Opcode};

    #[test]
    fn test_opcode_generation() {
        let input = 1;
        let opcode: Opcode = input.into();
        assert_eq!(opcode.code, 1);
        assert_eq!(opcode.param_modes, vec![]);

        let input = 99;
        let opcode: Opcode = input.into();
        assert_eq!(opcode.code, 99);
        assert_eq!(opcode.param_modes, vec![]);

        let input = 101;
        let opcode: Opcode = input.into();
        assert_eq!(opcode.code, 1);
        assert_eq!(opcode.param_modes, vec![1]);

        let input = 1050;
        let opcode: Opcode = input.into();
        assert_eq!(opcode.code, 50);
        assert_eq!(opcode.param_modes, vec![0, 1]);

        let input = 11050;
        let opcode: Opcode = input.into();
        assert_eq!(opcode.code, 50);
        assert_eq!(opcode.param_modes, vec![0, 1, 1]);
    }

    #[test]
    fn test_opcode_to_instruction() {
        let input = 11002;
        let opcode: Opcode = input.into();
        let mem: Vec<isize> = vec![0, 1, 2, 3, 4, 5];
        let inst = Instruction::from_opcode(opcode, mem.iter()).unwrap();
        assert_eq!(
            inst,
            Instruction::Mult(Param::Pos(1), Param::Imm(2), Param::Imm(3))
        );
    }
}
