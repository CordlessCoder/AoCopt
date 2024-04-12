-- Import C timing library
package.cpath = "../../timing_lib/target/?.so;" .. package.cpath
local timing = require("timing")

local input = io.read("*all")
local start = timing.time_ns()

local result = 0

local function getFirstAndLastNumber(str)
	local first = string.match(str, "%d")
	local last = string.match(string.reverse(str), "%d")
	return tonumber(first), tonumber(last)
end

for line in input:gmatch("[^\r\n]+") do
	local first, last = getFirstAndLastNumber(line)
	result = result + first * 10 + last
end

local stop = timing.time_ns()
print("Result: " .. result)
print("Runtime: " .. (stop - start) .. "ns")
