mod stack;
use stack::Stack;
fn main() {
    println!("Stack in linkedlist");
    let mut stacklist:Stack<i32> = Stack::new();
    for i in 0..5{
        println!("入栈元素:{:?}",i);
        stacklist.push(i);
    }
    let top = stacklist.peek().take().expect("no value");
    println!("top has value : {}",top);
    viewer(&stacklist);
    // 修改Stack
    for item in stacklist.itermut(){
        *item *= 2;
    }
    viewer(&stacklist);
}

fn viewer(stacklist:&Stack<i32>){
    print!("Top->");
    for item in stacklist.iter(){
        print!("{}->",*item);
    }
    print!("End");
    println!("");
}
