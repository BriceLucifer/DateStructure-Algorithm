package main

import "fmt"

func isValid(s string) bool {
	if len(s) == 0 {
		return true
	}

	var stack []rune

	for _, i := range s {
		switch i {
		case ')':
			if len(stack) == 0 || stack[len(stack)-1] != '(' {
				stack = append(stack, i)
			} else {
				stack = stack[:len(stack)-1]
			}
		case ']':
			if len(stack) == 0 || stack[len(stack)-1] != '[' {
				stack = append(stack, i)
			} else {
				stack = stack[:len(stack)-1]
			}
		case '}':
			if len(stack) == 0 || stack[len(stack)-1] != '{' {
				stack = append(stack, i)
			} else {
				stack = stack[:len(stack)-1]
			}
		case '(', '[', '{':
			stack = append(stack, i)
		}
	}
	return len(stack) == 0
}

func main() {
	s := "(((({{{{}}}}))))"
	if isValid(s) {
		fmt.Println("Matched")
	}
}
