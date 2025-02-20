use std::collections::VecDeque;

pub fn is_valid(s: String) -> bool {
    if s.is_empty() {
        return false;
    }

    // stack using VecDeque
    let mut stack = VecDeque::new();

    for i in s.chars(){
        match i {
            '(' | '{' | '[' => stack.push_back(i),
            '}' => {
                if stack.back().unwrap().clone() != '{' {
                    stack.push_back(i);
                } else {
                    stack.pop_back();
                }
            },
            ']' => {
                if stack.back().unwrap().clone() != '[' {
                    stack.push_back(i);
                } else {
                    stack.pop_back();
                }
            },
            ')' => {
                if stack.back().unwrap().clone() != '(' {
                    stack.push_back(i);
                } else {
                    stack.pop_back();
                }
            }
            _ => {

            }
        }
    }

    return stack.is_empty();
}

fn main() {
    let s = "(((({{{}}}))))[][]".to_string();
    if is_valid(s) {
        println!("Matched");
    } else {
        println!("UnMatched");
    }
}
