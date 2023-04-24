struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

// fn main() {
//     let pt1 = Point { x: 1, y: 1 };
//     let pt2 = Point { x: 2.0, y: 2.0 };
// }

fn main() {
    let p = Point { x: 2.0, y: 2.0 };
    println!("{}", p.get_x());
}