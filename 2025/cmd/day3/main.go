package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("./input/3_input.txt")
	if err != nil {
		fmt.Println("Error opening input file:", err)
		return
	}
	defer file.Close()

	part1(file)
	file.Seek(0, 0)
	part2(file)
}

func part1(file *os.File) {
	part1 := 0
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		runes := []rune(line)
		max := rune(0)
		maxIndex := 0

		for i := 0; i < len(runes)-1; i++ {
			if runes[i] > max {
				max = runes[i]
				maxIndex = i
			}
		}

		max2 := '0'
		for i := maxIndex + 1; i < len(runes); i++ {
			if runes[i] > max2 {
				max2 = runes[i]
				maxIndex = i
			}
		}

		joltageStr := string([]rune{max, max2})
		joltage, _ := strconv.Atoi(joltageStr)

		part1 += joltage
	}
	fmt.Println("Part 1:", part1)
}

func part2(file *os.File) {
	part2 := 0
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		runes := []rune(line)
		var joltageRunes []rune
		digitsSoFar := 0

		left := 0
		right := len(runes) - 12
		for {
			if digitsSoFar == 12 {
				break
			}

			max := rune(0)
			for i := left; i < right; i++ {
				if runes[i] > max {
					max = runes[i]
					left = i + 1
				}
			}

			digitsSoFar++
			right = len(runes) - (11 - digitsSoFar)

			joltageRunes = append(joltageRunes, max)
		}

		joltageStr := string(joltageRunes)
		joltage, _ := strconv.Atoi(joltageStr)

		part2 += joltage
	}
	fmt.Println("Part 2:", part2)
}
