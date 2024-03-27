//
// Created by 23766 on 2024/3/27.
//

#include "Stack.h"

using namespace std;

int main(){
    // 声明STACK
    STACK<int> list;

    if (list.is_empty()){
        printf("list is empty");
    } else{
        printf("no list");
    }

    //入栈
    list.push(12);
    list.push(34);
    list.push(15);

    // 查看栈顶元素
    int Top = list.peek();
    cout << "top is " << Top << endl;

    // 长度
    int len = list.len();
    cout << "length is " << len << endl;

    return 0;
}
