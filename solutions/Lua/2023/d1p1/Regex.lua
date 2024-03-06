local uv = require "uv"
local input = io.read ("*all")
local start = uv.hrtime ( )

local result = 0
local lines = { }

for line in input:gmatch ("[^\r\n]+") do
    table.insert (lines, line)
end

function getFirstAndLastNumber (str)
    local first = string.match (str, "%d")
    local last = string.match (string.reverse (str), "%d")
    return tonumber(first), tonumber(last)
end

for i = 1, #lines do
    local first, last = getFirstAndLastNumber (lines [i])
    result = result + first * 10 + last
end

local stop = uv.hrtime ( )
print ("Result: "..result)
print ("Benchmark: "..(stop - start)..'ns')
