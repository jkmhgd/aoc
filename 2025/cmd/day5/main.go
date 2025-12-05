package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Range struct {
	start  int
	end    int
	merged bool
}

func main() {
	ranges, ingredients := parseInput()
	part1 := 0
	part2 := 0

	for _, ing := range ingredients {
		for _, r := range ranges {
			if ing >= r.start && ing <= r.end {
				part1++
				break
			}
		}
	}

	for i, r := range ranges {
		for j, r2 := range ranges {
			if i == j || r.merged {
				continue
			}
			if (r.end >= r2.start && r.end <= r2.end) || (r.start >= r2.start && r.start <= r2.end) {
				r.start = min(r.start, r2.start)
				r.end = max(r.end, r2.end)
				r2.merged = true
			}
		}
	}

	for _, r := range ranges {
		if !r.merged {
			part2 += (r.end - r.start) + 1
		}
	}

	fmt.Println("Part 1:", part1)
	fmt.Println("Part 2:", part2)
}

func parseInput() ([]*Range, []int) {
	file, _ := os.Open("./input/5_input.txt")
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var ranges []*Range
	for scanner.Scan() {
		line := scanner.Text()
		if len(line) == 0 {
			break
		}
		var r Range

		line = strings.TrimSpace(line)
		parts := strings.Split(line, "-")
		start, _ := strconv.Atoi(parts[0])
		end, _ := strconv.Atoi(parts[1])
		r.start = start
		r.end = end
		r.merged = false

		ranges = append(ranges, &r)
	}

	var ingredients []int
	for scanner.Scan() {
		line := scanner.Text()
		line = strings.TrimSpace(line)
		ingredient, _ := strconv.Atoi(line)

		ingredients = append(ingredients, ingredient)
	}

	return ranges, ingredients
}
