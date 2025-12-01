package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("./input/1_input.txt")
	if err != nil {
		fmt.Println("Error opening input file:", err)
		return
	}
	defer file.Close()

	counter_p1 := 0
	counter_p2 := 0
	current := 50
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		dir := line[0]
		valueStr := line[1:]
		value, err := strconv.Atoi(valueStr)
		if err != nil {
			fmt.Println("Invalid value, exiting..")
			return
		}

		for range value {
			if dir == 'L' {
				current = (current - 1 + 100) % 100
			} else {
				current = (current + 1) % 100
			}
			if current == 0 {
				counter_p2++
			}
		}

		if current == 0 {
			counter_p1++
		}
	}

	fmt.Printf("Part 1: %d\n", counter_p1)
	fmt.Printf("Part 2: %d\n", counter_p2)
}
