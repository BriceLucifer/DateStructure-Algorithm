fn main() {
    let num = [1,2,3,4,5];
    let r = num1(&num);
    let r2 = num2(&num);
    println!("结果是{},{}",r,r2);
}

// 递归算法
// num1 是头到尾巴
fn num1(numbs:&[i32])->i32{
    if 1 == numbs.len(){
        return numbs[0];
    }else {
        let first = numbs[0];
        first + num1(&numbs[1..])
    }
}

// num2 是尾巴到头
fn num2(numbs:&[i32])->i32{
    if 1 == numbs.len(){
        numbs[0]
    }else {
        let last = numbs[numbs.len()-1];
        num2(&numbs[0..numbs.len()-1])+last
    }
}

/*
    递归三定律：基本情况 向基本情况靠近 用递归方式调用自身
*/

// 汉诺塔游戏 a b c
fn hanno(height:u32,A:&str,C:&str,B:&str){
    if height >= 1{
        hanno(height-1, A, C, B);
        println!("")
    }
}
