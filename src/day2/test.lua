#!/usr/bin/env lua

-- Load LuaUnit from lib/
package.path = package.path .. ";./external/?.lua"
local luaunit = require("luaunit")

-- Load the day2 module
package.path = package.path .. ";./src/day2/?.lua"
local day2 = require("day2")

TestIdRange = {}

function TestIdRange:testNewValid()
    local result = day2.IdRange.new(2, 33)
    luaunit.assertEquals(result.start, 2)
    luaunit.assertEquals(result.finish, 33)

    local result = day2.IdRange.new(2, 2)
    luaunit.assertEquals(result.start, 2)
    luaunit.assertEquals(result.finish, 2)
end

function TestIdRange:testNewBackwards()
    luaunit.assertErrorMsgContains("Finish must be >= to start", day2.IdRange.new, 2, 1)
end

function TestIdRange:testNewNotNumbers()
    luaunit.assertErrorMsgContains("Start and end must be numbers", day2.IdRange.new, "1", 2)
    luaunit.assertErrorMsgContains("Start and end must be numbers", day2.IdRange.new, 1, "2")
end

function TestIdRange:testParseValid()
    local result = day2.IdRange.parse("123-456")
    luaunit.assertEquals(result.start, 123)
    luaunit.assertEquals(result.finish, 456)
end

function TestIdRange:testParseInvalid()
    luaunit.assertError(day2.IdRange.parse, "foo")
end

function TestIdRange:testParseRanges()
    local ranges = day2.parse_id_ranges("123-456,78-99,1-1")
    luaunit.assertItemsEquals(
        ranges,
        {
            day2.IdRange.new(123, 456),
            day2.IdRange.new(78, 99),
            day2.IdRange.new(1, 1),
        }
    )
end

function TestIdRange:test_find_p1_invalid_ids_none()
    local range = day2.IdRange.new(1, 9)
    local invalid_ids = range:find_p1_invalid_ids()

    luaunit.assertEquals(#invalid_ids, 0)
end

function TestIdRange:test_find_p1_invalid_ids_one()
    local range = day2.IdRange.new(11, 11)
    local invalid_ids = range:find_p1_invalid_ids()

    luaunit.assertItemsEquals(invalid_ids, {11})
end

function TestIdRange:test_find_p1_invalid_ids_many()
    local range = day2.IdRange.new(10, 23)
    local invalid_ids = range:find_p1_invalid_ids()

    luaunit.assertItemsEquals(invalid_ids, {11, 22})
end

TestP1InvalidId = {}

function TestP1InvalidId:test_next_from_any_before_same_size()
    local id = day2.P1InvalidId.next_from_any(31)
    luaunit.assertEquals(id.value, 33)
    luaunit.assertEquals(id.value_str, "33")
    luaunit.assertEquals(id.repeated_half, 3)
    luaunit.assertEquals(id.num_digits, 2)
end

function TestP1InvalidId:test_next_from_any_before_odd_digits()
    local id = day2.P1InvalidId.next_from_any(1)
    luaunit.assertEquals(id.value, 11)
    luaunit.assertEquals(id.value_str, "11")
    luaunit.assertEquals(id.repeated_half, 1)
    luaunit.assertEquals(id.num_digits, 2)

    local id = day2.P1InvalidId.next_from_any(919)
    luaunit.assertEquals(id.value, 1010)
    luaunit.assertEquals(id.value_str, "1010")
    luaunit.assertEquals(id.repeated_half, 10)
    luaunit.assertEquals(id.num_digits, 4)

    local id = day2.P1InvalidId.next_from_any(1698522)
    luaunit.assertEquals(id.value, 10001000)
end

function TestP1InvalidId:test_next_from_any_is_invalid()
    local id = day2.P1InvalidId.next_from_any(3434)
    luaunit.assertEquals(id.value, 3434)
    luaunit.assertEquals(id.value_str, "3434")
    luaunit.assertEquals(id.repeated_half, 34)
    luaunit.assertEquals(id.num_digits, 4)
end

function TestP1InvalidId:test_next_same_num_digits()
    local id = day2.P1InvalidId.new(33)
    local next = id:next()

    luaunit.assertItemsEquals(
        next,
        day2.P1InvalidId.new(44)
    )

    id = day2.P1InvalidId.new(5656)
    next = id:next()

    luaunit.assertItemsEquals(
        next,
        day2.P1InvalidId.new(5757)
    )
end

function TestP1InvalidId:test_next_more_digits()
    local id = day2.P1InvalidId.new(99)
    local next = id:next()

    luaunit.assertItemsEquals(
        next,
        day2.P1InvalidId.new(1010)
    )

    id = day2.P1InvalidId.new(9999)
    next = id:next()

    luaunit.assertItemsEquals(
        next,
        day2.P1InvalidId.new(100100)
    )
end

-- Run tests
os.exit(luaunit.LuaUnit.run())
