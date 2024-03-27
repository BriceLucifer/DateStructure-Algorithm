#include "Stack.h"

int main()
{
    Stack list = init();
    int length = list.len(&list);
    printf("length is %d\n",length);
    list.push(12,&list);
    list.push(22,&list);
    list.push(32,&list);
    list.push(72,&list);
    Node *p = list.peek(&list);
    printf("top has value : %d\n",p->data);

    if(list.is_empty(&list)){
        printf("Stack is empty\n");
    } else{
        printf("Stack is not empty\n");
    }

    // 全部弹出
    for(int i = 0; i < 4; i++){
        list.pop(&list);
    }

    list.pop(&list);

    return 0;
}