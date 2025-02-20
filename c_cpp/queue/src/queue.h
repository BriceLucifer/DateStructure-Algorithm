#ifndef QUEUE_H
#define QUEUE_H

#include <iostream>
#include <vector>
using namespace std;

template<typename T>
class Queue {
    private:
        vector<T> elements;
    public:
        Queue(){
            elements = vector<T>{};
        }

        auto push_front(T elem) -> void {
            auto begin = elements.begin();
            elements.insert(begin, elem);
        }

        auto pop_front() -> T {
            auto begin = elements[0];
            elements.erase(elements.begin());
        }

        auto push_back(T elem) -> void {
            elements.push_back(elem);
        }

        auto pop_back() -> T {
            auto elem = elements[elements.size() - 1];
            elements.pop_back();
            return elem;
        }
        
        auto peek_first() -> T {
            return elements[0];
        }

        auto peek_last() -> T {
            return elements[elements.size() - 1];
        }

        auto print() -> void {
            cout << "[";
            for (auto &elem:elements) {
                cout << elem << ",";
            }
            cout << "]" << endl; 
        }

        ~Queue() = default;
};

#endif