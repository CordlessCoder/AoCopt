defmodule Aoc.Bench do
  def bench do
    Benchee.run(
      %{
        d1_regex: fn -> Aoc.DayOne.part_one_regex(true) end,
        d1_no_regex: fn -> Aoc.DayOne.part_one_no_regex(true) end,
        d1_no_regex_parallel: fn -> Aoc.DayOne.part_one_no_regex_parallel(true) end
      },
      time: 4,
      memory_time: 2,
      reduction_time: 2,
      measure_function_call_overhead: true
    )
  end
end
