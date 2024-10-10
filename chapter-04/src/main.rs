use std::io;

fn main() {
    // 1. 变量、引用、类型
    println!("Please input a number: ");
    let mut input = String::new(); // let mut 代表声明的是一个可变的变量，如果不加 mut 默认声明的就是一个不可变的变量
    io::stdin()
        .read_line(&mut input) // 这里我们拿到了 input 这个变量的可变引用，也因此在声明的时候需要声明称可变变量，关于引用我们会在后续章节中详细介绍
        .expect("Failed to read input!");
    println!("Your raw input is: {:?}.", input);
    // 这里的 number 是不可变的，类型我们指定为了 i64，在这里指定的原因是为了告诉编译器 parse 需要解析成什么类型，因为 String 可以 parse 成多个类型，所以在这里 Rust 编译器无法自行推断出来
    let number: i64 = input.trim().parse().expect("Input is not a number!");
    println!("Your input is: {}.", number);

    // 2. 标量类型
    let _a: i8 = 1; // <==> let _a = 1_i8; <==> let _a = 1i8; 数字字面量均允许使用类型后缀来指定类型，数字字面量还可以使用 _ 作为可视分隔符以方便读数
    let _a: i16 = 1;
    let _a: i32 = 1;
    let _a: i64 = 1;
    let _a: i128 = 1;
    let _a: isize = 1;
    let _a: u8 = 1;
    let _a: u16 = 1;
    let _a: u32 = 1;
    let _a: u64 = 1;
    let _a: u128 = 1;
    let _a: usize = 1;
    let _a: f32 = 1.0;
    let _a: f64 = 1.0;
    let _a: bool = true;
    let _a: char = 'a'; // 字符（char），一个字符占用 4 个字节

    let a = (98_222, 0xff, 0o77, 0b1111_0000, b'A'); // 数字字面量，支持多种进制，元组内分别为十进制、十六进制、八进制、二进制、字符(仅限于u8，b'A'表示A的ASCII码，<==> 65u8)
    println!("a: {:?}", a); // (98222, 255, 63, 240, 65)

    // 3. 元组 - 复合类型
    let _a: (i32, f64, u8) = (1, 1.0, 1); // 元组（tuple），一个元组可以有任意多个成员，每个成员的类型可以不同
    let a = (1, "a");
    println!("a: {:?}", a);
    let (a, b) = a; // 解构，且支持复用变量名
    println!("a: {:?}, b: {:}", a, b);

    // 4. 数组 - 复合类型
    let _a = [1, 2, 3, 4, 5]; // 数组（array），一个数组必须有相同类型的元素，且长度固定
    let a = [1; 5]; // 数组的初始化，这里我们指定了数组的长度为 5，且所有元素的值都为 1
    let [a, b, c, d, e] = a; // 解构，且支持复用变量名
    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}, e: {:?}", a, b, c, d, e);
}
