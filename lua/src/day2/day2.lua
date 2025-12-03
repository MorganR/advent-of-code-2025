local M = {}

M.IdRange = {}
M.IdRange.__index = M.IdRange

function M.IdRange.new(start, finish)
    if type(start) ~= "number" or type(finish) ~= "number" then
        error("Start and end must be numbers, received start type: " .. type(start) .. ", finish type: " .. type(finish))
    end
    if finish < start then
        error("Finish must be >= to start, received start: " .. start .. ", finish: " .. finish)
    end
    return setmetatable({start = start, finish = finish}, M.IdRange)
end

function M.IdRange.parse(str)
    local start, finish = string.match(str, "(%d+)%-(%d+)")
    return M.IdRange.new(tonumber(start), tonumber(finish))
end

M.IdRange.find_p1_invalid_ids = function(self)
    local invalid_ids = {}
    local invalid_id = M.P1InvalidId.next_from_any(self.start)

    while invalid_id.value <= self.finish do
        invalid_ids[#invalid_ids + 1] = invalid_id.value
        invalid_id = invalid_id:next()
    end

    return invalid_ids
end

M.IdRange.find_p2_invalid_ids = function(self)
    local invalid_ids_set = {}
    local min_value = M.NumString.new(self.start)
    local max_value = M.NumString.new(self.finish)
    local min_value_len = math.max(2, min_value:num_digits())

    for value_len=min_value_len,max_value:num_digits() do
        local iterator = M.P2InvalidIdValueLenIterator.new(value_len, min_value, max_value)
        for _, id in ipairs(iterator:search()) do
            invalid_ids_set[id] = true
        end
    end

    local invalid_ids = {}
    for id in pairs(invalid_ids_set) do
        invalid_ids[#invalid_ids + 1] = id
    end

    return invalid_ids
end

M.IdRange.__tostring = function(self)
    return "[" .. self.start .. ", " .. self.finish .. "]"
end

function M.parse_id_ranges(str)
    local ranges = {}
    for range_str in string.gmatch(str, "%d+%-%d+") do
        ranges[#ranges + 1] = M.IdRange.parse(range_str)
    end
    return ranges
end

M.P1InvalidId = {}
M.P1InvalidId.__index = M.P1InvalidId

function M.P1InvalidId.new(value)
    local value_str = tostring(value)
    local num_digits = string.len(value_str)
    local half_str = string.sub(value, 1, num_digits / 2)
    local repeated_half = tonumber(half_str)

    if value_str ~= half_str .. half_str then
        error(tostring(value) .. " is a valid ID")
    end

    return setmetatable({
        value = value,
        repeated_half = repeated_half,
        value_str = value_str,
        num_digits = num_digits
    }, M.P1InvalidId)
end

function M.P1InvalidId.next_from_any(value)
    local full_id = value
    local value_str = tostring(value)
    local repeated_half = value
    local num_digits = string.len(value_str)
    if num_digits % 2 == 1 then
        num_digits = num_digits + 1
        if num_digits == 2 then
            repeated_half = 1
        else
            repeated_half = math.pow(10, ((num_digits / 2) - 1))
        end
        local half_str = tostring(repeated_half)
        value_str = half_str .. half_str
        full_id = tonumber(value_str)
    else
        local half_str = string.sub(value_str, 1, num_digits / 2)
        repeated_half = tonumber(half_str)
        value_str = half_str .. half_str
        full_id = tonumber(value_str)
        if full_id < value then
            repeated_half = repeated_half + 1
            half_str = tostring(repeated_half)
            value_str = half_str .. half_str
            full_id = tonumber(value_str)
        end
        num_digits = string.len(value_str)
    end
    return setmetatable({
        value = full_id,
        repeated_half = repeated_half,
        value_str = value_str,
        num_digits = num_digits
    }, M.P1InvalidId)
end

M.P1InvalidId.next = function(self)
    local repeated_half = self.repeated_half + 1
    local half_str = tostring(repeated_half)
    local value_str = half_str .. half_str
    local value = tonumber(value_str)
    local num_digits = string.len(value_str)

    return setmetatable({
        value = value,
        repeated_half = repeated_half,
        value_str = value_str,
        num_digits = num_digits
    }, M.P1InvalidId)
end

M.NumString = {}
M.NumString.__index = M.NumString

function M.NumString.new(value)
    if type(value) ~= "number" then
        error("NumString.new requires a number, received " .. value .. " with type " .. type(value))
    end
    return setmetatable({
        value=value,
        str=tostring(value)
    }, M.NumString)
end

function M.NumString.from_str(value_str)
    if type(value_str) ~= "string" then
        error("NumString.from_str requires a string, received " .. value_str .. " with type " .. type(value_str))
    end
    return setmetatable({
        value=tonumber(value_str),
        str=value_str,
    }, M.NumString)
end

function M.NumString.__lt(a, b)
    return a.value < b.value
end

function M.NumString.__le(a, b)
    return a.value <= b.value
end

function M.NumString.__eq(a, b)
    return a.value == b.value
end

function M.NumString:num_digits()
    return string.len(self.str)
end

function M.NumString:first_n_digits(n)
    return string.sub(self.str, 1, n)
end

M.P2InvalidIdRepeaterIterator = {}
M.P2InvalidIdRepeaterIterator.__index = M.P2InvalidIdRepeaterIterator

function M.P2InvalidIdRepeaterIterator.new(repeater_size, value_len, min_value, max_value)
    -- If we're using the same number of digits as the min value, this sets a lower bound on our
    -- repeater.
    local repeater = M.NumString.from_str(min_value:first_n_digits(repeater_size))
    -- Else there is no lower bound
    if value_len > min_value:num_digits() then
        repeater = M.NumString.new(math.pow(10, repeater_size - 1))    
    end
    local num_repeats = value_len / repeater_size

    local _self = setmetatable(
        {
            repeater_size=repeater_size,
            repeater=repeater,
            num_repeats=num_repeats,
            max_value=max_value,
        }, M.P2InvalidIdRepeaterIterator
    )

    -- Confirm the repeater is greater than min_value while we still have access to it.
    local value = _self:_to_value()
    if value < min_value then
        _self.repeater = M.NumString.new(_self.repeater.value + 1)
    end

    return _self
end

function M.P2InvalidIdRepeaterIterator:_to_value()
    local value_str = string.rep(self.repeater.str, self.num_repeats)
    return M.NumString.from_str(value_str)
end

function M.P2InvalidIdRepeaterIterator:iterate()
    return function()
        if self.repeater:num_digits() > self.repeater_size then
            return
        end

        local next = self:_to_value()
        if next > self.max_value then
            return
        end

        self.repeater = M.NumString.new(self.repeater.value + 1)
        return next.value
    end
end

M.P2InvalidIdValueLenIterator = {}
M.P2InvalidIdValueLenIterator.__index = M.P2InvalidIdValueLenIterator

function M.P2InvalidIdValueLenIterator.new(value_len, min_value, max_value)
    local repeater_len = math.floor(value_len / 2)
    if repeater_len == 0 then
        error("value_len must be > 1; received " .. tostring(value_len))
    end

    while value_len % repeater_len ~= 0 do
        repeater_len = repeater_len - 1
    end

    return setmetatable(
        {
            value_len=value_len,
            repeater_len=repeater_len,
            min_value=min_value,
            max_value=max_value,
        },
        M.P2InvalidIdValueLenIterator
    )
end

function M.P2InvalidIdValueLenIterator:search()
    local seen_repeater_lengths = {}
    local results = {}
    local repeater_len = self.repeater_len
    while repeater_len > 0 do
        -- print("\tChecking repeater len " .. repeater_len)
        local can_skip = false
        for _, old_len in ipairs(seen_repeater_lengths) do
            if old_len % repeater_len == 0 then
                can_skip = true
                break
            end
        end
        if self.value_len % repeater_len ~= 0 then
            can_skip = true
        end

        if not can_skip then
            -- print("\tIterating repeater len " .. repeater_len)
            local repeater = M.P2InvalidIdRepeaterIterator.new(repeater_len, self.value_len, self.min_value, self.max_value)
            for value in repeater:iterate() do
                -- print("\t\tFound value " .. value)
                results[#results + 1] = value
            end
            seen_repeater_lengths[#seen_repeater_lengths + 1] = repeater_len
        end

        repeater_len = repeater_len - 1
    end
    return results
end

return M
