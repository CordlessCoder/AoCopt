defmodule Aoc do
  def main(argv) do
    optimus = Optimus.new!(
      name: "aoc",
      description: "Advent of Code runner",
      version: "0.0.1",
      author: "Risa",
      about: "It's for advent of code",
      allow_unknown_args: false,
      parse_double_dash: false,
      args: [],
      flags: [
        print_header: [
          short: "-h",
          long: "--print-header",
          help: "Specifies whether to print header before outputs",
          multiple: false
        ]
      ],
      subcommands: [
        bench: [
          name: "bench",
          about: "Does all the benchy benchy benchee"
        ],
        d1_regex: [
          name: "d1_regex",
          about: "Performs the day one advent of code challenge part one"
        ],
        d1_no_regex: [
          name: "d1_no_regex",
          about: "Performs d1 aoc challenge without regex"
        ],
        d1_no_regex_parallel: [
          name: "d1_no_regex_parallel",
          about: "Performs d1_no_regex but lazily parallelized"
        ]
      ]
    )

    args = Optimus.parse!(optimus, argv)

    case args do
      %{args: %{}} -> Optimus.parse!(optimus, ["--help"])
      {[:d1_regex], _args} -> Aoc.DayOne.part_one_regex()
      {[:d1_no_regex], _args} -> Aoc.DayOne.part_one_no_regex()
      {[:d1_no_regex_parallel], _args} -> Aoc.DayOne.part_one_no_regex_parallel()
      {[:bench], _args} -> Aoc.Bench.bench()
    end

    System.halt(0)
  end
end
