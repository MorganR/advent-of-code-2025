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

function TestIdRange:test_find_p2_invalid_ids_none()
    local range = day2.IdRange.new(1, 9)
    local invalid_ids = range:find_p2_invalid_ids()

    luaunit.assertEquals(#invalid_ids, 0)
end

function TestIdRange:test_find_p2_invalid_ids_one()
    local range = day2.IdRange.new(10, 20)
    local invalid_ids = range:find_p2_invalid_ids()

    luaunit.assertItemsEquals(invalid_ids, {11})
end

function TestIdRange:test_find_p2_invalid_ids_two()
    local range = day2.IdRange.new(998, 1012)
    local invalid_ids = range:find_p2_invalid_ids()

    luaunit.assertItemsEquals(invalid_ids, {999, 1010})
end

function TestIdRange:test_find_p2_invalid_ids_multi_repeater_length()
    local range = day2.IdRange.new(1010010100, 1010110101)
    local invalid_ids = range:find_p2_invalid_ids()

    luaunit.assertItemsEquals(
        invalid_ids,
        {1010010100, 1010110101, 1010101010}
    )
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

    local id = day2.P1InvalidId.next_from_any(99199)
    luaunit.assertEquals(id.value, 100100)
    luaunit.assertEquals(id.value_str, "100100")
    luaunit.assertEquals(id.repeated_half, 100)
    luaunit.assertEquals(id.num_digits, 6)
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

TestNumString = {}

function TestNumString:test_new()
    local value = day2.NumString.new(100)
    luaunit.assertEquals(value.value, 100)
    luaunit.assertEquals(value.str, "100")
end

function TestNumString:test_from_str()
    local value = day2.NumString.from_str("100")
    luaunit.assertEquals(value.value, 100)
    luaunit.assertEquals(value.str, "100")
end

function TestNumString:test_num_digits()
    local value = day2.NumString.new(100)
    luaunit.assertEquals(value:num_digits(), 3)
end

function TestNumString:test_first_n_digits()
    local value = day2.NumString.new(12345)
    luaunit.assertEquals(value:first_n_digits(2), "12")
end

TestP2InvalidIdRepeaterIterator = {}

function TestP2InvalidIdRepeaterIterator:test_iterate_full_range()
    local it = day2.P2InvalidIdRepeaterIterator.new(2, 4, day2.NumString.new(9500), day2.NumString.new(10000))
    local values = {}
    for value in it:iterate() do
        values[#values + 1] = value
    end
    luaunit.assertItemsEquals(
        values,
        {9595, 9696, 9797, 9898, 9999}
    )
end

function TestP2InvalidIdRepeaterIterator:test_iterate_exact()
    local it = day2.P2InvalidIdRepeaterIterator.new(2, 4, day2.NumString.new(9595), day2.NumString.new(9595))
    local values = {}
    for value in it:iterate() do
        values[#values + 1] = value
    end
    luaunit.assertItemsEquals(
        values,
        {9595}
    )
end

function TestP2InvalidIdRepeaterIterator:test_iterate_none()
    local it = day2.P2InvalidIdRepeaterIterator.new(2, 4, day2.NumString.new(9500), day2.NumString.new(9590))
    local values = {}
    for value in it:iterate() do
        values[#values + 1] = value
    end
    luaunit.assertItemsEquals(
        values,
        {}
    )
end

TestP2InvalidIdValueLenIterator = {}

function TestP2InvalidIdValueLenIterator:test_iterate_one_valid_len()
    local it = day2.P2InvalidIdValueLenIterator.new(4, day2.NumString.new(9500), day2.NumString.new(9999))
    local values = it:search()
    luaunit.assertItemsEquals(values, {
        9595, 9696, 9797, 9898, 9999
    })
end

function TestP2InvalidIdValueLenIterator:test_iterate_multi_valid_len()
    local it = day2.P2InvalidIdValueLenIterator.new(10, day2.NumString.new(1010010100), day2.NumString.new(1010110101))
    local invalid_ids = it:search()
    luaunit.assertItemsEquals(
        invalid_ids,
        {1010010100, 1010110101, 1010101010}
    )
end

-- Run tests
os.exit(luaunit.LuaUnit.run())
