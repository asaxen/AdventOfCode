package main

import (
	"fmt"
	"strings"

	"destilore.com/advent"
)

func solutionPartOne(inputPrepped []int) int {
	increase := 0
	previous := inputPrepped[0]
	for _, v := range inputPrepped[1:len(inputPrepped)] {
		if v > previous {
			increase++
		}
		previous = v
	}
	return increase
}

func solutionPartTwo(inputPrepped []int) int {
	increase := 0
	var sum int = 0
	var previous int = 0
	for i, v := range inputPrepped {

		if (i + 2) < len(inputPrepped) {
			sum = v + inputPrepped[i+1] + inputPrepped[i+2]
			if previous > 0 && sum > previous {
				increase++
			}
		} else {
			break //Done
		}
		previous = sum
	}
	return increase
}

func main() {
	input := advent.Input("input.txt")

	s := strings.Split(input, "\n")
	data := advent.PrepNumbers(s)

	resultPartOne := solutionPartOne(data)
	outputPartOne := fmt.Sprintf("Part one: %d", resultPartOne)
	fmt.Println(outputPartOne)

	resultPartTwo := solutionPartTwo(data)
	outputPartTwo := fmt.Sprintf("Part two: %d", resultPartTwo)
	fmt.Println(outputPartTwo)
}
