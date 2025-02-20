#include "Stack.h"

Node* newNode(int data){
    Node *p = (Node*)malloc(sizeof(Node));
    p->data = data;
    p->next = NULL;
    return p;
}

void deleteNode(Node *p){
    free(p);
    printf("has freed the memory on heap\n");
}

Stack init(){
    Stack new = {
            .size = 0,
            .head = NULL,
            .peek = peek,
            .len = len,
            .pop = pop,
            .push = push,
            .is_empty = is_empty,
    };
    return new;
}

int len(Stack *p){
    return p->size;
}

bool is_empty(Stack *p){
    return p->size == 0;
}

Node* peek(Stack *p){
    return p->head;
}

void push(int data,Stack *p){
    if (p->head == NULL){
        p->head = newNode(data);
        p->size += 1;
        return ;
    } else{
        Node *temp = newNode(data);
        temp->next = p->head;
        p->head = temp;
        p->size += 1;
        return ;
    }
}

void pop(Stack *p){
    if(p->size == 0){
        printf("no element to pop\n");
        return ;
    } else{
        Node *temp = p->head;
        p->size -= 1;
        printf("pop Node: {%d}\n",temp->data);
        p->head = p->head->next;
        deleteNode(temp);
        return ;
    }
}

