# Advent of Code 2025 - Rust Solutions

A Rust project for solving Advent of Code 2025 puzzles with a modular structure.

## Project Structure

```
src/
├── main.rs          # CLI entry point: argument parsing, stdin reading, dispatch
├── days/            # Day solutions
│   └── mod.rs       # Module declarations for each day
└── utils/           # Shared utilities
    ├── mod.rs       # Module declarations
    └── input.rs     # Input parsing helpers
```

## Building

```bash
cargo build --release
```

The binary will be at `target/release/rust`.

## Usage

Run a specific day and part with input from stdin:

```bash
./target/release/rust -d <DAY> -p <PART> < input.txt
```

Examples:
```bash
# Day 1, Part 1
./target/release/rust -d 1 -p 1 < data/day01/input.txt

# Day 2, Part 2
./target/release/rust -d 2 -p 2 < data/day02/input.txt

# Using cargo run
cargo run -- -d 1 -p 1 < data/day01/input.txt
```

Help:
```bash
./target/release/rust --help
```

## Adding a New Day

To add a solution for day N:

1. **Create the day module** (`src/days/dayNN.rs`):
   ```rust
   use std::error::Error;

   pub fn solve_part1(input: &str) -> Result<String, Box<dyn Error>> {
       // Parse and solve part 1
       Ok("answer".to_string())
   }

   pub fn solve_part2(input: &str) -> Result<String, Box<dyn Error>> {
       // Parse and solve part 2
       Ok("answer".to_string())
   }
   ```

2. **Register the module** in `src/days/mod.rs`:
   ```rust
   pub mod dayNN;
   ```

3. **Add dispatch entries** in `src/main.rs`:
   ```rust
   match (args.day, args.part) {
       // ... existing entries ...
       (N, 1) => {
           let result = days::dayNN::solve_part1(&input)?;
           println!("{}", result);
       },
       (N, 2) => {
           let result = days::dayNN::solve_part2(&input)?;
           println!("{}", result);
       },
       // ... rest of match ...
   }
   ```

## Testing

Add tests within each day module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = "sample input";
        assert_eq!(solve_part1(input).unwrap(), "42");
    }
}
```

Run tests with:
```bash
cargo test
cargo test day01  # Run tests for a specific day
```

## Shared Utilities

Common parsing helpers can be added to `src/utils/input.rs` for reuse across days.
