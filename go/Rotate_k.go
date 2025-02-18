package main

import "fmt"

func RotateArray(data []int, k_step int) {
	n := len(data)
	ReverseArray(data, 0, k_step-1)
	fmt.Printf("%d\n", data)
	ReverseArray(data, k_step, n-1)
	fmt.Printf("%d\n", data)
	ReverseArray(data, 0, n-1)
	fmt.Printf("%d\n", data)
}

func ReverseArray(data []int, start int, end int) {
	i := start
	j := end

	for i < j {
		data[i], data[j] = data[j], data[i]
		i += 1
		j -= 1
	}
}

func main() {
	data := []int{1, 2, 3, 4, 5, 6, 7, 8}
	fmt.Printf("%d\n", data)
	k_step := 2
	RotateArray(data, k_step)
}
