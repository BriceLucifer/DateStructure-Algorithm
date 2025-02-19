package main

import "fmt"

func ToHuTil(num int, from string, to string, temp string) {
	if num < 1 {
		return
	}
	ToHuTil(num-1, from, temp, to)
	fmt.Println("Move Disk", num, "from peg", from, "to peg", to)
	ToHuTil(num-1, temp, to, from)
}

func TowersOfHanoi(num int) {
	fmt.Println("The sequence of moves involved in the Tower of Hanoi are:")
	ToHuTil(num, "A", "B", "C")
}

func main() {
	TowersOfHanoi(3)
}
