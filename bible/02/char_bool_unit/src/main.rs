fn main() {
    char();

    bool();
}

fn char() {
    
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';

    // 由于 Unicode 都是 4 个字节编码，因此字符类型也是占用 4 个字节：
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小",std::mem::size_of_val(&x));
    
}

fn bool() {
    // 布尔类型有两个可能的值：true 和 false，布尔值占用内存的大小为 1 个字节
    let t = true;

    let f: bool = false; // 使用类型标注,显式指定f的类型

    if f {
        println!("这是段毫无意义的代码");
    }
}