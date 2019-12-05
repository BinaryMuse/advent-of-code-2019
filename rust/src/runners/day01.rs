pub fn run(input: String, _args: &[String]) {
  let masses: Vec<i32> = input.lines().map(|line| line.parse::<i32>().expect("could not parse line into number")).collect();

  let total_fuel = masses.iter().fold(0, |acc, mass| acc + fuel_for_mass(*mass));
  println!("Total fuel for all modules: {}", total_fuel);

  let total_fuel_including_fuel = masses.iter().fold(0, |acc, mass| acc + fuel_for_mass_with_fuel(*mass));
  println!("Total fuel for all modules including fuel: {}", total_fuel_including_fuel);
}

fn fuel_for_mass(mass: i32) -> i32 {
  let fuel = mass / 3 - 2;
  std::cmp::max(0, fuel)
}

fn fuel_for_mass_with_fuel(mass: i32) -> i32 {
  let mut fuel = fuel_for_mass(mass);
  let mut fuel_added = fuel;
  while fuel_added > 0 {
    fuel_added = fuel_for_mass(fuel_added);
    fuel += fuel_added;
  }

  fuel
}

#[test]
fn test_fuel_for_mass() {
  assert_eq!(fuel_for_mass(8), 0);
  assert_eq!(fuel_for_mass(12), 2);
  assert_eq!(fuel_for_mass(14), 2);
  assert_eq!(fuel_for_mass(1969), 654);
  assert_eq!(fuel_for_mass(100_756), 33583);
}

#[test]
fn test_fuel_for_mass_with_fuel() {
  assert_eq!(fuel_for_mass_with_fuel(14), 2);
  assert_eq!(fuel_for_mass_with_fuel(1969), 966);
  assert_eq!(fuel_for_mass_with_fuel(100_756), 50346)
}
