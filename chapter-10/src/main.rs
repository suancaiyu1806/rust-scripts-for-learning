fn main() {
    // 1. 动态数组 Vec<T>
    // 1.1 初始化
    // Vec的长度可以动态增长
    let _: Vec<i32> = Vec::new(); // 初始化一个空的Vec，类型为i32，长度为0，即不进行内存分配
    let _: Vec<i32> = Vec::with_capacity(16); // 初始化一个空的Vec，类型为i32，提前分配好长度为16个元素的内存空间
    let _ = vec!["hello", "world"]; // 直接用vec!宏的方式创建Vec，类型为&str，长度为2
    // rust 会根据元素的类型自动推导Vec的类型
    let mut v = Vec::new(); // 这里可以先不指定类型
    v.push("str");          // 在这里，编译器会通过元素的类型确定 v 的类型，为 Vec<&str>
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
}
