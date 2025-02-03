use crate::computer::Computer;

pub(crate) fn run(input: String) {
    let mut computer = Computer::new(input.clone().into());
    computer.push_input(1);
    computer.run();
    let result = computer.get_output().last().unwrap().clone();
    println!("Part 1: {}", result);

    let mut computer = Computer::new(input.clone().into());
    computer.push_input(2);
    computer.run();
    let result = computer.get_output().last().unwrap().clone();
    println!("Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::computer::Computer;

    #[test]
    fn test_memory() {
        let program = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".to_string();
        let mut computer = Computer::new(program.clone().into());
        computer.run();
        assert_eq!(
            computer
                .get_output()
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(","),
            program
        );
    }

    #[test]
    fn test_large_numbers() {
        let program = "104,1125899906842624,99".to_string();
        let mut computer = Computer::new(program.into());
        computer.run();
        assert_eq!(computer.get_output()[0], 1125899906842624);
    }
}
