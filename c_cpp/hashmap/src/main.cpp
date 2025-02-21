#include "hashmap.h"

int main(int argc, char** argv) { 
    ArrayHashMap a = ArrayHashMap();

    a.put(12, "hellow");
    a.put(13, "Hello");

    a.print();
    a.put(14, "good");
    a.print();
}