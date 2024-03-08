defmodule Aoc.DayOne do
  @compile {:inline, process: 1, char_conv: 1, check_digit: 1, comprehension: 2}

  defp char_conv(char), do: char - 48

  defp check_digit(char), do: char in ?0..?9

  defp newline do
    :binary.compile_pattern("\n")
  end

  defp process(line) do
    digits =
      :binary.bin_to_list(line)
      |> Enum.filter(&check_digit/1)

    first = char_conv(List.first(digits))
    last = char_conv(List.last(digits))
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

  defp comprehension(10, {sum, first, last}), do: {sum + first * 10 + last, 0, 0}

  defp comprehension(letter, {sum, 0, 0}) when letter in ?0..?9,
    do: {sum, letter - 48, letter - 48}

  defp comprehension(letter, {sum, first, _last}) when letter in ?0..?9,
    do: {sum, first, letter - 48}

  defp comprehension(_letter, {sum, first, last}), do: {sum, first, last}

  def part_one_compr(dev \\ false) do
    input =
      if dev do
        File.read!("inputs.txt")
      else
        IO.read(:stdio, :eof)
      end

    start = System.os_time(:nanosecond)

    {sum, _, _} =
      for <<x <- input>>, x == ?\n or x in ?0..?9, reduce: {0, 0, 0} do
        {sum, first, last} -> comprehension(x, {sum, first, last})
      end

    endpoint = System.os_time(:nanosecond)
    duration = endpoint - start

    IO.puts(sum)
    IO.puts("Took #{duration} nanoseconds")
  end

  def part_one_compr2(dev \\ false) do
    input =
      if dev do
        File.read!("inputs.txt")
      else
        IO.read(:stdio, :eof)
      end

    start2 = System.os_time(:nanosecond)
    arr = :binary.bin_to_list(input)

    {sum2, _, _} =
      Enum.reduce(arr, {0, 0, 0}, fn letter, acc ->
        comprehension(letter, acc)
      end)

    endpoint2 = System.os_time(:nanosecond)
    duration2 = endpoint2 - start2

    IO.puts(sum2)
    IO.puts("Took #{duration2} nanoseconds")
  end
end
