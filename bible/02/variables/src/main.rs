fn main() {
    println!("Hello, world!");

    bad_variable();
    good_variable();

    no_use();

    destructor();
    destructor_by_pattern_match();

    const_variable();

    range_shadowing();
    range_shadowing2();
}

fn bad_variable() {
    //let x = 5;
    let mut x = 5;
    println!("the value of x is {}", x);
    x = 6; // error, compile error, const not changed. detail like: cannot assign twice to immutable variable
    println!("the value of x is {}", x);
}

fn good_variable() {
    let mut x = 5;
    println!("the value of x is {}", x);
    x = 6; // ok, because x is mutable.
    println!("the value of x is {}", x);
}

fn no_use() {
    let x = 10; // no use, will warning. detail: help: if this is intentional, prefix it with an underscore: `_x`
    let y = 11;
    println!("the value of the y is {}", y);
}

fn destructor() {
    let (a, mut b): (bool, bool) = (true, false);
    println!("a is {}, b is {}", a, b);

    b = true;
    assert_eq!(a, b);
}

struct Struct {
    e: i32,
}

fn destructor_by_pattern_match() {
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    [c,.., d, _] = [1, 2, 3, 4, 5];
    Struct{e,..} = Struct{e: 5};

    println!("a is {}", a);
    assert_eq!([a, b, c, d, e], [1, 2, 1, 4, 5]); // assert_eq!() 宏，用于判断是否相等，否则直接退出。
}

fn const_variable() {
    const MAX_POINTS: u32 = 100_000;
    println!("the value of MAX_POINTS is {}", MAX_POINTS);

    const max_points: i32 = 100_000; // rust suggest to upper case, if no, will warning: help: convert the identifier to upper case: `MAX_POINTS`
    println!("the value of max_points is {}", max_points);
}

fn range_shadowing() {
    let x = 5;
    // 覆盖原来的x。
    let x = x + 1;
    {
        // 覆盖外部的x。
        let x = x * 2;
        println!("the value of x inner scope is {}", x);
    }
    println!("the value of x outer scope is {}", x);
}

fn range_shadowing2() {
    let mut x = 5;
    // 覆盖原来的x。
    let mut x = x + 1; // 编译会出警告：help: remove this `mut`，个人理解为，后面的x，并不是新的x，而是原来的x?
    {
        // 覆盖外部的x。
        let mut x = x * 2; // 编译会出警告：help: remove this `mut`，个人理解为，后面的x，并不是新的x，而是原来的x?
        println!("the value of x inner scope is {}", x);
    }
    println!("the value of x outer scope is {}", x);
}