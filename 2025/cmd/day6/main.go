package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	part1()
}

func part1() {
	file, _ := os.Open("./input/6_input.txt")
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var operands [][]int
	var operators []string
	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Fields(line)

		if parts[0] == "+" || parts[0] == "*" {
			operators = parts
		} else {
			var values []int
			for _, p := range parts {
				value, _ := strconv.Atoi(p)
				values = append(values, value)
			}
			operands = append(operands, values)
		}
	}

	part1 := 0
	for p := range len(operands[0]) {
		problemTotal := 0
		for r := range len(operands) {
			if operators[p] == "*" {
				if problemTotal == 0 {
					problemTotal = 1
				}
				problemTotal *= operands[r][p]
			} else {
				problemTotal += operands[r][p]
			}
		}
		part1 += problemTotal
	}

	fmt.Println("Part 1:", part1)
}

func part2() {
	file, _ := os.Open("./input/6_input.txt")
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var operands [][]int
	var operators []string
	for scanner.Scan() {
		line := scanner.Text()
		line = strings.TrimSpace(line)
		parts := strings.Fields(line)

		if parts[0] == "+" || parts[0] == "*" {
			operators = parts
		} else {
			var values []int
			for _, p := range parts {
				value, _ := strconv.Atoi(p)
				values = append(values, value)
			}
			operands = append(operands, values)
		}
	}

	part2 := 0
	for p := range len(operands[0]) {
		problemTotal := 0
		for r := range len(operands) {
			if operators[p] == "*" {
				if problemTotal == 0 {
					problemTotal = 1
				}
				problemTotal *= operands[r][p]
			} else {
				problemTotal += operands[r][p]
			}
		}
		part2 += problemTotal
	}

	fmt.Println("Part 2:", part2)
}
