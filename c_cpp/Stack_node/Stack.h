#ifndef STACK_NODE_STACK_H
#define STACK_NODE_STACK_H

#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>

typedef struct item{
    int data;
    struct item *next;
}Node;

// Node malloc function to make the function more useful
Node* new(int data);
void delete(Node *p);

typedef struct list{
    int size;
    Node *head;
    // 函数指针
    int (*len)(struct list*);
    bool (*is_empty)(struct list*);
    Node* (*peek)(struct list*);
    void (*push)(int,struct list*);
    void (*pop)(struct list*);
}Stack;

// Initialize
Stack init();
// functions that needed
int len(Stack *p);
bool is_empty(Stack *p);
Node* peek(Stack *p);
void push(int data,Stack *p);
void pop(Stack *p);





#endif //STACK_NODE_STACK_H
