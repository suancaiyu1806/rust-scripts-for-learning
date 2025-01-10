mod apple;
mod pear;
mod orange;
use rand::Rng;

fn main() {
    // 1. 新建项目
    // cargo new hello-world
    // cargo init // 项目入库src/main.rs
    // cargo init --lib // 项目入库src/lib.rs，库类型的项目
    // 2. 包管理
    // 2.1 安装依赖
    // 方式一：命令行运行cargo add rand
    // 方式二：编辑Cargo.toml文件，添加依赖，在[dependencies]下添加 rand = "0.8.5"
    // 2.2 引入依赖
    // use rand::Rng;
    let gen = rand::thread_rng().gen::<i64>() % 2;
    if gen == 0 {
        println!("Hello");
    } else {
        println!("World");
    }
    // 3. 模块管理
    // 模块构成的树是以包的根节点为根节点构成的树，没有被根节点包含的模块是无法被访问的，IDE也无法识别。
    // 3.1 文件模块
    // 子模块文件内通过pub声明可以被外部访问的模块，pub可以修饰模块、函数、结构体、枚举等；父模块通过mod引入子模块
    apple::eat_apple();
    pear::eat_pear();
    // 3.2 目录模块
    // 目录模块是将目录下的文件作为子模块导出，目录下的文件需要通过pub声明可以被外部访问的模块，并且需要在mod.rs文件中导出；父模块通过mod引入子模块
    orange::eat::eat_orange();
    // 3.3 文件内模块
    // 文件内支持声明模块，支持嵌套
    mod fruit {
        pub mod apple {
            pub fn eat_apple() {
                println!("I eat apple");
            }
        }
        
        pub mod pear {
            fn eat_pear() {
                println!("I eat pear");
            }       
        }
        
        mod orange {
            pub fn eat_orange() {
                println!("I eat orange");
            }               
        }
    }
    fruit::apple::eat_apple(); // 各级模块都要为pub
    // fruit::pear::eat_pear(); // 报错：函数eat_pear()在模块fruit::pear中不可见
    // fruit::orange::eat_orange(); // 报错 mod orange 是私有的
}
