#!/usr/bin/env lua

-- Advent of Code 2025 - Day 1, Part 2

package.path = package.path .. ";./src/day1/?.lua"
local day1 = require("day1")

local function solve(lines)
    local rotations = day1.parse_rotations(lines)
    local count = day1.count_zero_clicks(rotations)
    return count
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

    if input == nil then
        error("A valid input must be provided")
    end

    local result = solve(input)
    print(result)
end

main()
