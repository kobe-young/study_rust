fn main() {
    println!("Hello, world!");
    owner();
    fn_owner();
}

fn owner() {
    let s1 = String::from("hello, world");
    let s2 = s1;
    // 这里需要注意：
    // 1. String的内容存在堆上，而字符串的长度，堆的指针等信息是存在栈上了。
    // 2. 其中堆上的信息并不会拷贝，而栈上的东西，在经过s2 = s1后，就将所有权交给s2了，在访问s1将报错。
    // 3. s2在离开作用域后自动释放，包括栈和堆上的内存。
    // 4. 基本类型，和这个有区别，基本类型直接复制，即在栈上直接会复制出来一个。
    // 5. 如果需要s2不从s1转移所有权，这里可以使用clone()函数。

    let x = 10;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // 下面访问会报错。所有权转移了
    //println!("s1 = {}, s2 = {}", s1, s2);

    // clone将深度克隆，这样保证s2和s3独立，不存在转移所有权。但是存在性能损失。
    let s3 = s2.clone();
    println!("s2 = {}, s2 = {}", s3, s2);

    // a是字符串的引用，可以理解为引用，并不涉及到所有权转移。
    let a: &str = "hello, world";
    let b = a;
    println!("a = {}, b = {}", a, b);
}

fn copy() {
    /*
    那么什么类型是可 Copy 的呢？可以查看给定类型的文档来确认，不过作为一个通用的规则： 任何基本类型的组合可以 Copy ，不需要分配内存或某种形式资源的类型是可以 Copy 的。如下是一些 Copy 的类型：

    1. 所有整数类型，比如 u32
    2. 布尔类型，bool，它的值是 true 和 false
    3. 所有浮点数类型，比如 f64
    4. 字符类型，char
    5. 元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是
    6. 不可变引用 &T ，例如转移所有权中的最后一个例子，但是注意: 可变引用 &mut T 是不可以 Copy的
     */
}

fn fn_owner() {
    let s = String::from("hello, world");
    take_ownership(s); // 注意这里所有权已经转移了。作为函数参数，转移到函数内部了，然后函数执行完毕后，就释放了内存了。后面就不能在使用了。

    let x = 10;
    make_copy(x); // 遵循copy的语义，基本类型，会拷贝一份，所以并不存在所有权转移。

    //println!("s = {}, x = {}", s, x);
    println!("x = {}", x);

    // 从函数获取所有权
    let a = give_ownership();
    let b = String::from("hello, world");
    let c = take_and_give_back(b);

    // 这里会报错，因为，b的所有权经过多次转移，现在给了c
    //println!("a = {}, b = {}, c = {}", a, b, c);
    println!("a = {}, b = nil, c = {}", a, c);

}

fn take_ownership(a : String) {
    println!("{}", a);
}

fn make_copy(x : i32) {
    println!("{}", x);
}

fn give_ownership() -> String {
    let s = String::from("hello");
    s
}

fn take_and_give_back(s : String) -> String {
    s
}

