package main

import "fmt"

func SequentialSearch(data []int, value int) bool {
	for item := range data {
		if item == value {
			return true
		}
	}

	return false
}

func main() {
	data := []int{1, 2, 3, 4, 5}
	if SequentialSearch(data, 2) {
		fmt.Printf("found")
	}
}
