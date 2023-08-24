/*
HashMap<K, V> 类型储存了一个键类型 K 对应一个值类型 V 的映射。
它通过一个 哈希函数（hashing function）来实现映射，决定如何将键和值放入内存中。
哈希 map 的数据储存在堆上，哈希 map 所有的键必须是相同类型，值也必须都是相同类型。
*/
// 从标准库导入 Hashmap
use std::collections::HashMap;

fn main() {
    // 使用 new 创建一个空的 HashMap
    let mut scores = HashMap::new();
    // 使用 insert 增加元素
    scores.insert(String::from("Yellow"), 28);
    scores.insert(String::from("Green"), 35);
    println!("Hashmap scores: {:?}", scores);

    // 使用 zip 方法来创建一个元组的 vector，再用 vector 的 collect 方法转换成 HashMap
    let teams = vec![String::from("Pink"), String::from("Blue")];
    let initial_scores = vec![53, 66];
    // HashMap<_, _> 类型标注是必要的，可以使用下划线占位来忽略键和值的参数类型
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("Hashmap scores: {:?}", scores);
    println!("teams: {:?}", teams);
    println!("initial_scores: {:?}", initial_scores);

    // Hashmap 中键值所有权的转移
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut hmap = HashMap::new();
    // 此处field_name 和 field_value 的所有权都发生了转移
    hmap.insert(field_name, field_value);
    println!("hmap: {:?}", hmap);
    // 会报错：borrow of moved value: `field_name` 和 borrow of moved value: `field_value`
    // println!(
    //     "field_name: {:?}, field_value: {:?}",
    //     field_name, field_value
    // );

    // 通过 get 方法获取对应键的值，get 返回 Option<V>，如果不存在则返回 None
    match scores.get(&String::from("Pink")) {
        Some(value) => println!("Pink's score is {}", value),
        None => println!("No score for Pink"),
    }

    // 遍历哈希 map 中的每一个键值对
    for (key, value) in &scores {
        println!("key: {:?}, value: {:?}", key, value);
    }
}
