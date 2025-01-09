fn main() {
    // 1. 枚举
    enum Fruit {
        Apple,
        Pear,
    }
    // ::初始化
    let _ = Fruit::Apple;
    let _ = Fruit::Pear;
    // 枚举支持复杂类型，如嵌套枚举、结构体等
    enum Color {
        Red,
        Green,
    }
    struct Price(u64);
    enum FruitBox {
        Apple(Color),
        Pear(Color, Price),
    }
    // 也可以直接这样写
    // enum FruitBox {
    //     Apple(Color),
    //     Pear{color: Color, price: u64},
    // }
    let _ = FruitBox::Apple(Color::Red);
    let _ = FruitBox::Pear(Color::Green, Price(50));

    // 2. match 语句、if let + let else
    // match 语句，match 需要遍历所有的枚举值，否则是无法编译通过的；可以用_来匹配所有未列出的枚举值，类似于js中的default
    fn _eat_or_sell(fruit_box: FruitBox) {
        match fruit_box {
            FruitBox::Apple(_) => _eat_apple(),
            FruitBox::Pear(_, price) => _sell_pear(price),
        }
    }
    fn _eat_apple() {
        println!("eat apple");
    }
    fn _sell_pear(price: Price) {
        println!("sell pear for {}", price.0);
    }
    // if let 语句
    fn _eat_if_is_apple_2(fruit_box: FruitBox) {
        if let FruitBox::Apple(_) = fruit_box {
            _eat_apple();
        }
    }
    // let else 语句
    fn _eat_if_is_apple_3(fruit_box: FruitBox) {
        if let FruitBox::Apple(_) = fruit_box {
            _eat_apple();
        } else {
            println!("not apple");
        }
        println!("========");
        // 也可以if满足时不处理
        let FruitBox::Apple(_) = fruit_box else {
            return;
        };
        _eat_apple();
    }

    _eat_if_is_apple_3(FruitBox::Apple(Color::Red));
}
