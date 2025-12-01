#!/usr/bin/env lua

-- Load LuaUnit from lib/
package.path = package.path .. ";./lib/?.lua"
local luaunit = require("luaunit")

-- Load the part1 module
package.path = package.path .. ";./src/day1/?.lua"
local part1 = require("part1")

-- Tests for parse_rotations
TestParseRotations = {}

function TestParseRotations:testL50()
    local direction, amount = part1.parse_rotations("L50")
    luaunit.assertEquals(direction, "l")
    luaunit.assertEquals(amount, 50)
end

function TestParseRotations:testR01()
    local direction, amount = part1.parse_rotations("R01")
    luaunit.assertEquals(direction, "r")
    luaunit.assertEquals(amount, 1)
end

function TestParseRotations:testR99()
    local direction, amount = part1.parse_rotations("R99")
    luaunit.assertEquals(direction, "r")
    luaunit.assertEquals(amount, 99)
end

function TestParseRotations:testL00()
    local direction, amount = part1.parse_rotations("L00")
    luaunit.assertEquals(direction, "l")
    luaunit.assertEquals(amount, 0)
end

-- Run tests
os.exit(luaunit.LuaUnit.run())
