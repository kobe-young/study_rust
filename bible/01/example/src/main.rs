fn main() {

    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    // 字符串分行，返回一个迭代器?
    let lines = penguin_data.lines();
    print_type_of(&lines);
    //  
    for (i, line) in lines.enumerate() {
        if i == 0 || line.trim().len() == 0 {
            continue;
        }
        // 分割字符串，然后trim掉空格。
        let fields:Vec<_> = line.split(",").map(|field| field.trim()).collect();

        print_type_of(&fields); // alloc::vec::Vec<&str>
        print_type_of(&fields[0]); // &str

        if cfg!(debug_assertions) { // debug模式才会命中。
            eprintln!("debug: {:?} -> {:?}", line, fields); // 打印到标准错误流
        }
        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
}


// 判断变量类型
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}