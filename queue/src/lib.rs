pub mod queue_t;
use queue_t::Queue;

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn queue_new(){
        let mut queue = Queue::new(4);
        for i in 0..4{
            queue.enqueue(i);
        }
        assert_eq!(queue.cap,4);
    } 
    #[test] 
    fn enqueueTest(){
        let mut queue:Queue<i32> = Queue::new(5);
        assert!(queue.is_empty());
    }
    #[test]
    fn iteratortest(){
        let mut queue = Queue::new(4);
        for i in 0..4{
            queue.enqueue(i);
        }
        let obj = queue.IntoIter().next().unwrap();
        assert_eq!(obj,3);
    }
}