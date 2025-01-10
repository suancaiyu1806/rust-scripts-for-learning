use std::collections::HashMap;

fn main() {
    // 1. 动态数组 Vec<T>
    // 1.1 初始化
    // Vec的长度可以动态增长
    let _: Vec<i32> = Vec::new(); // 初始化一个空的Vec，类型为i32，长度为0，即不进行内存分配
    let _: Vec<i32> = Vec::with_capacity(16); // 初始化一个空的Vec，类型为i32，提前分配好长度为16个元素的内存空间
    let _ = vec!["hello", "world"]; // 直接用vec!宏的方式创建Vec，类型为&str，长度为2
    // rust 会根据元素的类型自动推导Vec的类型
    let mut v = Vec::new(); // 这里可以先不指定类型
    v.push("str"); // 在这里，编译器会通过元素的类型确定 v 的类型，为 Vec<&str>
    // v.push(1);              // 这里会报错，因为 v 的类型已经确定为 Vec<&str>，不能再push i32类型的元素
    // 也可以显式指定类型，即调用具体泛型的new方法
    let _ = Vec::<String>::new();
    // 1.2 访问、修改元素
    // 1.2.1 访问元素
    // 直接索引访问
    println!("{:?}", v[0]); // 访问第一个元素
    // println!("{:?}", v[1]); // 访问第二个元素，访问不到，会报运行时错误而panic。因为Vec长度不定，所以编译器无法检查越界
    // 使用get方法访问，会多一次校验，性能会略差
    println!("{:?}", v.get(0)); // 输出 Some("str")
    println!("{:?}", v.get(1)); // 输出 None
    // 1.2.2 修改元素
    v[0] = "str2"; // 修改第一个元素
    println!("{:?}", v[0]); // 输出 "str2"
    // 1.3 添加、删除元素
    // 1.3.1 添加元素
    v.push("str3"); // 添加一个元素
    v.push("str4"); // 添加一个元素
    v.push("str5"); // 添加一个元素
    v.push("str6"); // 添加一个元素
    println!("{:?}", v); // 输出 ["str2", "str3", "str4", "str5", "str6"]
    // 1.3.2 删除元素
    v.pop(); // ① 删除最后一个元素
    println!("{:?}", v); // 输出 ["str2", "str3", "str4", "str5"]
    v.remove(0); // ② 除指定索引的元素
    println!("{:?}", v); // 输出 ["str3", "str4", "str5"]
    v.swap_remove(0); // ③ 删除指定索引的元素，并将最后一个元素移动到该位置
    println!("{:?}", v); // 输出 ["str5", "str4"]
    // 1.4 遍历元素 ⭐️
    // 1.4.1 直接遍历会消耗所有权
    // for i in v {
    //     println!("{}", i);
    // }
    // println!("{:?}", v); // 编译器会报错，因为v已经被消耗了，这里拿不到所有权
    // 1.4.2 借用遍历不会消耗所有权
    for i in &v {
        println!("{}", i);
    }
    println!("{:?}", v); // 输出 ["str5", "str4"]
    // 1.4.3 可变借用遍历可以修改元素
    for i in &mut v {
        println!("before\t{}", i);
        *i = "str6";
        println!("after\t{}", i);
    }
    println!("{:?}", v); // 输出 ["str6", "str6"]

    // 2. HashMap<K, V>
    // 不同于Vec<T>，HashMap需要手动导入
    // 2.1 初始化
    // let _ = HashMap::new(); // 会报错，没有泛型参数，必须要指定泛型参数
    let _: HashMap<String, i64> = HashMap::new(); // 初始化一个空的HashMap，类型为String，i64，长度为0，即不进行内存分配
    let _: HashMap<String, i64> = HashMap::with_capacity(16); // 初始化一个空的HashMap，类型为String，i64，提前分配好长度为16个元素的内存空间
    // HashMap没有官方实现的，类似vec!一样的宏。不过 crates.io 上有类似的实现HashMap没有官方实现的，类似vec!一样的宏。不过 crates.io 上有类似的实现
    // 2.2 添加、删除元素
    let mut m: HashMap<String, i64> = HashMap::new();
    // 2.2.1 添加元素
    m.insert("key1".to_string(), 1); // 添加一个元素
    m.insert("key2".to_string(), 2); // 添加一个元素
    m.insert("key3".to_string(), 3); // 添加一个元素
    println!("{:?}", m); // 输出 {"key1": 1, "key2": 2, "key3": 3}
    // 2.2.2 删除元素
    m.remove("key1"); // 删除指定key的元素
    println!("{:?}", m); // 输出 {"key2": 2, "key3": 3}
    // m.clear(); // 清空HashMap中的所有元素
    // println!("{:?}", m); // 输出 {}
    // 2.3 访问、修改元素
    // 2.3.1 访问元素
    // 通过key访问，会返回一个Option<T>类型的值，Some(T)表示key存在，None表示key不存在
    println!("{:?}", m.get("key2")); // 输出 Some(2)
    println!("{:?}", m.get("key4")); // 输出 None
    /** ⭐️
     * .get的输入为&Q，且需要满足 K: Borrow<Q>以及Q: Hash + Eq。
     * 上边的例子中，.get的入参为&str（即&Q类型)，K类型为 HashMap 的 key，即 String。
     * Borrow是一个 trait，K: Borrow<Q>限制了Q一定要为一个可以从 String 类型通过 Borrow 这个 trait 转换过去的类型，str满足要求。
     * Q: Hash + Eq 还限制了 Q 一定要满足可哈希以及可判断是否相等，str也满足要求。
     */
    // 2.3.2 修改元素
    // 当.insert的 key 存在时，会更新 value
    m.insert("key2".to_string(), 22); // 修改指定key的元素
    println!("{:?}", m); // 输出 {"key2": 22, "key3": 3}
    /**
     * .entry 方法会返回一个 Entry 枚举类型的引用，该枚举类型有两个变体：Occupied 和 Vacant
     * Occupied 表示该 key 已经存在，Vacant 表示该 key 不存在
     * .or_insert 方法会在 Vacant 变体下插入一个新的 key-value 对，返回一个可变引用
     */
    m.entry("key4".to_string()).or_insert(4); // 添加一个元素
    println!("{:?}", m); // 输出 {"key2": 22, "key3": 3, "key4": 4}
    /**
     * .and_modify 方法会在 Occupied 变体下执行一个闭包，闭包的参数是一个可变引用
     */
    m.entry("key4".to_string()).and_modify(|v| {
        if *v == 4 {
            *v = 44
        }
    });
    println!("{:?}", m); // 输出 {"key2": 22, "key3": 3, "key4": 44}
    // 2.4 遍历元素
    // 可以使用for循环遍历HashMap中的元素，需要注意的是，HashMap中的元素是无序的，如果需要有序的遍历，需要使用TreeMap
    for (k, v) in &m {
        // 为了不消耗掉所有权，用&map或者&mut map去遍历。相应的，遍历元素的类型也会变成引用
        println!("key: {}, value: {}", k, v);
    }
}
