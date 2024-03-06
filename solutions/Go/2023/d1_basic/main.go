package main

import (
	"fmt"
	"io"
	"os"
	"strings"
	"time"
)

func main() {
	debug := false
	var reader io.Reader

	if debug {
		readFile, _ := os.Open("inputs.txt")
		reader = readFile
	} else {
		reader = os.Stdin
	}

	stdin, err := io.ReadAll(reader)
	if err != nil {
		panic(err)
	}
	input := string(stdin)

	start := time.Now()

	sum := 0
	for _, line := range strings.Split(input, "\n") {
		for _, char := range line {
			num := int(char - '0')

			if num < 10 && num >= 0 {
				sum += (num * 10)
				break
			}
		}

		for i := len(line) - 1; i >= 0; i-- {
			num := int(line[i] - '0')

			if num < 10 && num >= 0 {
				sum += num
				break
			}
		}
	}

	duration := time.Since(start).Nanoseconds()

	fmt.Println(sum)
	fmt.Println("Took", duration, "nanoseconds")
}
