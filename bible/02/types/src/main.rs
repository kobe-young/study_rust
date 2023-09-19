fn main() {
    integer();
}

fn integer() {
    let x = 1; // 自动推导位i32
    let y : i32 = 2; // 指定类型
    let z = 3i32; // 指定类型。
    println!("x is {}, y is {}, z is {}", x, y, z);

    // 整型字面值。
    let a = 1u32; // 无符号整数
    let b = 10_000; // 下划线分隔符
    let c = 0xff; // 十六进制
    let d = 0o700; // 八进制
    let e = 0b100_0000; // 二进制
    println!("a is {}, b is {}, c is {}, d is {}, e is {}", a, b, c, d, e);

    // 溢出测试
    let f = 255u8;
    let g: u8 = f + 1;
    println!("f is {}, g is {}", f, g);
    // debug/release 结果不同
    /* 
    $ cargo run
    x is 1, y is 2, z is 3
    a is 1, b is 10000, c is 255, d is 448, e is 64
    thread 'main' panicked at 'attempt to add with overflow', src/main.rs:21:17
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    */

    /*
    $ cargo run --release
    x is 1, y is 2, z is 3
    a is 1, b is 10000, c is 255, d is 448, e is 64
    f is 255, g is 0
    */
}