#[derive(Debug,Clone,PartialEq)]
pub struct HashMap<T>{
    size:usize,
    slot:Vec<usize>,
    data:Vec<T>,
}
/*      
----------------------------------------------------------
        hash(key) = pos value:T                                           
    逻辑：                                                 
        1. 计算出键key的位置值pos                           
        2. 然后在slot插槽的pos处 保存key值                 
           如果出现哈希冲突 rehash(pos)                    
        3. 在data数据处的pos处 保存value                   
----------------------------------------------------------
    slot :  slot[pos] = key
    data :  data[pos] = value
*/

// 实现哈希表算法
impl <T:Default+Clone+PartialEq> HashMap<T> {
    // 构造函数
    pub fn new(capacity:usize)->Self{
        let mut slot = Vec::with_capacity(capacity);
        let mut data = Vec::with_capacity(capacity);
        for _ in 0..capacity{
            slot.push(0);
            data.push(Default::default());
        }
        Self{
            size:capacity,
            slot:slot,
            data:data,
        }
    }

    // 判断hashmap是不是空
    fn is_empty(&self)->bool{
        self.size == 0
    }

    // 计算hash 其实就是取余 通过key获得hash value 然后插入到
    // hashmap.slot[pos] = key 
    // hashmap.data[pos] = value;
    pub fn hash(&self,key:usize)->usize{
        key % self.size
    }

    // 重新计算hash 避免哈希冲突
    pub fn rehash(&self,pos:usize)->usize{
        (pos+1) % self.size
    }

    // hashmap 插入
    pub fn insert(&mut self,key:usize,value:T){
        // key不能为0
        if key == 0{
            panic!("no key is 0");
        }
        let pos = self.hash(key);

        // 判断是否冲突
        if self.slot[pos] == 0{
            self.slot[pos] = key;
            self.data[pos] = value;
        }else {
            // 查找新的位置
            let mut next = self.rehash(pos);
            // 这个判断条件也比较有意思 要么插槽不为0 要么插槽的存值不是key
            while self.slot[next] != 0 && self.slot[next] != key {
                next = self.rehash(next);
                // 插槽以及满
                if next == pos{
                    /* 这个判断条件其实不好理解 其实算数一下就很简单
                       example： 24%11= 2 如果循环一圈之后没有合适的插槽
                                (24+1)%11 = 3;
                                (24+2)%11 = 4 
                                ...依次类推
                       余数还是2, 那么(24+11)%11 = 2
                    */
                    println!("there is no slot for you to insert");
                    return;
                }
            }

            // 如果找到插槽
            if self.slot[next] == 0 {
                self.slot[next] = key;
                self.data[next] = value;
            }else {
                // 计算出来key和原来key相同 进行替换
                self.data[next] = value
            }
        }
    }

    // 删除方法
    pub fn remove(&mut self,key:usize)->Option<T>{
        if 0 == key{
            panic!("key can not be 0!!!");
        }
        
        let pos = self.hash(key);
        if 0 == self.slot[pos]{
            None
        }else if key == self.slot[pos]{
            // 如果一步找到
            self.slot[pos] = 0;     // 改key
            let data = Some(self.data[pos].clone()); // 克隆数据返回
            self.data[pos] = Default::default();    // value初始化
            // 返回data
            data
        }else {
            let mut data = None;
            let mut stop = false;
            let mut found = false;
            let mut cur = pos;

            while self.slot[cur] != 0 && !found && !stop {
                if key == self.slot[pos] {
                    found = true;
                    self.slot[cur] = 0;
                    data = Some(self.data[cur].clone());
                    self.data[cur] = Default::default();
                }else {
                    cur = self.rehash(cur);
                    // 没找到
                    if cur == pos {stop = true;}
                    // 退出循环
                }
            }
            // 如果找到 data就是Some(value) 找不到就是None
            data
        }

    }

    // 找到哈希表返回一个value
    pub fn get(&self,key:usize)->Option<&T>{
        if key == 0{
            panic!("key can not be 0");
        }
        let pos = self.hash(key);
        let mut data = None;
        let mut found = false;
        let mut stop = false;
        let mut cur = pos;
        // 循环判断
        while self.slot[cur] != 0 && !found && !stop {
            // 如果找到key 克隆+found = true;
            if self.slot[cur] == key{
                data = self.data.get(cur);
                found = true;
            }else {
                // 如果没找到key cur 刷新 然后和pos一一比较 
                // 如果遍历之后没有发现key 那就stop 然后退出循环
                cur = self.rehash(cur);
                if cur == pos{
                    stop = true
                }
            }
        }
        data
    }

    pub fn len(&self)->usize{
        let mut length = 0;
        for i in self.slot.iter(){
            if *i != 0{
                length += 1;
            }
        }
        length
    }

    pub fn contain_key(&self,key:usize)->bool{
        // 先确保key 不为0
        if key == 0{
            panic!("key can not be 0");
        }else {
            // 使vec自带的contain()函数 记住参数是不可变引用
            self.slot.contains(&key)
        }
    }

    pub fn contain_value(&self,value:T)->bool{
        // 利用vec的自带函数
        self.data.contains(&value)
    }

    // 迭代器
    pub fn into_iter(self)->IntoIter<T>{
        IntoIter(self)
    }

    pub fn iter(&self)->Iter<T>{
        let mut iterator = Iter{next:Vec::new()};
        for item in self.data.iter(){
            iterator.next.push(item)
        }
        iterator
    }

    pub fn itermut(&mut self)->IterMut<T>{
        let mut iterator = IterMut{next:Vec::new()};
        for item in self.data.iter_mut(){
            iterator.next.push(item);
        }
        iterator
    }
}

// 因为是键值对字典 我后期会考虑加入迭代器
#[derive(Debug,Clone,PartialEq)]
pub struct IntoIter<T>(HashMap<T>);
impl <T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item>{
        if self.0.size != 0{
            self.0.data.pop()
        }else {
            None
        }
    }
}

// 迭代器构造方式
#[derive(Debug,Clone,Default,PartialEq)]
pub struct Iter<'a,T:'a>{next:Vec<&'a T>}
impl <'a,T:'a> Iterator for Iter<'a,T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.pop()
    }
}

#[derive(Debug,Default,PartialEq)]
pub struct IterMut<'a,T:'a>{next:Vec<&'a mut T>}
impl <'a,T:'a> Iterator for IterMut<'a,T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.pop()
    }
}
