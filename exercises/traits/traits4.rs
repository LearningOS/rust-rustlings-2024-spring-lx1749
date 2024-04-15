// traits4.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits4` or use the `hint` watch subcommand for a
// hint.



pub trait Licensed {
    fn licensing_info(&self) -> String {
        "some information".to_string()
    }
}

struct SomeSoftware {}

struct OtherSoftware {}

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// YOU MAY ONLY CHANGE THE NEXT LINE
//softwaware的类型不能只填 dyn Licensed，必须用 & dyn Licensed 即特型对象
//因为函数内局部变量的大小在编译期必须是确定的，而 dyn Licensed 的大小不确定
//而特型对象的大小必为两个机器字，故使用特型对象......吗？
//由于函数调用处传入的是值而非指针，所以此处也不能用特型对象,应采用impl Licensed
//impl trait 语法是 trait bound 的语法糖
//(software:impl Licensed, software_two:impl Licensed)  <=>
//<T1: Licensed, T2: Licensed>(software: T1, software_two: T2)
//所以impl trait 本质上是泛型，不会产生运行期动态派发

fn compare_license_types(software:impl Licensed, software_two:impl Licensed) -> bool {
    software.licensing_info() == software_two.licensing_info()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(some_software, other_software));
    }

    #[test]
    fn compare_license_information_backwards() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(other_software, some_software));
    }
}
