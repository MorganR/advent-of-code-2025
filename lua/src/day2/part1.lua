#!/usr/bin/env lua

-- Advent of Code 2025 - Day 2, Part 1

package.path = package.path .. ";./src/day2/?.lua"
local day2 = require("day2")

local function solve(input)
    local ranges = {}
    for _, line in ipairs(input) do
        local more_ranges = day2.parse_id_ranges(line)
        for _, r in ipairs(more_ranges) do
            table.insert(ranges, r)
        end
    end
    print("Parsed " .. #ranges .. " ranges")

    local id_sum = 0
    local count_ids = 0
    for _, range in ipairs(ranges) do
        local invalid_ids = range:find_p1_invalid_ids()
        -- print("\tIn range " .. tostring(range) .. ", found IDs " .. table.concat(invalid_ids, ", "))
        for _, id in ipairs(invalid_ids) do
            id_sum = id_sum + id
        end
        count_ids = count_ids + #invalid_ids
    end

    print("Found " .. count_ids .. " invalid IDs with sum " .. id_sum)

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
