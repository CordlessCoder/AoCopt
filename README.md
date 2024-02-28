# AoCopt

Just a couple of people optimizing AoC problems a bit too far.

# Usage

## CLI Arguments

| short/long argument | meaning                                                      | default       |
| ------------------- | ------------------------------------------------------------ | ------------- |
| -y/year             | The year to run solutions for                                | 2023          |
| -d/day              | The day to run solutions for                                 |               |
| -p/part             | The part to run solutions for                                | Both parts    |
| -a/aoc_token        | The AOC session token to use for downloading the inputs.     |               |
|                     | Only necessary if you want to automatically download inputs. |               |
|                     | Can be specified in the environment variable AOC_TOKEN       |               |
| -b/bench            | The names of the solutions you want to run. When left empty, | All solutions |
|                     | all solutions for that year/day/part will be ran.            |               |
|                     | Can be provided multiple times to run multiple solutions.    |               |
| -c/clean            | Run the clean hook of the solutions                          | false         |

# Defining solutions
