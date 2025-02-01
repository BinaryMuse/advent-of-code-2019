use clap::Parser;
use std::{error::Error, fs, path::Path};

mod days;
mod util;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let day = util::normalize_day(&args.day);
    let input_path = format!("inputs/{}.txt", day);
    let input = fs::read_to_string(Path::new(&input_path))?;

    match day.as_str() {
        "01" => days::day01::run(input),
        _ => panic!("Day not implemented"),
    };

    Ok(())
}
