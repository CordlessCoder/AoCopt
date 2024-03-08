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
        ],
        d1_flow: [
          name: "d1_flow",
          about: "Parallelism via flow"
        ],
        d1_compr: [
          name: "d1_compr",
          about: "Comprehension to bypass having to split by newline"
        ],
        d1_compr2: [
          name: "d1_compr2",
          about: "Comprehension with a single pass. No pre-filtering"
        ]
      ]
    )

    args = Optimus.parse!(optimus, argv)

    case args do
      %{args: %{}} -> Optimus.parse!(optimus, ["--help"])
      {[:d1_regex], _args} -> Aoc.DayOne.part_one_regex()
      {[:d1_no_regex], _args} -> Aoc.DayOne.part_one_no_regex()
      {[:d1_no_regex_parallel], _args} -> Aoc.DayOne.part_one_no_regex_parallel()
      {[:d1_flow], _args} -> Aoc.DayOne.part_one_flow()
      {[:d1_compr], _args} -> Aoc.DayOne.part_one_compr()
      {[:d1_compr2], _args} -> Aoc.DayOne.part_one_compr2()
      {[:bench], _args} -> Aoc.Bench.bench()
    end

    System.halt(0)
  end
end
