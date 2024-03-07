-- Import C timing library
package.cpath = "../../timing_lib/target/?.so;" .. package.cpath
local timing = require("timing")

local input = io.read("*all")
local start = timing.time_ns()

local result = 0

for i = 1, string.len(input) do
	local byte = string.byte(input, i)
end

local stop = timing.time_ns()
print("Result: " .. result)
print("Benchmark: " .. (stop - start) .. "ns")
