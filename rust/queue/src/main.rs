mod queue_t;
use queue_t::Queue;
fn main() {
    // 创建queue
    let mut queue = Queue::new(6);
    println!("创建空queue = {}",queue.is_empty());
    // 开始添加元素
    for i in 1..8{
        queue.enqueue(i);
    }
    println!("{:?}",queue.data);
}
