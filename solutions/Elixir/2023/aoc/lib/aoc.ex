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
        d1: [
          name: "d1",
          about: "Performs the day one advent of code challenge part one"
        ]
      ]
    )

    args = Optimus.parse!(optimus, argv)

    case args do
      %{args: %{}} -> Optimus.parse!(optimus, ["--help"])
      {[:d1], args} -> Aoc.DayOne.part_one()
    end
  end
end
