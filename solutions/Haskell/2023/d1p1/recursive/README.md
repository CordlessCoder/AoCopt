# Naive haskell advent of code implementation
Author: Aimar Ibarra @rolfan

# Build
You will need to install `stack`.
Use [ghcup](https://www.haskell.org/ghcup/) and make sure to install cabal and stack.
Then just run `stack build` from the root directory of this implementation.

# Run
`stack run`
- input: The advent of code problem
- output: Line 1 is the result, line 2 is the time in nanoseconds

# Notes
- The actual implementation is inside `src`.
- There is a more aggresive optimization level, to turn it on change the configuration in `package.yaml`.
  The line on `ghc-options` containing `-O2` can be turned to `-O3`.

# Notes on implementation
Although haskell can be quite performant, it is easy to trip and make your program functionally (pun intended) a snail.
This implementation makes some assumptions about the input, the correctness of it being the first one.
Aside from that the algorithm to calculate the result is expressed clearly, therefore the compiler has a better time
optimizing the code.

# We need more speed!
To go faster a really simple option is to use arrays instead of lists :)
Not implemented here though.
