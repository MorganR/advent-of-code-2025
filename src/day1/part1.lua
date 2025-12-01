#!/usr/bin/env lua

-- Advent of Code 2025 - Day 1, Part 1

local M = {}

-- Parse rotation string like "L50" or "R01" into direction and amount
function M.parse_rotations(str)
    -- TODO: Implement parsing
    return nil, nil
end

local function solve(input)
    -- TODO: Implement solution
    return 0
end

-- Read input from stdin or file
local function read_input()
    local lines = {}
    for line in io.lines() do
        table.insert(lines, line)
    end
    return lines
end

-- Main execution
local function main()
    local input = read_input()
    local result = solve(input)
    print(result)
end

-- Only run main if this file is executed directly (not required as module)
if not pcall(debug.getlocal, 4, 1) then
    main()
end

return M
