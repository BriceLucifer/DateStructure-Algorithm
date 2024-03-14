#[derive(Debug)]
pub struct Queue<T>{
    pub cap:usize,
    pub data:Vec<T>
}

impl<T> Queue<T>{
    pub fn new(size:usize)->Self{
        Self{
            cap:size,
            data:Vec::with_capacity(size),
        }
    }

    pub fn is_empty(&self)->bool{
        self.data.len() == 0
    }

    pub fn len(&self)->usize{
        self.data.len()
    }

    pub fn is_full(&self)->bool{
        self.len() == self.cap
    }
    
    pub fn clear(&mut self){
        self.data = Vec::with_capacity(self.cap);
    }

    pub fn enqueue(&mut self,val:T)->Result<(),String>{
        if self.cap == self.len(){
            return Err("No space for enqueue".to_string());
        }else {
            self.data.insert(0, val);
            Ok(())
        }
    }

    pub fn dequeue(&mut self)->Option<T>{
        if self.len() == 0{
            None
        }else {
            self.data.pop()
        }
    }
    // 实现迭代器
    // 迭代器 获取所有权
    pub fn IntoIter(self) -> IntoIter<T>{
        IntoIter(self)
    }

    pub fn iter(&self)->Iter<T>{
        let mut iterator = Iter{ 
            stack: Vec::new()};
        for item in self.data.iter(){
            iterator.stack.push(item);
        }

        iterator
    }

    pub fn iterMut(&mut self)->IterMut<T>{
        
        // 创建一个迭代器 Vec类型 然后迭代
        let mut iterator = IterMut{
            stack:Vec::new(),
        };

        for item in self.data.iter_mut(){
            iterator.stack.push(item);
        }
        iterator
    }
}

// 迭代器构造
pub struct IntoIter<T>(Queue<T>);
// 这个迭代功能是迭代返回队列头
// remove(index) -> T 也就是被remove的对象
impl <T:Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty(){
            Some(self.0.data.remove(0))
        }else {
            None
        }
    }
}

pub struct Iter<'a,T:'a>{stack:Vec<&'a T>}
impl <'a,T> Iterator for Iter<'a,T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.len() != 0{
            Some(self.stack.remove(0))
        }else {
            None
        }
    }
}

pub struct IterMut<'a,T:'a>{stack:Vec<&'a mut T>}
impl <'a,T> Iterator for IterMut<'a,T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.len() == 0{
            None
        }else {
            Some(self.stack.remove(0))
        }
    }
}