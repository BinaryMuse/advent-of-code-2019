use super::computer::Computer;

pub fn run(input: String, _args: &[String]) {
  let mut computer = input.parse::<Computer>().unwrap();
  computer.input(1);
  computer.run().expect("Expected computer to run");
  let successful = computer.get_output().iter().rev().skip(1).all(|output| *output == 0);
  println!("Diagnostic success: {}; output: {}", successful, computer.get_output().back().unwrap());
}
