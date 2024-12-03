#!/bin/sh
if [ "$#" -lt 3 ]; then
    echo "Please provide the year, day and part of the solution you want to create"
    exit
fi
mkdir -p "$1"
cd "$1" || exit
path="d${2}p$3"
if [ -d "$path" ]; then
    echo "Solution path \`$1/$path\` already exists!"
    exit 1
fi
cargo new "$path" --name "${path}_$1"
cargo add aoc_util --package "${path}_$1"
