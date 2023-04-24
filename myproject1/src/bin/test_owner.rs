// fn main() {
//     let s1 = String::from("hello"); // ①
//     let s2 = s1;  // ② 객체의 소유권이 s2로 이동
//     let (s2, len) = calculate_length(s2);
//     println!("The length of '{}' is {}.", s2, len); // ④
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // ③
//     (s, length);
// }

// main() {
//     int sarr[50];
//     int *darr;

//     darr = (int*)malloc(sizeof(int)*50);

//     free(darr);
// }

use std::convert::TryFrom;

fn analyze_slice(slice: &[i32]) {
    println!("[0] = {}, 길이: {}", slice[0], slice.len());
}

fn main() {
    // 고정크기 배열 (let xs:[i32; 5]의 생략)
    let xs = [1,2,3,4,5];

    // 모든 값을 0으로 초기화
    let mut ys: [i32; 500] = [0;500];

    // 배열의 크기 출력
    println!("배열의 크기: {}", xs.len());

    // 배열을 대여 - 함수 호출
    analyze_slice(&xs);

    for x in 0..500 {
        ys[x] = i32::try_from(x).unwrap();
    }
    // 배열의 슬라이스를 대여
    analyze_slice(&ys[5 .. 8]);
}