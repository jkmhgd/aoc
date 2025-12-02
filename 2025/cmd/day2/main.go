package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	content, _ := os.ReadFile("./input/2_input.txt")
	input := string(content)
	idStrs := parseValues(input)
	sumP1 := 0
	sumP2 := 0

	for _, idStr := range idStrs {
		if !isValidIdP1(idStr) {
			id, _ := strconv.Atoi(idStr)
			sumP1 += id
		}
		if !isValidIdP2(idStr) {
			id, _ := strconv.Atoi(idStr)
			sumP2 += id
		}
	}

	fmt.Println("Part 1:", sumP1)
	fmt.Println("Part 2:", sumP2)
}

func parseValues(input string) []string {
	var result []string
	parts := strings.SplitSeq(input, ",")

	for p := range parts {
		p = strings.TrimSpace(p)

		r := strings.Split(p, "-")

		start, _ := strconv.Atoi(r[0])
		end, _ := strconv.Atoi(r[1])

		for i := start; i <= end; i++ {
			result = append(result, fmt.Sprint(i))
		}
	}

	return result
}

func isValidIdP1(id string) bool {
	len := len(id)
	mid := len / 2

	return id[0:mid] != id[mid:len]
}

func isValidIdP2(id string) bool {
	len := len(id)
	divisors := getDivisors(len)

	for _, divisor := range divisors {
		partLen := len / divisor
		curr := 0
		last := id[curr:partLen]
		isCurrValid := false

		for i := 1; i <= divisor; i++ {
			if last != id[curr:curr+partLen] {
				isCurrValid = true
				break
			}
			curr += partLen
		}

		if !isCurrValid {
			return false
		}
	}

	return true
}

func getDivisors(num int) []int {
	var divisors []int

	for i := 2; i <= num; i++ {
		if num%i == 0 {
			divisors = append(divisors, i)
		}
	}

	return divisors
}
