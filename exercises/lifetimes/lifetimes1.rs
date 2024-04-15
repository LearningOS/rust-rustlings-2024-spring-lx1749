// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a
// hint.



//若不显式声明引用的生命周期，编译器将采用三条规则来推断，若推断不出则报错:
//(函数或方法的参数的生命周期被称为 输入生命周期，而返回值的生命周期被称为 输出生命周期)
//第一条规则是编译器为每一个引用参数都分配一个生命周期参数
//第二条规则是如果只有一个输入生命周期参数，那么它被赋予给所有输出生命周期参数
//第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self,
//那么所有输出生命周期参数被赋予 self 的生命周期
//此处推断不出，故必须显式写出,且
//当从函数返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配。
//否则意味着该引用指向函数内部创建的值，这会使得它成为一个悬垂指针
//(当然即便指定返回值生命周期与参数相同也不能指向函数内部创建的值)
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
