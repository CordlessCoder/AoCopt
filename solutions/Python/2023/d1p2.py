#!/usr/bin/python
from sys import stdin
from time import time_ns


options = {
    b"one": 1,
    b"two": 2,
    b"three": 3,
    b"four": 4,
    b"five": 5,
    b"six": 6,
    b"seven": 7,
    b"eight": 8,
    b"nine": 9,
    b"0": 0,
    b"1": 1,
    b"2": 2,
    b"3": 3,
    b"4": 4,
    b"5": 5,
    b"6": 6,
    b"7": 7,
    b"8": 8,
    b"9": 9,
}

input = stdin.buffer.read()

start = time_ns()

sum = 0

for line in input.split():
    first = 0
    for left in range(len(line)):
        for option, value in options.items():
            if line[left:].startswith(option):
                first = value
                break
    last = first
    for right in range(len(line) - 1, -1, -1):
        for option, value in options.items():
            if line[right:].startswith(option):
                last = value
                break
    sum += first * 10 + last

end = time_ns()

print(sum)
print(end - start)
