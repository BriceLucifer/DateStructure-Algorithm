package main

import "fmt"

func Func1() {
	fmt.Println("func1 line 1")
	Func2()
	fmt.Println("func1 line 2")
}

func Func2() {
	fmt.Println("func2 line 1")
}

func main() {
	fmt.Println("main line 1")
	Func1()
	fmt.Println("main line 2")
}
