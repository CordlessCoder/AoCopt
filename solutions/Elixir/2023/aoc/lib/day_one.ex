defmodule Aoc.DayOne do
  def part_one_regex(dev \\ false) do
    input =
      if dev do
        File.read!("inputs.txt")
      else
        IO.read(:stdio, :eof)
      end

    start = System.os_time(:nanosecond)

    sum =
      String.split(input, "\n", trim: true)
      |> Stream.map(fn line ->
        digits = String.replace(line, ~r/[^0-9]/, "")
        first = String.to_integer(String.first(digits))
        last = String.to_integer(String.last(digits))

        first * 10 + last
      end)
      |> Enum.sum()

    endpoint = System.os_time(:nanosecond)
    duration = endpoint - start

    IO.puts(sum)
    IO.puts("Took #{duration} nanoseconds")
  end

  def part_one_no_regex(dev \\ false) do
    input =
      if dev do
        File.read!("inputs.txt")
      else
        IO.read(:stdio, :eof)
      end

    start = System.os_time(:nanosecond)

    sum =
      String.split(input, "\n", trim: true)
      |> Stream.map(fn line ->
        digits =
        line
          |> String.to_charlist()
          |> Stream.map(&(&1 - 48))
          |> Stream.filter(&(&1 >= 0 and &1 < 10))

        first = Enum.at(digits, 0)
        last = Enum.at(digits, -1)

        first * 10 + last
      end)
      |> Enum.sum()

    endpoint = System.os_time(:nanosecond)
    duration = endpoint - start

    IO.puts(sum)
    IO.puts("Took #{duration} nanoseconds")
  end

  def part_one_no_regex_parallel(dev \\ false) do
    input =
      if dev do
        File.read!("inputs.txt")
      else
        IO.read(:stdio, :eof)
      end

    fun = fn line ->
      digits =
        line
        |> String.to_charlist()
        |> Stream.map(&(&1 - 48))
        |> Stream.filter(&(&1 >= 0 and &1 < 10))

      first = Enum.at(digits, 0)
      last = Enum.at(digits, -1)
      first * 10 + last
    end

    start = System.os_time(:nanosecond)

    sum =
      String.split(input, "\n", trim: true)
      |> Task.async_stream(&fun.(&1), max_concurrency: 10, timeout: :infinity)
      |> Stream.map(fn {:ok, val} -> val end)
      |> Enum.sum()

    endpoint = System.os_time(:nanosecond)
    duration = endpoint - start

    IO.puts(sum)
    IO.puts("Took #{duration} nanoseconds")
  end
end
