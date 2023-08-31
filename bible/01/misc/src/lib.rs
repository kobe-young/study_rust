

// 判断变量类型
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
