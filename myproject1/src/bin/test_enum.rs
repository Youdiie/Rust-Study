// enum LogType {
//     None,
//     Info(String),
//     Warning(String),
//     Error {code:i32, msg: String, caller: String} // 구조체로 정의
// }

// enum Option<T> {
//     None,
//     Some(T),
// }

// fn main() {
//     // Option::None 지정
//     let no_index: Option<i32> = Option::None;
//     // Option::Some 지정
//     let index: Option<i32> = Option::Some(1);

//     // Option:: 생략 표현
//     let no_index: std::option::Option<i32> = None;
//     let index = Some(1);

//     println!("{:?}, {:?}", no_index, index);
// }

// enum Priority {
//     Low,
//     Medium,
//     High
// }

// fn priority_weight(priority: Priority) -> i32 {
//     match priority {
//         Priority::High => 100,
//         Priority::Medium => 10,
//         Priority::Low => {
//             println!("Default priority");
//             1
//         }
//     }
// }

// fn main() {
//     let weight = priority_weight(Priority::High);
//     println!("{}", weight);
// }

fn process(opt: Option<i32>) {
    match opt {
        Some(val) => println!("#{} was chosen", val),
        _ => ()
    }
}

fn process(opt: Option<i32>) {
    if let Some(val) = opt {
        println!("#{} was chosen", val);
    }
}