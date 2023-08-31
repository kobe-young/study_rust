fn main() {
    println!("Hello, world!");
    greeting();
}

// 函数可以先使用，后声明。
fn greeting() {
    // rust原生支持utf-8字符串。
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    let regions = [southern_germany, chinese, english];
    // rust集合类型，不能直接迭代，需要调用其iter()方法。
    for region in regions.iter() {
        // !是宏，可以先理解为一个特殊的函数。占位符不想c的%d等，而是用{}。
        println!("{}", region)
    }
}