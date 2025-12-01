local M = {}

-- Parse rotation string like "L50" or "R01" into direction and amount
function M.parse_rotation(str)
    local str_len = string.len(str)
    if str_len < 2 then
        error(
            "cannot parse "
                .. str
                .. " as a rotation; it must include both a direction and a number"
        )
    end
    local direction = string.sub(str, 1, 1)
    if direction ~= "L" and direction ~= "R" then
        error("invalid direction in '" .. str .. "'; must be L or R")
    end

    local amount = tonumber(string.sub(str, 2, str_len))

    if amount == nil then
        error("invalid number in '" .. str .. "' after the direction")
    end

    return { direction = direction, amount = amount }
end

-- Parses the rotations from the given string or sequence.
function M.parse_rotations(str_or_lines)
    local rotations = {}
    if type(str_or_lines) == "string" then
        for line in str_or_lines:gmatch("[^\n%s]+") do
            table.insert(rotations, M.parse_rotation(line))
        end
    else
        for _, line in ipairs(str_or_lines) do
            table.insert(rotations, M.parse_rotation(line))
        end
    end
    return rotations
end

-- Returns the value of the dial after performing the given direction.
function M.rotate_dial(value, direction, amount)
    if direction == "R" then
        value = value + (amount % 100)
        if value > 99 then
            value = value - 100
        end
    else
        value = value - (amount % 100)
        if value < 0 then
            value = 100 + value
        end
    end
    return value
end

-- Count the zeros if applying the given rotation to a dial starting at a value of 50.
function M.count_zeros(rotations)
    local value = 50
    local num_zeros = 0
    for _, r in ipairs(rotations) do
        value = M.rotate_dial(value, r.direction, r.amount)
        if value == 0 then
            num_zeros = num_zeros + 1
        end
    end
    return num_zeros
end

return M
