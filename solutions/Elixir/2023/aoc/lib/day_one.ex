defmodule Aoc.DayOne do
  @compile {:inline, process: 1}

  defp newline do
    :binary.compile_pattern("\n")
  end

  defp process(line) do
    digits =
      String.to_charlist(line)
      |> Enum.filter(&(&1 >= ?0 and &1 <= ?9))

    first = List.first(digits) - 48
    last = List.last(digits) - 48

    first * 10 + last
  end

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
      String.split(input, newline(), trim: true)
      |> Stream.map(&process/1)
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

    start = System.os_time(:nanosecond)

    sum =
      String.split(input, newline(), trim: true)
      |> Task.async_stream(&process/1, max_concurrency: 10, timeout: :infinity)
      |> Stream.map(fn {:ok, val} -> val end)
      |> Enum.sum()

    endpoint = System.os_time(:nanosecond)
    duration = endpoint - start

    IO.puts(sum)
    IO.puts("Took #{duration} nanoseconds")
  end

  def part_one_flow(dev \\ false) do
    input =
      if dev do
        File.read!("inputs.txt")
      else
        IO.read(:stdio, :eof)
      end

    start = System.os_time(:nanosecond)

    sum =
      String.split(input, newline(), trim: true)
      |> Flow.from_enumerable()
      |> Flow.map(&process/1)
      |> Enum.sum()

    endpoint = System.os_time(:nanosecond)
    duration = endpoint - start

    IO.puts(sum)
    IO.puts("Took #{duration} nanoseconds")
  end
end
