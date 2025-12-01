#!/usr/bin/env lua

-- Load LuaUnit from lib/
package.path = package.path .. ";./external/?.lua"
local luaunit = require("luaunit")

-- Load the part1 module
package.path = package.path .. ";./src/day1/?.lua"
local day1 = require("day1")

TestParseRotation = {}

function TestParseRotation:testL50()
    local result = day1.parse_rotation("L50")
    luaunit.assertEquals(result.direction, "L")
    luaunit.assertEquals(result.amount, 50)
end

function TestParseRotation:testR01()
    local result = day1.parse_rotation("R01")
    luaunit.assertEquals(result.direction, "R")
    luaunit.assertEquals(result.amount, 1)
end

function TestParseRotation:testL00()
    local result = day1.parse_rotation("L00")
    luaunit.assertEquals(result.direction, "L")
    luaunit.assertEquals(result.amount, 0)
end

function TestParseRotation:testLongRotation()
    local result = day1.parse_rotation("L3401")
    luaunit.assertEquals(result.amount, 3401)
end

function TestParseRotation:testTooShort()
    luaunit.assertErrorMsgContains("cannot parse", day1.parse_rotation, "L")
end

function TestParseRotation:testInvalidDirection()
    luaunit.assertErrorMsgContains("invalid direction", day1.parse_rotation, "A1")
end

function TestParseRotation:testInvalidNumber()
    luaunit.assertErrorMsgContains("invalid number", day1.parse_rotation, "LX")
end

TestParseRotations = {}

function TestParseRotations:testParseRotations()
    luaunit.assertItemsEquals(
        day1.parse_rotations("L68\nL30\nR48"),
        { { direction = "L", amount = 68 }, { direction = "L", amount = 30 }, {
            direction = "R",
            amount = 48,
        } }
    )
end

function TestParseRotations:testParseRotationsFromLines()
    luaunit.assertItemsEquals(
        day1.parse_rotations({"L68", "L30", "R48"}),
        { { direction = "L", amount = 68 }, { direction = "L", amount = 30 }, {
            direction = "R",
            amount = 48,
        } }
    )
end

TestRotateDial = {}

function TestRotateDial:testRotateSimple()
    luaunit.assertEquals(day1.rotate_dial(50, "R", 1), 51)
    luaunit.assertEquals(day1.rotate_dial(50, "L", 1), 49)
end

function TestRotateDial:testOverrotateOnce()
    luaunit.assertEquals(day1.rotate_dial(50, "R", 50), 0)
    luaunit.assertEquals(day1.rotate_dial(50, "R", 51), 1)
    luaunit.assertEquals(day1.rotate_dial(50, "L", 51), 99)
    luaunit.assertEquals(day1.rotate_dial(50, "L", 52), 98)
end

function TestRotateDial:testOverrotateMany()
    luaunit.assertEquals(day1.rotate_dial(0, "R", 1234), 34)
    luaunit.assertEquals(day1.rotate_dial(0, "L", 1233), 67)
end

TestCountZeros = {}

function TestCountZeros:testSmallSample()
    day1.count_zeros(
        day1.parse_rotations([[
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
        ]]),
        3
    )
end

-- Run tests
os.exit(luaunit.LuaUnit.run())
