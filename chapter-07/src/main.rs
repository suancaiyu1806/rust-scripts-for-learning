fn main() {
    // 1. 结构体定义
    struct Point {
        x: f64,
        y: f64,
    }

    // 2. 结构体实例化
    let p = Point { x: 0.0, y: 0.0 }; // 每个字段都需要初始化
    let x = 1.2;
    let y = 2.3;
    let p = Point { x, y }; // 字段名和变量名相同时可以简写，同JS

    // 3. 结构体的访问
    println!(
        "The coordinates of the point are ({}, {})",
        point.x, point.y
    );

    // 4. 更新结构体字段值
    /**
     * 默认情况下，结构体实例内的所有字段都是不可变的。如果想要更新字段的值，结构体实例必须是可变的。
     * Rust只能够将整个实例标记为可变的，而不支持将结构体内部某个字段标记成可变的
     */
    let mut point = Point { x: 3.0, y: 4.0 };
    point.x = 5.0;
    point.y = 6.0;

    // 5. 通过已有结构体创建
    struct User {
        username: String,
        email: String,
        age: u8,
    }
    let user1 = User {
        username: String::from("Alice"),
        email: String::from("alice@example.com"),
        age: 18,
    };
    let user2 = User {
        email: String::from("alice2@example.com"),
        ..user1 // 支持类似JS析构的写法
    }; // age 为 u8，会自动 copy，user1.age仍可用；username 为 String，会发生所有权转移，user1.username 不可用
}
