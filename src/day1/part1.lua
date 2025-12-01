#!/usr/bin/env lua

-- Advent of Code 2025 - Day 1, Part 1

package.path = package.path .. ";./src/day1/?.lua"
local day1 = require("day1")

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

main()
