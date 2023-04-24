// struct Member {
//     fname: String,
//     lname: String,
//     age: u16,
//     active: bool
// }

// fn main() {
//     // 구조체 초기화
//     let mem1 = Member {
//         active: true,
//         fname: String::from("Tom"),
//         lname: String::from("Lee"),
//         age: 35
//     };

//     // 구조체 필드 읽기
//     println!("{}: {}", mem1.fname, mem1.active);
// }

// fn main() {
//     let mem = get_member("Tom".to_owned(), "Lee".to_owned(), 33);
// }

// fn get_member(fname: String, lname: String, age: u16) -> Member {
//     Member {
//         fname: fname, // 필드명: 파라미터명 지정 (원래 표현)
//         lname,        // 필드명만 지정해도 ok
//         age,
//         active: true  // 파라미터가 없을 땐 타입 넣어줘야함
//     }
// }

// struct Admin {
//     name: String,
//     group: String,
//     active: bool
// }

// fn main() {
//     let adm1 = Admin {
//         name: String::from("Tom").
//         group: String::from("IT"),
//         active: true
//     };

//     let adm2 = Admin{
//         name: String::from("Kim"),
//         ..adm1    // struct update syntax
//     };

//     println!("{}", adm2.group);
//     println!("{}", adm1.name);
//     //println!("{}", adm1.group); // 에러 -> 소유권이 넘어갔기 때문
// }

struct Person {
    id: i32,
    name: String,
    active: bool
}

impl Person {
    fn new(id: i32, name: String) -> Person {
        Person{id, name, active:true}
    }

    fn display(&self) {
        if self.active {
            println!("{}: {}", self.id, self.name);
        }
        else {
            println!("{}: inactive", self.id);
        }
    }
}

fn main() {
    let p = Person::new(101, String::from("Tom"));
    p.display();
}