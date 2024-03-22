// | 0  | 1 |  2 |  3 |
//   0  root left right
// 父节点 = 任意节点 /2 
// 左子节点 = 2p
// 右节点 = 2p+1 

// 宏
// 父节点

macro_rules! parent {
    ($child:ident) => {
        $child >> 1                              // 是计算自变量p
    };
}
// 左子节点
macro_rules! left_child {
    ($parent:ident) => {
        $parent << 1
    };
}
// 右子节点
macro_rules! right_child {
    ($parent:ident) => {
        ($parent << 1) + 1
    };
}

#[derive(Debug,Clone)]
pub struct BinaryHeap{
    size:usize,     // 数据量
    data:Vec<i32>,    // 数据
}

impl BinaryHeap {
    pub fn new()->Self{
        Self{
          size:0,
          data:vec![0],
          // 首位置为0 填充 无实际作用
        }
    }

    pub fn len(&self)->usize{
        self.size
    }

    pub fn is_empty(&self)->bool{
        self.size == 0
    }

    // 获取binaryheap中最小的数据
    pub fn min(&self)->Option<i32>{
        if self.size == 0{
            None
        }else {
            // 泛型用克隆 但是大部分使用引用就行
            Some(self.data[1])
        }
    }

    // 插入数据
    pub fn push(&mut self,val:i32){
        // 插入数据
        self.data.push(val);
        // 数据总量+1
        self.size += 1;
        // 调整数据 小数据上浮 不然就待在原地
        self.move_up(self.size);
    }

    // 小数据上浮
    pub fn move_up(&mut self,mut c:usize){
        loop {
          let p = parent!(c);
          if p <= 0{
            break;
          }

          if self.data[c] < self.data[p]{
            self.data.swap(c, p);
          }
          c = p; // 不断循环
          // 其实这个逻辑其实疏通一下就很好 
          /*
          比如： 一个序列 0 1 13 14 15 16 17 18 19 
                树的形状：
                            1
                       13       14
                    15   16   17  18
                19
            这样的形状 如果我添加一个数据 11 那么我就加在了最后也就是19 的右节点
            奇妙的发生了 11 的下标是9 那么 p = 4他会和4坐标 15 比较  11 < 15
            转移 就变成了
                11
            19      15
            11 继续比较 4 / 2 = 2 也就是13  13 < 11 11和13 交换 就是
                             1
                        11        14
                    13    16    17  18
                 19   15
          */
        }
    }

    // pop() 弹出最小数据
    pub fn pop(&mut self)->Option<i32>{
        if 0 == self.size{
            None
        }else if self.size == 1{
            self.data.pop()
        }else {
            // 先交换
            self.data.swap(1, self.size);
            // 再弹出获取
            let val = self.data.pop();
            // 减少size
            self.size -= 1;
            // 大数据下沉 -> 在move_down 详细介绍
            self.move_down(1);
            val
        }
    }

    // 最小子节点
    pub fn min_child(&self,i:usize)->usize{
        let (lc,rc) = (left_child!(i),right_child!(i));
        if rc > self.size{
            lc
        }else if self.data[lc] < self.data[rc]{
            lc
        }else {
            rc
        }
        // 这个逻辑关系可以详细解释一下：
        /*
            lc , rc = p<<2 , (p<<2)+1;
            rc 如果比self.size 大 那么说明二叉树没填满 选lc
            如果 二叉树填满了 self.data[lc] < slef.data[rc]
                那就选lc
            如果 没有其他情况 就是rc了
        */
    } 

    // 大数据下沉
    pub fn move_down(&mut self,mut c:usize){
        loop {
            let lc = left_child!(c);
            if lc > self.size{
                break;
            }

            let mc = self.min_child(c);
            if self.data[mc] < self.data[c]{
                self.data.swap(mc, c);
                // 大数据下沉
            }
            c = mc;
        }
    }

    /*
        举例说明大数据下沉和最小节点的选择 向下衍生
                            1
                    13               14
                15     16        17      18
             19  20  22 23     34  35   44  45

        1. 先将45 下标25 = self.size 和 下标1 1 交换
        2. 然后将 self.data.pop() 弹出这个25下标 元素为1 
        3. 然后这时候 头结点是45 不合适 咱们就得重新调整 大数据下沉
        4. 首先选择合适的最小子节点 
            1. 先判断是不是就他一个元素 如果是直接break
            2. 比如 举例
                          45
                        13  14
            判断 1 -> 2 3
            self.size > lc rc
            所以开始选择最小节点
            rc < self.size 说明有右节点
            self.data[lc] < slef.data[lc]
            找到最小子节点 13 下标是2
            45 和 13 交换 然后继续重复过程 直到 无法进行
            为啥要找最小的 应为二叉堆本质就是最小是根节点在上面 
            然后把小的放在上面 大的下沉
    */

    pub fn build_new(&mut self,arr:&[i32]){
        for _ in 0..self.size{
            self.data.pop();
        }

        for &elem in arr{
            self.data.push(elem);
        }
        self.size = arr.len();

        // 调整小顶堆
        let Ls = self.size;
        let mut p = parent!(Ls);
        while p!=0 {
            self.move_down(p);
            p -= 1;
        }
    }

    // 从切片加入元素
    pub fn add_slice(&mut self,arr:&[i32]){
        for &val in arr{
            self.push(val);
        }
    }
}

