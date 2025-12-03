#!/usr/bin/env lua

-- Load LuaUnit from lib/
package.path = package.path .. ";./external/?.lua"
local luaunit = require("luaunit")

-- Load the day2 module
package.path = package.path .. ";./src/day2/?.lua"
local day2 = require("day2")

-- TODO: Add test cases here
-- Example:
-- TestExample = {}
-- function TestExample:testSomething()
--     luaunit.assertEquals(day2.some_function(), expected_value)
-- end

-- Run tests
os.exit(luaunit.LuaUnit.run())
