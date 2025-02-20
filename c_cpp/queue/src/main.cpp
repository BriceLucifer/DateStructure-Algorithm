#include "queue.h"

int main(int argc, char** argv) {
    auto a = Queue<int>();
    a.push_back(12);
    a.push_back(14);
    a.push_back(15);
    cout << a.peek_first() << endl;
    cout << a.peek_last() << endl;

    a.print();
}
