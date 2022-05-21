package advent

import (
	"io/ioutil"
	"log"
	"strconv"
)

func Input(fileName string) string {
	content, err := ioutil.ReadFile(fileName)

	if err != nil {
		log.Fatal(err)
	}
	return string(content)
}

func PrepNumbers(strings []string) []int {
	var data []int

	for _, value := range strings {
		v, err := strconv.Atoi(value)
		if err != nil {
			log.Fatal(err)
		} else {
			data = append(data, v)
		}
	}
	return data
}

func Sum(arr []int) int {
	res := 0
	for i := 0; i < len(arr); i++ {
		res += arr[i]
	}
	return res
}

func IntegerToBinary(n uint16) string {
	return strconv.FormatInt(int64(n), 2)
}
