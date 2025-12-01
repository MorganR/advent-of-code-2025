# advent-of-code-2025

Advent of Code solutions for 2025, written in Lua.

## Structure

```
advent-of-code-2025/
├── lib/
│   └── luaunit.lua         # LuaUnit testing framework
├── src/
│   ├── day1/
│   │   ├── part1.lua       # Day 1, Part 1 solution
│   │   ├── part2.lua       # Day 1, Part 2 solution
│   │   └── test_part1.lua  # Tests for day 1, part 1
│   ├── day2/
│   │   └── ...
│   └── ...
└── README.md
```

- `lib/` - Shared third-party libraries (LuaUnit testing framework)
- `src/` - Solution code organized by day

## Usage

This project uses [Just](https://github.com/casey/just) as a command runner for convenience.

### Installing Just

```bash
cargo install just

# Or download from: https://github.com/casey/just/releases
```

### Quick Commands (with Just)

```bash
# List all available commands
just

# Run a specific day and part
just run 1 1                    # Run day 1, part 1
just run 1 2                    # Run day 1, part 2

# Run with input file
just run 1 1 input.txt

# Test a specific day and part
just test 1 1

# Test all days
just test-all

# Create a new day from template
just new 2                      # Creates src/day2/ with part1.lua, and part2.lua
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
