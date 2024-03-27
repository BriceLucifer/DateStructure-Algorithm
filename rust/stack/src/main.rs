mod stack;
use stack::Stack;

fn main(){
    // 创建Stack
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    // 入栈三个元素
    println!("size:{},{:?}",s.len(),s.data);
    println!("栈顶元素:{}",s.peek().unwrap());
    // 修改栈顶元素
    if let Some(top) = s.peek_mut() {
        *top += 12;
    }
    // 查看栈顶元素
    println!("新版栈顶元素:{}",s.peek().unwrap());


    // 迭代
    let sum = s.iter();
    for item in sum{
        println!("-{}-",item);
    }

    // 迭代修改
    let sum2 = s.iter_mut();
    for item2 in sum2{
        *item2 *= 2;
        println!("{}",item2);
    }
    let mut iter = s.into_iter();
    println!("{}",iter.next().unwrap());

    let pars = "((()))()((()))";
    match par_checker1(pars) {
        true => println!("括号匹配"),
        false => println!("括号不匹配")
    }

    // 测试par_check2
    let sa = "{()}]";
    let sb = "{}()[]";
    println!("sa balance :{}\nsb balance :{}",
        par_checker2(sa),par_checker2(sb));

}

// 括号匹配程序
fn par_checker1(par:&str)->bool{
    let mut char_list = Vec::new();
    for c in par.chars(){
        char_list.push(c);
    }
    let mut index = 0;
    // 括号是否匹配
    let mut balance = true;
    // 创建一个栈
    let mut stack = Stack::new();
    while index < char_list.len() && balance {
        let c = char_list[index];
        if '(' == c {
            stack.push(c);
        }else {
            // 先检查是否空栈 再考虑弹出匹配
            if stack.is_empty(){
                balance =  false;
            }else {
                let _r = stack.pop();
            }
        }
        index += 1;
    }
    balance && stack.is_empty()

}

// 检查是不是这三个符号之一
fn par_match(open:char,close:char)->bool{
    let opens = "([{";
    let closers = ")]}";
    opens.find(open) == closers.find(close)
}

fn par_checker2(par:&str)->bool{
    let mut char_list = Vec::new();
    for c in par.chars(){
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true;
    let mut stack = Stack::new();

    // 老样子
    while index < char_list.len() && balance {
        let c = char_list[index];
        if '(' == c || '{' == c || '[' == c{
            stack.push(c);
        }else {
            if stack.is_empty() {
                balance = false;
            }else {
                let top = stack.pop().unwrap();
                if !par_match(top, c){
                    // 如果判断结果是正确的 那就保持balance是true
                    balance = false
                }
            }
        }
        index += 1;
    }
    balance && stack.is_empty()
}
