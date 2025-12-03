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

M.IdRange.__tostring = function(self)
    return "[" .. self.start .. ", " .. self.finish .. "]"
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

function M.parse_id_ranges(str)
    local ranges = {}
    for range_str in string.gmatch(str, "%d+%-%d+") do
        ranges[#ranges + 1] = M.IdRange.parse(range_str)
    end
    return ranges
end

return M
