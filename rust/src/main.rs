use clap::Parser;
use std::error::Error;
use std::io::{self, Read};

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
    env_logger::init();

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
            let joltage = days::day3::find_best_total_joltage(&input, 2);
            println!("Best total joltage: {}", joltage);
            Ok(())
        }
        (3, 2) => {
            let joltage = days::day3::find_best_total_joltage(&input, 12);
            println!("Best total joltage: {}", joltage);
            Ok(())
        }
        (4, 1) => {
            let count_accessible_rolls = days::day4::count_accessible_rolls_of_paper(&input)?;
            println!(
                "Count accessible rolls of paper: {}",
                count_accessible_rolls
            );
            Ok(())
        }
        (4, 2) => {
            let count_removable_rolls = days::day4::count_total_removable_rolls_of_paper(&input)?;
            println!("Count removable rolls of paper: {}", count_removable_rolls);
            Ok(())
        }
        (5, 1) => {
            let count_fresh = days::day5::count_fresh_ingredients(&input)?;
            println!("{} ingredients are fresh", count_fresh);
            Ok(())
        }
        (5, 2) => {
            let count_fresh = days::day5::count_all_fresh_ids(&input)?;
            println!("There are {} fresh IDs", count_fresh);
            Ok(())
        }
        (6, 1) => {
            let sum = days::day6::solve_and_sum_math_sheet(&input)?;
            println!("Sum: {}", sum);
            Ok(())
        }
        _ => Err(format!("Day {} part {} not implemented", args.day, args.part).into()),
    }
}
