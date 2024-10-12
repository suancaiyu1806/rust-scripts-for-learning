/**
 * rust中函数命名格式一般为snake_case
 * 没有返回值的函数会隐式返回单元类型()
 */
fn say_hello(name: String) {
    println!("Hello, {}", name);
}

fn pass_exam(score: u32) -> bool {
    if score < 60 {
        return false; // 可以调用return返回
    }
    true // 默认返回函数体最后一个表达式的值（有多个会报错）
}

/**
 * 表达式、语句
 * 函数、控制流
 */
fn main() {
    // 1. 语句与表达式
    // 语句通常表示一个动作没有返回值，表达式会求值因此有返回值
    let name = "Alice".to_string(); // 语句
    say_hello(name.clone()); // 表达式

    // 常见的表达式有：
    let _a = 1 + 1; // 1 + 1 为算术表达式
    let _b = say_hello("Alice".to_string()); // say_hello() 为函数表达式
    let _c = {
        let a = 1;
        let b = 2;
        a + b // 块表达式最后一行会被当作表达式返回，不能加分号，添加分号会变成语句
    }; // 块表达式，作为表达式结尾必须加分号
    let _d = if true { 1 } else { 2 };
    let _e = loop {
        break 1; // 这里的break相当于return
    }; // 作为表达式结尾必须加分号
    let _f = match 1 {
        // 模式匹配表达式
        1 => "one",
        2 => "two",
        _ => "other",
    }; // 作为表达式结尾必须加分号

    // 2. 函数
    say_hello(name.clone());
    pass_exam(20);

    // 3. 控制流
    // 控制流不同分支的返回值必须是同种类型，否则会报错
    // let _g = if pass_exam(20) { "1" } else { 2 };  // expected `&str`, found integer

    /*
     * loop 会创建一个无限循环，直到遇到 break 语句，loop语句支持 break 语句后接返回值
     * loop 会使代码变的冗长，尽量少使用
     */
    let mut counter = 0;
    let _e = loop {
        counter += 1;
        if counter < 5 {
            continue;
        }
        println!("Hello, world!");
        if counter >= 5 {
            break counter;
        }
    };
    /*
     * for 循环支持对range、数组、字符串、字符串切片、迭代器进行遍历
     * 1..=3 <==> [1,3]  1..3 <==> [1,3)
     */
    for x in 1..=3 {
        println!("x: {}", x);
    }

    // 循环labels
    let x = 1;
    let z = 'outer: loop {
        let mut y = 1;
        'inner: loop {
            if y == 3 {
                y += 1;
                continue 'inner; // Skips to the next iteration of the 'inner loop
            }

            println!("x: {}, y: {}", x, y);
            y += 1;
            if y > 5 {
                break 'outer y; // Breaks out of the 'outer loop
            }
        }
    };
    println!("z: {}", z);
}
