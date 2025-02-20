#ifndef STACK_H
#define STACK_H

#include <iostream>
#include <vector>

using namespace std;

// 定义一个类
template <typename T> class Stack {
  private:
    int size;
    vector<T> data;

  public:
    Stack<T>() {
      this->data = vector<T>();
      this->size = 0;
    }

    // 是否为空
    bool is_empty() { return this->size == 0; }

    // 长度
    int len() { return this->size; }

    // 清空栈
    void clear() {
      this->data.clear();
      this->size = 0;
    }

    void push(T val) {
      data.push_back(val);
      size += 1;
    }

    // 返回栈顶元素
    auto top() { return data.back(); }

    auto pop() {
      size -= 1;
      auto elem = data.back();
      data.pop_back();
      return elem;
    }
};

#endif
