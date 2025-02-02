use crate::computer::Computer;

pub(crate) fn run(input: String) {
    let mut computer = Computer::new(input.clone().into());
    computer.set_input(vec![1]);
    computer.run();
    let output = computer.get_output();
    let diag_code = output.last();
    println!("Part 1: {}", diag_code.unwrap());

    let mut computer = Computer::new(input.clone().into());
    computer.set_input(vec![5]);
    computer.run();
    let output = computer.get_output();
    let diag_code = output.last();
    println!("Part 1: {}", diag_code.unwrap());
}
