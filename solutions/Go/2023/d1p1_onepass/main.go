package main

import (
	"fmt"
	"io"
	"os"
	"time"
)

func main() {
	var input []byte
	debug := false
	var reader io.Reader

	if debug {
		readFile, _ := os.Open("inputs.txt")
		reader = readFile
	} else {
		reader = os.Stdin
	}

    input, err := io.ReadAll(reader)
	if err != nil {
		panic(err)
	}

	start := time.Now()

	sum := 0

    for len(input) > 0 {
        var first = 0
        for len(input) > 0 && input[0] != '\n' {
            num := input[0] - '0'
            input = input[1:]
            if 0<= num && num <= 9 {
                first = int(num)
                break
            }
        }
        var last = first
        for len(input) > 0 && input[0] != '\n' {
            num := input[0] - '0'
            input = input[1:]
            if 0<= num && num <= 9 {
                last = int(num)
            }
        }
        if len(input) > 0 {
            // Consume newline
            input = input[1:]
        }
        sum += first * 10 + last
    }

	duration := time.Since(start).Nanoseconds()

	fmt.Println(sum)
	fmt.Println("Took", duration, "nanoseconds")
}
