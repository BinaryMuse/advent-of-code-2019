use super::computer::Computer;

pub fn run(input: String, _args: &[String]) {
  {
    let mut computer = input.parse::<Computer>().unwrap();
    computer.input(1);
    computer.run().expect("Expected computer to run");
    let successful = computer.get_output().iter().rev().skip(1).all(|output| *output == 0);
    println!("Part 1: Diagnostic success: {}; output: {}", successful, computer.get_output().back().unwrap());
  }
  {
    let mut computer = input.parse::<Computer>().unwrap();
    computer.input(5);
    computer.run().expect("Expected computer to run");
    println!("Part 2: Diagnostic code: {}", computer.get_output().front().unwrap());
  }
}
