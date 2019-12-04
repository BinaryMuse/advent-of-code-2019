mod common;
mod day01;

type Runner = Box<dyn Fn(&[String])>;

pub fn run_day(day: u32, args: &[String]) {
    let runner = get_runner(day).unwrap_or_else(|| panic!("No runner found for day {}", day));
    runner(args);
}

fn get_runner(day: u32) -> Option<Runner> {
    match day {
        1 => Some(Box::new(day01::run)),
        _ => None
    }
}
