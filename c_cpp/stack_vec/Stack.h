#ifndef STACK_VEC_STACK_H
#define STACK_VEC_STACK_H

#include <iostream>
#include <vector>

using namespace std;

template<typename T>
class STACK{
    public:
        int size;
        vector<T> data;
        STACK(){
            size = 0;
            data = vector<T>();
            cout << "Stack is constructed " << endl;
        }

        void push(T val){
            data.push_back(val);
            size += 1;
        }

        void pop(){
            if (size == 0){
                cout << "No element to pop" << endl;
            } else{
                data.pop_back();
                size -= 1;
            }
        }

        T peek(){
            return data[size-1];
        }

        bool is_empty(){
            return size == 0;
        }

        int len(){
            return size;
        }

        ~STACK(){
            cout << "STACK is distructed" << endl;
        }
};

#endif //STACK_VEC_STACK_H
