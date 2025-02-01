use crate::computer::Computer;

pub(crate) fn run(input: String) {
    let mut computer = Computer::new(input.clone().into());
    computer.memory[1] = 12;
    computer.memory[2] = 2;
    computer.run();
    let val = computer.memory.get(0).unwrap();
    println!("Part 1: {}", *val);

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut computer = Computer::new(input.clone().into());
            computer.memory[1] = noun;
            computer.memory[2] = verb;
            computer.run();
            let val = computer.memory.get(0).unwrap();
            if *val == 19690720 {
                let result = 100 * noun + verb;
                println!("Part 2: {}", result);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::computer::Computer;

    #[test]
    fn test_computer() {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50".to_string();
        let mut computer = Computer::new(input.into());
        computer.run();
        let val = computer.memory.get(0);
        assert_eq!(val, Some(&3500));
    }
}
