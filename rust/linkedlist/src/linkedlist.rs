// 链表的实现单位是结点
type Link<T> = Option<Box<Node<T>>>;

// 节点
#[derive(Debug,Clone)]
pub struct Node<T>{
    pub elem : T,   // 存储元素
    pub next : Link<T>,
}

// 链表
#[derive(Debug)]
pub struct list<T>{
    pub size : usize,   // 大小
    pub head : Link<T>, // 头节点
}

impl <T> list<T> {
    // 为list<T> 创建方法
    pub fn new()->Self{
        Self { size: 0, head: None }
    }

    pub fn pop(&mut self)->Option<T>{
        self.head.take().map(|node|{
            self.head = node.next;
            self.size -= 1;
            node.elem
        })
    }

    pub fn is_empty(&self)->bool{
        self.size == 0
    }

    pub fn size(&self)->usize{
        self.size
    }

    pub fn push(&mut self,data:T){
        let mut temp = Node{elem:data,next : None};
        temp.next = self.head.take();
        self.size += 1;
        self.head = Some(Box::new(temp));
    }

    pub fn peek(&self)->Option<&T>{
        self.head.as_ref().map(|node|{
            &node.elem
        })
    }

    pub fn peek_mut(&mut self)->Option<&mut T>{
        self.head.as_mut().map(|node|{
            &mut node.elem
        })
    }

    pub fn into_iter(self)->IntoIter<T>{
        IntoIter(self)
    }

    pub fn iter(&self)->Iter<T>{
        Iter{ next:self.head.as_deref()}
    }

    pub fn itermut(&mut self)->IterMut<T>{
        IterMut { next: self.head.as_deref_mut() }
    }
}

#[derive(Debug)]
pub struct IntoIter<T>(list<T>);
impl <T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

#[derive(Debug)]
pub struct Iter<'a,T:'a>{next:Option<&'a Node<T>>}
impl <'a,T:'a> Iterator for Iter<'a,T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node|{
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

struct IterMut<'a,T:'a>{next:Option<&'a mut Node<T>>}
impl <'a,T:'a> Iterator for IterMut<'a,T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node|{
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}