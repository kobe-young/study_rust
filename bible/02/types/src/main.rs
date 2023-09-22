fn main() {
    integer();
    
    //float();

    nan();

    op();

    op2();
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

fn float() {
    let x = 1.0; // default is f64
    let y : f32 = 2.0;
    println!("x is {}, y is {}", x, y);

    let a = 0.1;
    let b: f32 = 0.2;
    println!("a is {}, b is {}", a, b);

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());

    assert_eq!(abc.0 + abc.1, abc.2);
    assert_eq!(xyz.0 + xyz.1, xyz.2);
}

fn nan() {
    let x = -42.0_f32;
    let y = x.sqrt();
    
    //assert_eq!(y, y);

    let a: f32 = -1.0;
    let b = a.sqrt();
    if b.is_nan() {
        println!("{} is NaN", b);
    }
}

fn op() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let reminder = 43 % 5;

    println!("sum is {}, difference is {}, product is {}, quotient is {}, reminder is {}", sum, difference, product, quotient, reminder);
}

fn op2() {
    // 编译器会进行自动推导，给予twenty i32的类型
    let twenty = 20;
    // 类型标注
    let twenty_one: i32 = 21;
    // 通过类型后缀的方式进行类型标注：22是i32类型
    let twenty_two = 22i32;
  
    // 只有同样类型，才能运算
    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);
  
    // 对于较长的数字，可以用_进行分割，提升可读性
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));
  
    // 定义一个f32数组，其中42.0会自动被推导为f32类型
    let forty_twos = [
      42.0,
      42f32,
      42.0_f32,
    ];
  
    // 打印数组中第一个值，并控制小数位为2位
    println!("{:.2}", forty_twos[0]);
  }