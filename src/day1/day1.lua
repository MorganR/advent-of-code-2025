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

    return direction, amount
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

return M
