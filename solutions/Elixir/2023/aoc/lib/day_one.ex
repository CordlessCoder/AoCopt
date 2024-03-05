defmodule Aoc.DayOne do
  import Unicode.Guards

  def part_one do
    dev = false

    input = if dev do
      File.read!("inputs.txt")
    else
      IO.read(:stdio, :eof)
    end

    start = System.os_time(:nanosecond)

    # Code here
    sum = String.split(input, "\n", trim: true)
    |> Enum.map(fn line ->
      digits = String.replace(line, ~r/[^0-9]/, "")
      first = String.to_integer(String.first(digits))
      last = String.to_integer(String.last(digits))

      first * 10 + last
    end)
    |> Enum.sum

    endpoint = System.os_time(:nanosecond)
    duration = endpoint - start

    IO.puts sum
    IO.puts "Took #{duration} nanoseconds"
  end
end
