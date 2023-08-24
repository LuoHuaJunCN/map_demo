/*
HashMap<K, V> 类型储存了一个键类型 K 对应一个值类型 V 的映射。
它通过一个 哈希函数（hashing function）来实现映射，决定如何将键和值放入内存中。
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
}
