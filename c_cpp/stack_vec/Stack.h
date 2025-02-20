#ifndef STACK_VEC_STACK_H
#define STACK_VEC_STACK_H

#include <iostream>
#include <vector>

using namespace std;

template<typename T>
class STACK{
    public:
        vector<T> data;
        STACK(){
            data = vector<T>();
            cout << "Stack is constructed " << endl;
        }

        void push(T val){
            data.push_back(val);
        }

        void pop(){
            if (data.size() == 0){
                cout << "No element to pop" << endl;
            } else{
                data.pop_back();
            }
        }

        T peek(){
            return data[data.size()-1];
        }

        bool is_empty(){
            return data.size() == 0;
        }

        int len(){
            return data.size();
        }

        ~STACK(){
            cout << "STACK is distructed" << endl;
        }
};

#endif //STACK_VEC_STACK_H
