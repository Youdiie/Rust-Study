// mod circle;

// fn main() {
//     let a = circle::area(5.0);
//     let p = circle::perimeter(5.0);
//     println!("{}, {}", a, p);
// }

// mod circle {
//     const PI: f64 = 3.14;

//     pub fn perimeter(r: f64) -> f64 {
//         2.0 * PI * r
//     }

//     pub fn area(r: f64) -> f64 {
//         PI * r * r
//     }
// }

// fn main() {
//     let a = circle::area(5.0);
//     let p = circle::perimeter(5.0);
//     println!("{}, {}",a,p);
// }

pub mod usa {
    pub mod washington {
        pub mod seattle {
            pub fn trip() {
                println!("seattle trip");
            }
        }
    }
}

fn main() {
    // 긴 경로 사용
    usa::washington::seattle::trip();
}

use usa::washington::seattle;

fn main() {
    // 간결한 표현
    seattle::trip();
}

use usa::washington::seattle as s;

fn main() {
    // 간결한 표현
    s::trip();
}