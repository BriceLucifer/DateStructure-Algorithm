use std::ops::DerefMut;

use hashmap::HashMap;

mod hashmap;

fn main() {
    let mut hash = HashMap::new(11);
    for i in 11..22{
        hash.insert(i, i*2);
    }
    if hash.contain_key(12){
        println!("在key = 12时找到 value = {}",hash.get(12).expect("无法打开数据"));
    }else {
        println!("不包含这个key");
    }
    // // 尾部遍历
    // for i in hash.into_iter(){
    //     // 直接变形为迭代器
    //     println!("value = {:?}",i);
    // }

    // 引用遍历
    for i in hash.iter(){
        println!("&value = {}",i);
    }

    // 可变引用遍历
    println!("修改后的元素表");
    for j in hash.itermut(){
        *j *=3; 
        println!("&mut value = {}",j)
    }
    println!("{:?}",hash.into_iter().next());
}

#[test]
fn testHash(){
    let mut hashtest = HashMap::new(11);
    // 11个元素
    hashtest.insert(11, "dog");
    hashtest.insert(12, "cat");
    hashtest.insert(13, "bird");
    hashtest.insert(20, "frog");

    // get 函数测试
    assert_eq!(hashtest.get(11),Some("dog").as_ref());
    assert_eq!(hashtest.get(12),Some("cat").as_ref());
    assert_eq!(hashtest.get(13),Some("bird").as_ref());
    assert_eq!(hashtest.get(20),Some("frog").as_ref());
    assert_eq!(hashtest.get(14),None);

    //删除函数 测试
    hashtest.remove(20);
    hashtest.remove(12);
    assert_eq!(hashtest.get(20),None);
    assert_eq!(hashtest.get(12),None);

    //测试contain函数
    assert_eq!(hashtest.contain_key(11),true);
    assert_eq!(hashtest.contain_value("dog"),true);
    assert_eq!(hashtest.contain_key(12),false);
    assert_eq!(hashtest.contain_value("cat"),false);

    // 测试len函数
    assert_eq!(hashtest.len(),2);

    // 测试迭代器
    hashtest.insert(21, "kangroo");
    assert_eq!(hashtest.iter().next(),Some(&"kangroo"));
    assert_eq!(hashtest.itermut().next().map(|elem| elem.to_ascii_uppercase())
                ,Some("KANGROO".to_string()));
}
