use std::error::Error;
use std::io::{self, Read};
use clap::Parser;

mod days;
mod utils;

#[derive(Parser)]
#[command(name = "aoc")]
#[command(about = "Advent of Code 2025 solutions in Rust", long_about = None)]
struct Args {
    /// Day number (1-12)
    #[arg(short, long)]
    day: u8,

    /// Part number (1 or 2)
    #[arg(short, long)]
    part: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Validate day and part
    if args.day < 1 || args.day > 12 {
        return Err(format!("Day must be between 1 and 12, got {}", args.day).into());
    }
    if args.part < 1 || args.part > 2 {
        return Err(format!("Part must be 1 or 2, got {}", args.part).into());
    }

    // Read all input from stdin
    println!("Reading input...");
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Dispatch to correct solver
    match (args.day, args.part) {
        (3, 1) => {
            let joltage = days::day3::find_best_total_joltage(&input);
            println!("Best total joltage: {}", joltage);
            Ok(())
        }
        _ => return Err(format!("Day {} part {} not implemented", args.day, args.part).into()),
    }
}
