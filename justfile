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

# Test a specific day and part (e.g., just test 1 1)
test day part:
    lua src/day{{day}}/test_part{{part}}.lua

# Test all days
test-all:
    @for test in src/day*/test.lua; do \
        echo "Running $test..."; \
        lua $test || exit 1; \
    done

# Create a new day from template (e.g., just new 2)
new day:
    @mkdir -p src/day{{day}}
    @echo '#!/usr/bin/env lua\n\n-- Advent of Code 2025 - Day {{day}}, Part 1\n\nlocal M = {}\n\nlocal function solve(input)\n    -- TODO: Implement solution\n    return 0\nend\n\nlocal function read_input()\n    local lines = {}\n    for line in io.lines() do\n        table.insert(lines, line)\n    end\n    return lines\nend\n\nlocal function main()\n    local input = read_input()\n    local result = solve(input)\n    print(result)\nend\n\nif not pcall(debug.getlocal, 4, 1) then\n    main()\nend\n\nreturn M' > src/day{{day}}/part1.lua
    @echo '#!/usr/bin/env lua\n\n-- Advent of Code 2025 - Day {{day}}, Part 2\n\nlocal M = {}\n\nlocal function solve(input)\n    -- TODO: Implement solution\n    return 0\nend\n\nlocal function read_input()\n    local lines = {}\n    for line in io.lines() do\n        table.insert(lines, line)\n    end\n    return lines\nend\n\nlocal function main()\n    local input = read_input()\n    local result = solve(input)\n    print(result)\nend\n\nif not pcall(debug.getlocal, 4, 1) then\n    main()\nend\n\nreturn M' > src/day{{day}}/part2.lua
    @chmod +x src/day{{day}}/part1.lua src/day{{day}}/part2.lua
    @echo "Created day {{day}} files in src/day{{day}}/"
