# Naive haskell advent of code implementation
Author: Aimar Ibarra @rolfan

# Build
You will need to install `stack`.
Use [ghcup](https://www.haskell.org/ghcup/) and make sure to install cabal.
Then just run `stack build` from the root directory of this implementation.

# Run
`stack run`
- input: The advent of code problem
- output: Line 1 is the result, line 2 is the time in nanoseconds

# Notes
- The actual implementation is inside `src`.
- I didn't use list comprehension in order to make the code more readable to non
  haskell users.
- There is a more aggresive optimization level, to turn it on change the configuration in `package.yaml`.
  The line on `ghc-options` containing `-O2` can be turned to `-O3`.
