mod binary_heap;
use binary_heap::BinaryHeap;

fn main() {
    let mut bh = BinaryHeap::new();
    for i in 0..7{
        bh.push(i);
    }
    println!("bh is empty is {}",bh.is_empty());
    println!("bh's min is {}",bh.min().unwrap());
    println!("bh pop min {}",bh.pop().unwrap());
    println!("bh size is {}",bh.len());


    let num = [-1,18,11,14,12,34,56,78];
    bh.add_slice(&num);
    println!("bh is empty is {}",bh.is_empty());
    println!("bh's min is {}",bh.min().unwrap());
    println!("bh pop min {}",bh.pop().unwrap());
    println!("bh size is {}",bh.len());

    bh.build_new(&num);
    println!("bh is empty is {}",bh.is_empty());
    println!("bh's min is {}",bh.min().unwrap());
    println!("bh pop min {}",bh.pop().unwrap());
    println!("bh size is {}",bh.len());

}
