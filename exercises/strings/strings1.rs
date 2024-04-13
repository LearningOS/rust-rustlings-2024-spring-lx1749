// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    //字符串字面量是预分配文本的 &str,该文本存储在只读内存区
    //本函数应返回owned string，可采用.to_string()方法，该方法生成一个新的owned string,具有和&str相同的文本串
    "blue".to_string()
}
