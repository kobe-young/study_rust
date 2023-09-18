fn main() {
    //println!("Hello, world!");
    // 不可变类型
    let a = 1; // a自动推导为i32类型，并且为不可变类型
    let b: i32 = 2; // b为i32类型(显示声明)，并且为不可变类型
    let c = 3i32; // c为i32类型(显示声明)，并且为不可变类型
    let d = 4_i32; // d为i32类型(显示声明)，并且为不可变类型

    // 可变类型
    let mut e = 5; // e自动推导为i32类型，并且为可变类型
    let mut f: i32 = 7; // f为i32类型(显示声明)，并且为可变类型
    let mut g = 6i32; // g为i32类型(显示声明)，并且为可变类型
    let mut h = 8_i32; // h为i32类型(显示声明)，并且为可变类型


    let i = add(add(a, b), add(e, f));
    println!("i = {}", i);
}

fn add(i: i32, j: i32) -> i32 {
    //i + j // rust比较特殊，这个直接返回作为返回值。
    // 或者这样写吗？
    return i + j;
}
