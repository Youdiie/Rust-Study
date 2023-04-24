use std::collections::HashMap;

// fn main() {
//     let mut map: HashMap<String, i32> = HashMap::new();

//     map.insert(String::from("Korea"), 5);

//     let germ = String::from("Germany");
//     map.insert(germ, 4); // germ의 소유권이 이동 -> 이후부턴 변수 germ을 쓸 수 없음

//     println!("{:?}", map);
// }

fn main() {
    let countries = vec![String::from("Korea"), String::from("Germany")];
    let scores = vec![5,4];
    let map: HashMap<_,_> = countries.into_iter().zip(scores.into_iter()).collect();
    println!("{:?}", map);
}