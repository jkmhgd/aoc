package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	file, err := os.Open("./input/4_input.txt")
	if err != nil {
		fmt.Println("Error opening input file:", err)
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var grid [][]int
	for scanner.Scan() {
		line := scanner.Text()
		var row []int

		for _, c := range line {
			if c == '@' {
				row = append(row, 1)
			} else {
				row = append(row, 0)
			}
		}
		grid = append(grid, row)
	}

	part1(grid)
	part2(grid)
}

func isAccessible(i int, j int, grid [][]int, rows int, cols int) bool {
	sum := 0

	if i > 0 {
		sum += grid[i-1][j]
	}

	if j > 0 {
		sum += grid[i][j-1]
	}

	if i > 0 && j > 0 {
		sum += grid[i-1][j-1]
	}

	if i < rows-1 {
		sum += grid[i+1][j]
	}

	if j < cols-1 {
		sum += grid[i][j+1]
	}

	if i < rows-1 && j < cols-1 {
		sum += grid[i+1][j+1]
	}

	if i > 0 && j < cols-1 {
		sum += grid[i-1][j+1]
	}

	if j > 0 && i < rows-1 {
		sum += grid[i+1][j-1]
	}

	if sum < 4 {
		return true
	}

	return false
}

func part1(grid [][]int) {
	rows := len(grid)
	cols := len(grid[0])

	part1 := 0
	for i, row := range grid {
		for j := range row {
			if grid[i][j] != 1 {
				continue
			}

			if isAccessible(i, j, grid, rows, cols) {
				part1++
			}
		}
	}
	fmt.Println("Part 1: ", part1)
}

type Coordinate struct {
	i int
	j int
}

func part2(grid [][]int) {
	rows := len(grid)
	cols := len(grid[0])

	part2 := 0
	var toRemove []Coordinate

	for {
		for i, row := range grid {
			for j := range row {
				if grid[i][j] != 1 {
					continue
				}

				if isAccessible(i, j, grid, rows, cols) {
					part2++
					toRemove = append(toRemove, Coordinate{i, j})
				}
			}
		}

		if len(toRemove) == 0 {
			break
		}

		for _, c := range toRemove {
			grid[c.i][c.j] = 0
		}

		toRemove = []Coordinate{}
	}

	fmt.Println("Part 2: ", part2)
}
