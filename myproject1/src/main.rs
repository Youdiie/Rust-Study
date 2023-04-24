// struct Person {
//     id: i32,
//     name: String,
//     active: bool
// }

// impl Person {
//     fn new(id: i32, name: String) -> Person {
//         Person{id, name, active:true}
//     }

//     fn display(&self) {
//         if self.active {
//             println!("{}: {}", self.id, self.name);
//         }
//         else {
//             println!("{}: inactive", self.id);
//         }
//     }
// }

// fn main() {
//     let p = Person::new(101, String::from("Tom"));
//     p.display();
// }
struct Point {
    x: f32,
    y: f32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}
