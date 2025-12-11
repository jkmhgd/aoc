package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	file, _ := os.Open("./input/7_input.txt")
	defer file.Close()
	scanner := bufio.NewScanner(file)

	var grid [][]rune
	for scanner.Scan() {
		line := scanner.Text()
		var row []rune
		for _, char := range line {
			row = append(row, char)
		}
		grid = append(grid, row)
	}

	part1(grid)
	part2(grid)
}

func part1(grid [][]rune) {
	part1 := 0
	for r, row := range grid {
		for c, char := range row {
			if char == 'S' {
				grid[r+1][c] = '|'
			} else if r > 0 && char == '.' && grid[r-1][c] == '|' {
				grid[r][c] = '|'
			} else if r > 0 && char == '^' && grid[r-1][c] == '|' {
				grid[r][c-1] = '|'
				grid[r][c+1] = '|'
				part1++
			}
		}
	}

	fmt.Println("Part 1:", part1)
}

func part2(grid [][]rune) {
	rows, cols := len(grid), len(grid[0])
	result := make([][]int, rows)
	for i := range result {
		result[i] = make([]int, cols)
	}

	for r, row := range grid {
		for c, char := range row {
			if char == 'S' {
				grid[r+1][c] = '|'
				result[r+1][c] = 1
			} else if r > 0 && char == '.' && grid[r-1][c] == '|' {
				grid[r][c] = '|'
				result[r][c] = result[r-1][c]
			} else if r > 0 && char == '^' && grid[r-1][c] == '|' {
				grid[r][c-1] = '|'
				grid[r][c+1] = '|'
				result[r][c-1] = result[r][c-1] + result[r-1][c]
				result[r][c+1] = result[r][c+1] + result[r-1][c]
			} else if r > 0 && char == '|' && grid[r-1][c] == '|' {
				result[r][c] = result[r][c] + result[r-1][c]
			}
		}
	}

	part2 := 0
	for _, timelines := range result[len(result)-1] {
		part2 += timelines
	}

	fmt.Println("Part 2:", part2)
}
