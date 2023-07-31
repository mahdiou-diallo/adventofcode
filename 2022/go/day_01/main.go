package main

import (
	"fmt"
	"os"
	"runtime/debug"
	"strconv"
	"strings"
)

func main() {
	defer func() {
		if r := recover(); r != nil {
			fmt.Println("stacktrace from panic: \n" + string(debug.Stack()))
		}
	}()

	data, err := os.ReadFile("../../data/2022/01.txt")
	if err != nil {
		panic(err)
	}
	text := string(data)
	elves := strings.Split(trimWhiteSpace(text), "\n\n")
	// fmt.Println(len(elves))
	n := len(elves)
	allCalories := make([]int, n)
	for i, elf := range elves {
		// fmt.Printf("elf[%d] calories: %v\n", i, elf)
		allCalories[i] = getElfCalories(elf)
	}

	// fmt.Printf("all calories: %v\n", len(allCalories))
	fmt.Printf("max calories: %d\n", getMax(allCalories))

}

func trimWhiteSpace(text string) string {
	return strings.Trim(text, "\n \r\t")
}

func getElfCalories(elf string) int {
	calories := 0
	for _, calorie := range strings.Split(trimWhiteSpace(elf), "\n") {
		// fmt.Printf("current calorie: %d\n", calories)
		res, err := strconv.Atoi(trimWhiteSpace(calorie))
		if err != nil {
			panic(err)
		}
		calories += res
	}
	return calories
}

func getMax(calories []int) int {
	if len(calories) < 1 {
		panic("empty array")
	}
	max := calories[0]
	for _, calorie := range calories {
		if calorie > max {
			max = calorie
		}
	}
	return max
}
