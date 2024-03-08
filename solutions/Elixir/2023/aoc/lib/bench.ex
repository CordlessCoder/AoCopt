defmodule Aoc.Bench do
  def bench do
    Benchee.run(
      %{
        "d1_no_regex" => fn -> Aoc.DayOne.part_one_no_regex(true) end,
        "d1_flow" => fn -> Aoc.DayOne.part_one_flow(true) end,
        "d1_compr" => fn -> Aoc.DayOne.part_one_compr(true) end,
        "d1_compr2" => fn -> Aoc.DayOne.part_one_compr2(true) end
      },
      time: 4,
      memory_time: 2,
      reduction_time: 2,
      measure_function_call_overhead: true
    )
  end
end
