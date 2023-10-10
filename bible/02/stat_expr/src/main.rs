fn main() {
    println!("Hello, world!");
}

fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 5; // 语句
    x + y // 表达式

    // 对于 Rust 语言而言，这种基于语句（statement）和表达式（expression）的方式是非常重要的，
    // 你需要能明确的区分这两个概念, 但是对于很多其它语言而言，这两个往往无需区分。
    // 基于表达式是函数式语言的重要特征，表达式总要返回值。
}

fn expr() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
