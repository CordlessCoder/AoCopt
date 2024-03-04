#!/usr/bin/env python
from sys import stdin
from time import time_ns

input = stdin.buffer.read()

start = time_ns()

sum = 0

for line in input.split():
    digits = (c - 48 for c in line if 48 <= c <= 67)
    digits_reversed = (c - 48 for c in reversed(line) if 48 <= c <= 67)

    first = next(digits, 0)
    last = next(digits_reversed, first)

    sum += first * 10 + last

end = time_ns()

print(sum)
print(end - start)
