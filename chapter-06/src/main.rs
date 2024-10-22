fn main() {
    // 1. 所有权规则
    // 规则 1：Rust中每个值都有一个所有者
    let s1 = String::from("hello"); // s1 是值 "hello" 的所有者

    {
        // 规则 2：一个值同时只能有一个所有者
        let s2 = s1; // 所有权从 s1 转移到 s2（s1 不再有效）
                     // println!("{}", s1); // 这会导致编译时错误，因为 s1 不再有效
        println!("{}", s2); // 这是允许的，因为 s2 现在拥有该值
    } // s2在此处超出范围

    // 规则 3：当所有者离开作用域范围，这个值将被丢弃
    // 由于 s2 超出范围，因此为值 "hello" 分配的内存在此处自动释放。

    // 2. 所有权转移
    let x = 5;
    let y = x;
    println!("{}{}", x, y); // 5, 5
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}{}", s1, s2); // 编译报错：cannot move out of `s1` because it is borrowed

    // 3. clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // 4. 不可变引用
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // 5. 可变引用
    let mut s = String::from("hello");
    change(&mut s);
    println!("The updated string is: {}", s);

    // 6. 可变引用与不可变引用存在规则
    let mut s = String::from("hello");
    // 多个不可变引用是允许的
    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);
    // 一个可变引用也是允许的。
    let r3 = &mut s;
    println!("r3: {}", r3);
    // 这里编译时会有优化行为 Non-Lexical Lifetimes(NLL)，引用的作用域结束的位置不再是花括号的位置，而是最后一次使用的位置，因此不会报错
    // println!("r2: {}, r3: {}", r2, r3); // 这样就会报错，实际上变成了下面同时打印 r3 和 r4 的情况
    // 不允许同时存在可变引用和不可变引用，将导致编译错误
    // let r4 = &s;
    // println!("r3: {}, r4: {}", r3, r4); // 会报错

    // 7. 悬垂引用
    // let _reference_to_nothing = dangle();

    // 8. 字符串切片
    let s = String::from("hello world");
    let hello = &s[0..5]; // [0, 5)
    let world = &s[6..11]; // [6, 11)
    println!("{}, {}", hello, world);

    // 9. 数组切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]); // 断言执行成功
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}

// fn dangle() -> &String {
//     let s = String::from("hello"); // 创建一个 String 类型的 s
//     &s // 返回s的引用，但 s 离开函数 dangle 后就离开了作用域，会被丢弃。此时返回的 s 的引用指向的为非法位置。Rust编译器检查到了这个问题，从而报错。
// }
