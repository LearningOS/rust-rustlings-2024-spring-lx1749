// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.



struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match &y {// 此处不会移动
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),//这儿移动了，Point { x: 100, y: 200 }移动给了p
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
