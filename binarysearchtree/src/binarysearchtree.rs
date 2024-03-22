use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::Deref;

type Link<T,U> = Option<Box<BST<T,U>>>;

pub struct BST<T,U> {
    key:Option<T>,
    value:Option<U>,
    left:Link<T,U>,
    right:Link<T,U>,
}
// basic:
// KEY: left < parent < right

impl <T,U>  BST<T,U> 
    where T:Clone+Debug+Ord,U:Clone+Debug+Ord{
    
    // 构造函数
    pub fn new()->Self{
        Self { key: None, value: None, left: None, right: None }
    }

    pub fn is_empty(&self)->bool{
        // 若key为空 则树空
        self.key.is_none()
    }

    pub fn len(&self)->usize{
        self.calc_len(0)
    }

    // 计算长度 从一个数字开始迭代 其实算是为了递归加了一个i
    pub fn calc_len(&self,mut i:usize)->usize{
        if self.key.is_none(){
            return  i;
        }

        i +=1;

        if !self.left.is_none(){
            i = self.left.as_ref().unwrap().calc_len(i);
        }

        if !self.right.is_none(){
            i = self.right.as_ref().unwrap().calc_len(i);
        }

        i
    }

    // 前序排列
    // root - left - right
    pub fn preorder(&self){
        // 根 
        println!("key:{:#?},value:{:#?}",self.key,self.value);
        // 左
        match &self.left {
            Some(left) => println!("key:{:#?},value:{:#?}",left.key,left.value),
            None => ()
        }
        // 右
        match &self.right {
            Some(right) => println!("key:{:#?},value:{:#?}",right.key,right.value),
            None => {}
        }
    }

    // 中序排列
    // left root right
    pub fn inorder(&self){
        // 左
        match &self.left {
            Some(left) => println!("key:{:#?},value:{:#?}",left.key,left.value),
            None => ()
        }

        // 根 
        println!("key:{:#?},value:{:#?}",self.key,self.value);
        
        // 右
        match &self.right {
            Some(right) => println!("key:{:#?},value:{:#?}",right.key,right.value),
            None => {}
        }
    }

    // 后序排序
    // left - right - root
    pub fn postorder(&self){
        // 左
        match &self.left {
            Some(left) => println!("key:{:#?},value:{:#?}",left.key,left.value),
            None => ()
        }
        
        // 右
        match &self.right {
            Some(right) => println!("key:{:#?},value:{:#?}",right.key,right.value),
            None => {}
        }

        // 根 
        println!("key:{:#?},value:{:#?}",self.key,self.value);
    }

    // 插入操作 如果key为None 直接插入 如果有相同的 跟新替换
    pub fn insert(&mut self,Key:T,Value:U){
        // 如果没数据
        if self.key.is_none(){
            self.key = Some(Key);
            self.value = Some(Value);
        }else {
            match &self.key {
                Some(k) =>{
                    // 如果key 值相同 直接替换
                    if *k == Key{
                        self.value = Some(Value);
                        return;
                    }

                    
                    // 如果没找到
                    let child = if *k < Key {
                        &mut self.left
                    }else {
                        &mut  self.right
                    };

                    // 根据节点递归下去
                    match child {
                        Some(ref mut node)=>{
                            node.insert(Key, Value);
                        },
                        None => {
                            let mut node = BST::new();
                            node.insert(Key, Value);
                            *child = Some(Box::new(node));
                        }
                    }
                },
                None => (),
            }
        }
    }

    // 查找函数
    pub fn search(&self,Key:&T)->bool{
        match &self.key {
            Some(k) => {
                match k.cmp(&Key) {
                    Ordering::Equal => true,
                    Ordering::Greater => {
                        match &self.right {
                            Some(node) => node.search(Key),
                            None => false,
                        }
                    }
                    Ordering::Less => {
                        match &self.left {
                            Some(node) => node.search(Key),
                            None => false,
                        }
                    }
                }
            } 
            None => false,
        }
    }

    // 寻找最小值 左子树
    pub fn min(&self)->(Option<&T>,Option<&U>){
        match &self.left {
            Some(node) => {
                node.min()
            },

            None => {
                match &self.key {
                    Some(key) => {
                        (Some(&key),self.value.as_ref())
                    },
                    None => {
                        (None,None)
                    },
                }
            },
        }
    }
    
    // 查找最大值
    pub fn max(&self)->(Option<&T>,Option<&U>){
        match &self.right {
            Some(node) => node.max(),

            None => {
                match &self.key {
                    Some(key) => (Some(&key),self.value.as_ref()),
                    None => (None,None),
                }
            }
        }
    }

    // 获取值
    pub fn get(&self,Key:&T)->Option<&U>{
        match &self.key {
            None => None,
            Some(k) => {
                match k.cmp(&Key) {
                    Ordering::Equal =>{self.value.as_ref()},
                    Ordering::Greater =>{
                        match &self.right {
                            Some(node) => node.get(Key),
                            None => None,
                        }
                    }
                    Ordering::Less => {
                        match &self.left {
                            Some(node) => node.get(Key),
                            None => None,
                        }
                    }
                }
            }
        }
    }
}
