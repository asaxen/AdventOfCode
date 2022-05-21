package main

import (
	"fmt"
	"strconv"
	"strings"

	"destilore.com/advent"
)

func solutionPartOne(inputPrepped []uint16) int {
	return 0
}

func solutionPartTwo(inputPrepped []uint16) int {
	return 0
}

func computeGammaAndEpsilon(data []uint16) {
	//var bitMask uint16 = 1
	//var gamma uint16 = 0
	//var epsilon uint16 = 0

	initialValue := data[0]

	for _, value := range data[1:len(data)] {
		fmt.Printf("Values Representation of %s | %s = %s .\n", advent.IntegerToBinary(initialValue), advent.IntegerToBinary(value), advent.IntegerToBinary(initialValue|value))
		//fmt.Printf("Binary Representation of %s with mask %s is %s.\n", advent.IntegerToBinary(value), advent.IntegerToBinary(currentMask), advent.IntegerToBinary(value || currentMask))
		initialValue = value
	}

	// for i := 0; i < 12; i++ {
	// 	currentMask := bitMask << i
	// 	for _, value := range data {
	// 		fmt.Printf("Binary Representation of %s with mask %s is %s.\n", advent.IntegerToBinary(value), advent.IntegerToBinary(currentMask), advent.IntegerToBinary(value&currentMask))
	// 	}
	// }

}

func main() {
	input := advent.Input("input.txt")
	s := strings.Split(input, "\n")
	data := make([]uint16, len(s))
	for s_i, value := range s {
		if i, err := strconv.ParseInt(value, 2, 16); err != nil {
			fmt.Println(err)
		} else {
			//fmt.Println(i)
			data[s_i] = uint16(i)
		}
	}

	computeGammaAndEpsilon(data)

	// resultPartOne := solutionPartOne(data)
	// outputPartOne := fmt.Sprintf("Part one: %d", resultPartOne)
	// fmt.Println(outputPartOne)

	// resultPartTwo := solutionPartTwo(data)
	// outputPartTwo := fmt.Sprintf("Part two: %d", resultPartTwo)
	// fmt.Println(outputPartTwo)
}
