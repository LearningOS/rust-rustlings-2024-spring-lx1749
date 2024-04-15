// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.



#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        //assert_eq!()要求两个参数，两参数若类型不同会在编译期报错
        //类型相同但值不同会在运行时引发panic
        //若类型为引用则判断值是否相同时会自动解引用
        
        assert_eq!(1, 1);
    }
}
