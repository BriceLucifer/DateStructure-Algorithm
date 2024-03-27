#include "stack.h"
#include <iostream>
using namespace std;

int main() {
  // 创建一个stack
  Stack<int> test = Stack<int>();
  if (test.is_empty()) {
    cout << "empty stack" << endl;
  } else {
    cout << "fail" << endl;
  }

  for (int i = 0; i < 10; i++) {
    test.push(i);
    cout << "adding elements :" << test.top() << endl;
  }

  cout << "length is :" << test.len() << endl;

  for (int j = 0; j < 10; j++) {
    cout << "popping elements :" << test.pop() << endl;
  }

  return 0;
}