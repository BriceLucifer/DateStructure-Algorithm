#[derive(Debug)]
pub struct Stack<T>{
    pub size:usize,
    pub data:Vec<T>,
}

impl <T> Stack<T>{
    // new构造函数
    pub fn new()->Self{
        Self{
            size : 0,
            data : Vec::new(),
        }
    }

    // 是否空
    pub fn is_empty(&self)->bool{
        self.size == 0
    }

    // 长度函数
    pub fn len(&self) ->usize{
        self.size
    }

    // 清空栈
    pub fn clear(&mut self){
        self.size = 0;
        self.data.clear();
    }

    // 数据保存在Vec尾部
    pub fn push(&mut self,val:T){
        self.data.push(val);
        self.size += 1;
    }

    // 弹出数据
    pub fn pop(&mut self)->Option<T>{
        if self.size == 0 {return None;}
        self.size -= 1;
        self.data.pop()
    }

    // 弹出栈顶 不可变
    pub fn peek(&self)->Option<&T>{
        if 0 == self.size{return None}
        self.data.get(self.size-1)
    }

    // 弹出栈顶 可变修改栈顶元素
    pub fn peek_mut(&mut self)->Option<&mut T>{
        if self.size == 0{return None;}
        self.data.get_mut(self.size-1)
    }

    // copy的迭代器
    pub fn into_iter(self)->IntoIter<T>{
        IntoIter(self)
    }

    pub fn iter(&self)->Iter<T>{
        let mut iterator = Iter{stack: Vec::new()};
        for item in self.data.iter(){
            iterator.stack.push(item);
        }
        // 将所有的元素全部push给迭代器的stack
        iterator
    }
    // 可变迭代器
    pub fn iter_mut(&mut self)->IterMut<T>{
        let mut iterator = IterMut{
            stack:Vec::new()
        };
        for item in self.data.iter_mut(){
            iterator.stack.push(item);
        }
        iterator
    }


}

// 实现三种迭代功能
// 实现克隆
pub struct IntoIter<T>(Stack<T>);
impl <T:Clone> Iterator for IntoIter<T> {
    type Item =  T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty(){
            self.0.size -= 1;
            self.0.data.pop()
        }else {
            None
        }
    }
}

// 不可变引用
pub struct Iter<'a,T:'a> {stack: Vec<&'a T>}
impl <'a,T> Iterator for Iter<'a,T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

// 可变引用
// 'a 生命周期
pub struct IterMut<'a, T:'a> {stack: Vec<&'a mut T>}
impl <'a, T> Iterator for IterMut<'a,T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}