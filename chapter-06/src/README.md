## 基础知识

> 参考：[What Is Ownership?](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html)

### 1. 常用内存管理

- 垃圾回收(GC, Garbage Collection)，在程序运行时找到所有不再使用的内存并回收，eg: JavaScript(标记清除、引用计数等)；
- 显式的申请和释放内存，eg: C；
- 所有权系统，所有权系统规则会在编译时就被检查，任意规则不满足，程序将无法编译通过。由于是编译时检查的规则，从而不会拖慢程序运行时的性能，eg: Rust。

### 2. 堆/栈

堆和栈都可以在程序运行时给程序提供内存空间：

- 栈：先进后出，大小固定，像一叠盘子。
- 堆：大小不固定，指针寻址，像一个仓库。

在性能的赛跑中，栈就像速度迅猛的跑车，堆则更像一辆载重卡车：

- 栈的读写速度快，因为一切都井井有条，就像把盘子放在最上面那样简单。
- 而堆要先找到合适的存放空间，读取时还要通过标签（指针）找到物品，就像在杂货仓库里寻找一个小小的零件一样，可能需要多花点时间。

### 3. String 类型

**&str**

&str 为字符串字面值，它是不可变的，是硬编码到程序中的，其值在运行时保存在 Read Only Memory 中，引用保存在栈上，两个字面值相同的 &str 变量，其地址一致。

**String**

String 为可变的，其值保存在堆中，引用保存在栈上，是一个胖指针：`fat pointer`，结构为：

- pointer: heap中值的内存地址
- length: 当前值的长度、当前元素个数
- capacity: 当前缓冲区的容量，可以容纳元素的个数，当前字符串的长度超过当前分配的capacity会重新分配内存，会将当前字符串拷贝到新分配的内存中。

## 所有权

### 1. 所有权规则

- 每个值都有一个所有者；
- 一个值同时只能有一个所有者；
- 当所有者离开作用域范围，这个值将被丢弃。

```rust
fn main() {
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
}
```

### 2. 所有权转移

对于类似 i32 这样的简单类型，赋值的时候 Rust 会自动进行拷贝(深拷贝，因为这类简单类型都是存储在栈上的，对其执行拷贝动作开销很低)。而对于 String 这样的分配到堆上的复杂类型，发生的却是所有权的转移，而不是拷贝，如下面的两个例子。

```rust
let x = 5;
let y = x;
println!("{}{}", x, y); // 5, 5

let s1 = String::from("hello");
let s2 = s1;
println!("{}{}", s1, s2); // 编译报错：cannot move out of `s1` because it is borrowed
```

<img src="https://doc.rust-lang.org/stable/book/img/trpl04-01.svg" width="300px">

一个 String 类型变量由指针和内容两部分组成：
- 指针由三部分组成
    - 一个指针指向实际存储String内容的位置
    - 一个len表示长度
    - 一个capacity表示容量
- 内容存储具体的字符串内容

指针和内容是分开的，指针存储在栈上，内容存储在堆上。

当将 s1 的值赋给 s2 时，仅左半部分的 String 结构会被拷贝(即浅拷贝)。这时候内存中 s1 和 s2 是这样的：

<img src="https://doc.rust-lang.org/stable/book/img/trpl04-02.svg" width="300px">

根据上面说的所有权原则第三条，当所有者离开作用域范围，值将被丢弃(drop)。上图中 s1 和 s2 都指向同一个值，当他们离开作用域时，都会去执行drop的动作。这就产生了双重释放(double free)这一内存安全问题。

因此Rust会进行所有权的转移，在 `let s2 = s1;` 执行完之后，所有权会从 s1 转移给 s2，s1 不再有效。这样一来，上述 String 值只会在 s2 离开作用域的时候被释放。所有权转移之后内存的示意图如下，s1 被标记为失效。

<img src="https://doc.rust-lang.org/stable/book/img/trpl04-04.svg" width="300px">

如果确实想要实现深拷贝，可以使用 clone 方法实现：

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

此时内存示意图如下：

<img src="https://doc.rust-lang.org/stable/book/img/trpl04-03.svg" width="300px">

### 附. 函数传参与返回值

传参给函数以及函数返回值时，也会经历所有权转移的过程，如下例所示：

```rust
fn takes_ownership(s: String) {
    println!("Received string: {}", s);
} // s 离开作用域，被丢弃

fn gives_ownership() -> String {
    String::from("hello")
} // 返回了 String 的所有权

fn main() {
    let s = String::from("hello");
    takes_ownership(s); // s 转移到了函数内，不再可用

    // s 不可用

    let s = gives_ownership(); // s 获得了返回值的所有权
}
```

## 引用与借用

借用是 Rust 中非常重要的概念，它允许你使用值而不获取其所有权。

### 1. 不可变引用

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

在给 s 传入 &s1 的时候，实际是创建了一个指向 s1 的引用，这个引用没有值的所有权。因此当函数体内的 s 超出作用域范围被丢弃时，不会导致 String 被丢弃。其在内存中示意图如下：

<img src="https://doc.rust-lang.org/stable/book/img/trpl04-05.svg" width="400px">

Rust 中声明的变量默认是不可变变量，默认的引用也是不可变的引用。创建引用的这个动作就被称为借用。

### 2. 可变引用

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("The updated string is: {}", s);
}

fn change(s: &mut String) {
    s.push_str(", world!");
}
```

这里将 s 声明为可变变量，然后通过 &mut 创建了一个可变引用。而 change 函数的参数 s 也声明为可变引用。

**注意：对于一个变量，同时只能存在一个可变引用或者多个不可变引用。** 可参考该示例：

```rust
fn main() {
    let mut s = String::from("hello");

    // 多个不可变引用是允许的
    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);

    // 一个可变引用也是允许的。
    let r3 = &mut s;
    println!("r3: {}", r3); // 这里编译时会有优化行为 Non-Lexical Lifetimes(NLL)，引用的作用域结束的位置不再是花括号的位置，而是最后一次使用的位置，因此不会报错
    // println!("r2: {}, r3: {}", r2, r3); // 这样就会报错，实际上变成了下面同时打印 r3 和 r4 的情况

    // 不允许同时存在可变引用和不可变引用，将导致编译错误
    let r4 = &s;
    println!("r3: {}, r4: {}", r3, r4); // 会报错
}
```

上面例子中涉及了 NLL(Non-Lexical Lifetimes) 优化：**引用的作用域结束的位置不再是花括号的位置，而是最后一次使用的位置**。

### 3. 悬垂引用

> Rust 编译器保证引用永远也不会变成悬垂状态，即永远也不会出现一个指针指向一个已经释放掉的内存地址。

悬垂引用指的是指针指向的是内存中一个已经被释放的地址，这在其他的一些有指针的语言中是很常见的错误。而 Rust 则可以在编译阶段就保证不会产生悬垂引用。也就是说，如果有一个引用指向某个数据，编译器能保证不会存在引用离开作用域之前，数据被释放的情况（会编译不通过）。

尝试实现如下的悬垂引用，会导致编译不通过（this function's return type contains a borrowed value, but there is no value for it to be borrowed from，该函数返回了一个借用的值，但是没有可以借用的来源）。

```rust
fn main() {
    let reference_to_nothing = dangle();
}
fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

## 切片

### 1. 字符串切片

### 2. 其他切片
