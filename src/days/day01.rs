pub(crate) fn run(input: String) {
    let part1: u32 = input
        .trim()
        .lines()
        .map(|s| s.parse::<u32>().unwrap())
        .map(fuel_for_module)
        .sum();
    println!("Part 1: {}", part1);

    let part2: u32 = input
        .trim()
        .lines()
        .map(|s| s.parse::<u32>().unwrap())
        .map(fuel_for_module_alt)
        .sum();
    println!("Part 2: {}", part2);
}

fn fuel_for_module(mass: u32) -> u32 {
    let fuel = mass / 3;
    if fuel >= 2 {
        fuel - 2
    } else {
        0
    }
}

fn fuel_for_module_alt(mass: u32) -> u32 {
    let fuel = fuel_for_module(mass);
    let fuel_for_fuel = fuel_for_module(fuel);
    if fuel_for_fuel <= 0 {
        fuel
    } else {
        fuel + fuel_for_fuel + fuel_for_module_alt(fuel_for_fuel)
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day01::{fuel_for_module, fuel_for_module_alt};

    #[test]
    fn test_fuel_for_module() {
        assert_eq!(fuel_for_module(12), 2);
        assert_eq!(fuel_for_module(14), 2);
        assert_eq!(fuel_for_module(1969), 654);
        assert_eq!(fuel_for_module(100756), 33583);
    }

    #[test]
    fn test_fuel_for_module_alt() {
        assert_eq!(fuel_for_module_alt(12), 2);
        assert_eq!(fuel_for_module_alt(1969), 966);
        assert_eq!(fuel_for_module_alt(100756), 50346);
    }
}
