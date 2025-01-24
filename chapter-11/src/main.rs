fn main() {
    // 错误处理，Rust 的错误处理完全依靠 Rust 的类型系统
    // 1. Option<T>
    // Rust 不包含空类型 null 这样的概念，所以 Rust 也没有空指针异常的概念。
    // 对于空值的处理，Rust 使用 Option<T> 这一枚举来表示。Option<T> 类型有两个值：Some(T) 和 None。
    // Some(T) 表示值存在，None 表示值为空的情况。Option 通常用在结构体的字段上用来表示这个字段值不是必须的。
    // pub enum Option<T> {
    //     None,
    //     Some(T),
    // }
    fn divide(numerator: f64, denominator: f64) -> Option<f64> {
        if denominator == 0.0 {
            None
        } else {
            Some(numerator / denominator)
        }
    }
    assert_eq!(divide(6.0, 3.0), Some(2.0));
    assert_eq!(divide(6.0, 0.0), None);

    // 2. Result<T, E>
    // Result<T, E> 类型表示一个可能会失败的操作的结果。Result<T, E> 类型有两个值：Ok(T) 和 Err(E)。
    // Ok(T) 表示操作成功，Err(E) 表示操作失败。Result 通常用在函数的返回值上用来表示这个函数可能会失败。
    // 其中E一般是实现了Error trait的类型。
    // pub enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    #[derive(Debug)]
    enum Version {
        Version1,
        Version2,
    }
    fn parse_version(header: &[u8]) -> Result<Version, &'static str> {
        match header.get(0) {
            None => Err("invalid header length"),
            Some(&1) => Ok(Version::Version1),
            Some(&2) => Ok(Version::Version2),
            Some(_) => Err("invalid version"),
        }
    }
    let version = parse_version(&[1, 2, 3, 4]);
    match version {
        Ok(v) => println!("working with version: {v:?}"),
        Err(e) => println!("error parsing header: {e:?}"),
    }

    // 3. Error
    
}
