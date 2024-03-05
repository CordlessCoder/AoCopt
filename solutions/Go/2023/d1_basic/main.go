package main

import (
	"bufio"
	"fmt"
	"os"
	"time"
)

func main() {
	var input []string
	debug := false
	var scanner *bufio.Scanner

	if debug {
		readFile, _ := os.Open("inputs.txt")
		scanner = bufio.NewScanner(readFile)
	} else {
		scanner = bufio.NewScanner(os.Stdin)
	}

	for scanner.Scan() {
		line := scanner.Text()
		input = append(input, line)

		if line == "" {
			break
		}
	}

	start := time.Now()

	sum := 0
	for _, line := range input {
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
