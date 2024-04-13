// strings2.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    //比较运算符会追踪两边的所有引用并对最终目标a,b进行比较
    //如(a,b为相同类型的值):
    //&a == &b <=> a == b      &&a == &&b <=> a == b
    //但比较运算符的操作数必须具有相同的类型
    //若 &a == &&b 则会出现错误 
    //综上，对两个引用的比较实际比较的是引用目标
    //若确实希望比较两个引用是否指向同一块地址，可使用std::ptr::eq(&a, &b)
    attempt == "green" || attempt == "blue" || attempt == "red"
}
