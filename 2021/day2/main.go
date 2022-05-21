package main

import (
	"fmt"
	"strconv"
	"strings"

	"destilore.com/advent"
)

func solutionPartOne(inputPrepped []Direction) int {
	var depth int = 0
	var horizontal int = 0

	for _, v := range inputPrepped {
		if v.vector == "forward" {
			horizontal += v.scalar
		} else if v.vector == "down" {
			depth += v.scalar
		} else if v.vector == "up" {
			depth -= v.scalar
		}
	}

	return horizontal * depth
}

func solutionPartTwo(inputPrepped []Direction) int {
	var depth int = 0
	var horizontal int = 0
	var aim = 0

	for _, v := range inputPrepped {
		if v.vector == "forward" {
			horizontal += v.scalar
			depth += aim * v.scalar
		} else if v.vector == "down" {
			aim += v.scalar
		} else if v.vector == "up" {
			aim -= v.scalar
		}
	}

	return horizontal * depth
}

type Direction struct {
	vector string
	scalar int
}

func prepDirections(unparsedDirections []string) []Direction {
	var directions []Direction
	for _, v := range unparsedDirections {
		d := strings.Split(v, " ")
		amount, _ := strconv.Atoi(d[1])
		directions = append(directions, Direction{vector: d[0], scalar: amount})
	}
	return directions
}

func main() {
	input := advent.Input("input.txt")
	s := strings.Split(input, "\n")
	data := prepDirections(s)

	resultPartOne := solutionPartOne(data)
	outputPartOne := fmt.Sprintf("Part one: %d", resultPartOne)
	fmt.Println(outputPartOne)

	resultPartTwo := solutionPartTwo(data)
	outputPartTwo := fmt.Sprintf("Part two: %d", resultPartTwo)
	fmt.Println(outputPartTwo)
}
