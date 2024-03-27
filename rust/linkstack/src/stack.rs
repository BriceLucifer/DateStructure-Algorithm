type Link<T> = Option<Box<Node<T>>>;

// 利用节点实现stack
#[derive(Debug,Clone)]
pub struct Node<T> {
    data: T,
    next: Link<T>,
}

#[derive(Debug,Clone)]
pub struct Stack<T> {
    size : usize,
    top : Link<T>,
}

// 为node实现构造函数
// impl <T> Node<T> {
//     fn new(val:T)->Self{
//         Self { data: val, next: None }
//     }
// }
// 也可以不实现

impl <T> Stack<T> {
    // 构造函数
    pub fn new()->Self{
        Self { size: 0, top: None }
    }

    pub fn is_empty(&self)->bool{
        self.size == 0
    }

    // 头部插入
    pub fn push(&mut self,val:T){
        // let mut node = Box::new(Node::new(val));
        // (*node).next = self.top.take();
        // self.top = Some(node);
        // self.size += 1;
        let mut node = Box::new(Node{
            data:val,next:None
        });
        (*node).next = self.top.take();
        self.top = Some(node);
        self.size += 1;
    }

    pub fn pop(&mut self)->Option<T>{
        self.top.take().map(|node| {
            let node = *node;
            self.top = node.next;
            self.size -= 1;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T>{
        self.top.as_ref().map( | node | {
            &node.data
        })
    }

    pub fn size(&self)->usize{
        self.size
    }

    // 迭代器三板斧
    pub fn into_iter(self)->IntoIter<T>{
        IntoIter(self)
    }

    pub fn iter(&self)->Iter<T>{
        Iter{next:self.top.as_deref()}
    }

    pub fn itermut(&mut self)->IterMut<T>{
        IterMut{next:self.top.as_deref_mut()}
    }
}

// stacked的析构函数
impl <T> Drop for Stack<T> {
    fn drop(&mut self) {
        println!("Stack<T> is dropped!");
    }
}

// 迭代器
#[derive(Debug)]
pub struct IntoIter<T>(Stack<T>);
impl <T> Iterator for IntoIter<T>{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

#[derive(Debug,Clone)]
pub struct Iter<'a,T:'a> {next:Option<&'a Node<T>>}
impl <'a,T:'a> Iterator for Iter<'a,T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map( |node| {
            self.next = node.next.as_deref();
            &node.data
        })
    }
}

#[derive(Debug)]
pub struct IterMut<'a,T:'a> {next:Option<&'a mut Node<T>>}
impl <'a,T:'a> Iterator for IterMut<'a,T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(| node |{
            self.next = node.next.as_deref_mut();
            &mut node.data
        })
    }
}
