mod linkedlist;
use linkedlist::list;
fn main() {
    println!("---创建链表---");
    let mut linkedlist = list::new();
    if let true = linkedlist.is_empty() {
        println!("链表为空");
    }else {
        println!("链表不为空");
    }
    
    println!("头部添加元素");
    for i in 0..4{
        println!("添加元素:{i}");
        linkedlist.push(i);
    }

    // 查看元素
    print!("|head|");
    for item in linkedlist.iter(){
        print!("->{:?}",item);
    }
    print!("|end|");
    println!("");
    println!("头部弹出元素");
    for j in 0..4{
        println!("弹出元素:{}",linkedlist.pop().unwrap())
    }
}
