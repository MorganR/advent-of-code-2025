# advent-of-code-2025

Advent of Code solutions for 2025, written in Lua.

## Structure

```
advent-of-code-2025/
├── external/
│   └── luaunit.lua         # LuaUnit testing framework
├── src/
│   ├── day1/
│   │   ├── day1.lua        # Day 1 library (shared functions)
│   │   ├── part1.lua       # Day 1, Part 1 solution
│   │   ├── part2.lua       # Day 1, Part 2 solution
│   │   └── test.lua        # Tests for day 1
│   ├── day2/
│   │   └── ...
│   └── ...
└── README.md
```

- `external/` - Third-party libraries (LuaUnit testing framework)
- `src/` - Solution code organized by day
  - Each day has a `dayX.lua` library file for shared functions
  - `part1.lua` and `part2.lua` import the library and implement their solutions
  - `test.lua` contains tests for the library functions

## Setup

This project uses [Just](https://github.com/casey/just) as a command runner and [StyLua](https://github.com/JohnnyMorganz/StyLua) for code formatting.

### Set up

```bash
mkdir external
curl -o external/luaunit.lua https://github.com/bluebird75/luaunit/blob/master/luaunit.lua

cargo install stylua
cargo install just
```

## Usage

### Quick Commands (with Just)

```bash
# List all available commands
just

# Run a specific day and part
just run 1 1                    # Run day 1, part 1
just run 1 2                    # Run day 1, part 2

# Run with input file
just run 1 1 input.txt

# Test a specific day
just test 1

# Test all days
just test-all

# Format all Lua files
just format

# Check if files are formatted (for CI)
just format-check

# Create a new day from template
just new 2                      # Creates src/day2/ with day2.lua, part1.lua, part2.lua, and test.lua
```

### Manual Usage (without Just)

If you prefer not to use Just, you can run commands directly:

#### Running Solutions

Run a solution with input from a file:

```bash
lua src/day1/part1.lua < input.txt
```

Or with input piped directly:

```bash
echo "input data" | lua src/day1/part1.lua
```

Make scripts executable (optional):

```bash
chmod +x src/day1/part1.lua
./src/day1/part1.lua < input.txt
```

#### Running Tests

Tests use the [LuaUnit](https://github.com/bluebird75/luaunit) testing framework.

Run tests for a specific day from the project root:

```bash
lua src/day1/test.lua
```

Tests will show detailed output and return exit code 0 on success, non-zero on failure.
