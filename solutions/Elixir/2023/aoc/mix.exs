defmodule Aoc.MixProject do
  use Mix.Project

  def project do
    [
      app: :aoc,
      version: "0.1.0",
      elixir: "~> 1.15",
      build_embedded: Mix.env() == :prod,
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      escript: [main_module: Aoc],
      releases: releases(),
      # consolidate_protocols: false,
      # compilers: [:erlang, :tria, :app]
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger],
    ]
  end

  def releases do
    [
      aoc: [
        steps: [:assemble, &Burrito.wrap/1],
        burrito: [
          targets: [
            macos: [os: :darwin, cpu: :aarch64],
            linux: [os: :linux, cpu: :x86_64],
            windows: [os: :windows, cpu: :x86_64]
          ]
        ]
      ]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:optimus, "~> 0.5.0"},
      {:benchee, "~> 1.3", only: :dev},
      {:burrito, "~> 1.0"},
      {:flow, "~> 1.2"},
      # {:tria, github: "hissssst/tria"},
    ]
  end
end
