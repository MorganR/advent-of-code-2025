# List available commands
default:
    @just --list

# Run a specific day and part (e.g., just run 1 1)
run day part input="":
    @if [ -n "{{input}}" ]; then \
        lua src/day{{day}}/part{{part}}.lua < {{input}}; \
    else \
        lua src/day{{day}}/part{{part}}.lua; \
    fi

# Test a specific day (e.g., just test 1)
test day:
    lua src/day{{day}}/test.lua

# Test all days
test-all:
    @for test in src/day*/test.lua; do \
        echo "Running $test..."; \
        lua $test || exit 1; \
    done

# Format all Lua files with stylua
format:
    stylua .

# Check if Lua files are formatted (for CI)
format-check:
    stylua --check .

# Create a new day from template (e.g., just new 2)
new day:
    @mkdir -p src/day{{day}}
    @echo 'local M = {}\n\n-- TODO: Add shared functions here\n\nreturn M' > src/day{{day}}/day{{day}}.lua
    @echo '#!/usr/bin/env lua\n\n-- Advent of Code 2025 - Day {{day}}, Part 1\n\npackage.path = package.path .. ";./src/day{{day}}/?.lua"\nlocal day{{day}} = require("day{{day}}")\n\nlocal function solve(input)\n    -- TODO: Implement solution\n    return 0\nend\n\n-- Read input from stdin or file\nlocal function read_input()\n    local lines = {}\n    for line in io.lines() do\n        table.insert(lines, line)\n    end\n    return lines\nend\n\n-- Main execution\nlocal function main()\n    local input = read_input()\n    local result = solve(input)\n    print(result)\nend\n\nmain()' > src/day{{day}}/part1.lua
    @echo '#!/usr/bin/env lua\n\n-- Advent of Code 2025 - Day {{day}}, Part 2\n\npackage.path = package.path .. ";./src/day{{day}}/?.lua"\nlocal day{{day}} = require("day{{day}}")\n\nlocal function solve(input)\n    -- TODO: Implement solution\n    return 0\nend\n\n-- Read input from stdin or file\nlocal function read_input()\n    local lines = {}\n    for line in io.lines() do\n        table.insert(lines, line)\n    end\n    return lines\nend\n\n-- Main execution\nlocal function main()\n    local input = read_input()\n    local result = solve(input)\n    print(result)\nend\n\nmain()' > src/day{{day}}/part2.lua
    @echo '#!/usr/bin/env lua\n\n-- Load LuaUnit from lib/\npackage.path = package.path .. ";./external/?.lua"\nlocal luaunit = require("luaunit")\n\n-- Load the day{{day}} module\npackage.path = package.path .. ";./src/day{{day}}/?.lua"\nlocal day{{day}} = require("day{{day}}")\n\n-- TODO: Add test cases here\n-- Example:\n-- TestExample = {}\n-- function TestExample:testSomething()\n--     luaunit.assertEquals(day{{day}}.some_function(), expected_value)\n-- end\n\n-- Run tests\nos.exit(luaunit.LuaUnit.run())' > src/day{{day}}/test.lua
    @chmod +x src/day{{day}}/part1.lua src/day{{day}}/part2.lua src/day{{day}}/test.lua
    @echo "Created day {{day}} files in src/day{{day}}/"
