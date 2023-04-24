// fn main() {
//     // create empty i32 vector
//     let mut v1: Vec<i32> = Vec::new();
//     v1.push(1);
//     v1.push(2);

//     // create i32 vector with initial values
//     let v2 = vec![1,2,3,4,5];

//     println!("{:?} {:?}", v1, v2);

//     // (1) 인덱스 사용하여 읽기
//     let n1 = &v1[1];
//     println!("&v1[1]: {}", n1);

//     // (2) get() 메서드로 읽기
//     let elem1 = v1.get(1);
//     println!("&v1.get(1): {:?}", elem1); // 리턴 타입 : Option<T>
// }

fn main() {
    let mut v = vec![1,2,3];

    for n in &mut v {
        *n = *n + 1;
        println!("{}", n);
    }
}