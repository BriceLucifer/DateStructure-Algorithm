package main

import "fmt"

func SumArray(data []int) int {
	size := len(data)
	total := 0
	for index := 0; index < size; index++ {
		total = total + data[index]
	}
	return total
}

func main() {
	data := []int{1, 2, 3, 4, 4}
	fmt.Printf("total number fo data is %d\n", SumArray(data))
}
