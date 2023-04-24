fn main() {
    // (1) String::new()
    let mut s1 = String::new();
    s1.push_str("initial");
    println!("{}", s1);

    // (2) String::from()
    let s2 = String::from("initial");
    println!("{}", s2);

    // (3) &str.to_string()
    let s: &str = "hello";
    let s3: String = s.to_string();
    println!("{}", s3);

    // (4) {Display}.to_string();
    let n: i32 = 123;
    let s4: String = n.to_string();
    println!("{}", s4);
}