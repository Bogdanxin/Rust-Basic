// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let mut data = "Rust is great!".to_string();
    // 因为下一个函数也要对 data 做借用，
    // 所以 get_char 方法不能做 move 操作，只能使用借用的方式
    get_char(&data);
    // 因为函数要对 data 做改动，所以需要传入参数以及函数声明都要设置为 mut
    string_uppercase(data);
}

// Should not take ownership
// 如果是借用的话，对应的函数也要有借用声明，rust 在后面会自己解引用
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
// 因为要获取 data 所有权，所以data 属性声明不能是引用&
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}