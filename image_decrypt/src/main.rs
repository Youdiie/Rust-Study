#[macro_use]
extern crate magic_crypt;

use magic_crypt::MagicCryptTrait;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::prelude::*;

static KEY: &str = "magickey";

#[derive(Serialize, Deserialize, Debug)]
struct Image {
    content: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct Gltf {
    content: String,
}

fn encrypt_image(image_file: &str) {
    // 이미지 파일을 열어 암호화 실행
    let mut file = File::open(image_file).unwrap();
    // let mut file = File::open("./src/image.png").unwrap();
    let mut image_bytes = Vec::new();
    file.read_to_end(&mut image_bytes).unwrap();

    let image_struct = Image {
        content: image_bytes,
    };
    println!("{:#?}", image_struct.content.len());

    let serialized_json = serde_json::to_string(&image_struct).unwrap();
    let client = reqwest::blocking::Client::new();
    let res = client
        .post("http://localhost:3000/encrypt_image_byte")
        .body(serialized_json) // 데이터 전송을 body에 담아서 보냄
        .header("Content-Type", "application/json") // 데이터 타입을 json으로 지정
        .send()
        .expect("Failed to send request");
    let res = res.text().unwrap();
    println!("{:#?}", res);
}

fn decrypt_image_byte<'a>(encrypted_file: &'a str) -> &'a str {
    let mut encrypted_file = File::open(encrypted_file).unwrap();
    let mut encrypted_image_bytes = Vec::new();
    encrypted_file
        .read_to_end(&mut encrypted_image_bytes)
        .unwrap();

    let mcrypt = new_magic_crypt!(KEY, 256); //Creates an instance of the magic crypt library/crate.
    let decrypted_image_bytes = mcrypt
        .decrypt_bytes_to_bytes(&encrypted_image_bytes)
        .unwrap();
    // println!("Decrypted String: {}", decrypted_image_bytes);
    let decrypted_file = "./src/decrypted_image.jpg";
    let mut output_file = File::create(decrypted_file).unwrap();
    output_file.write_all(&decrypted_image_bytes).unwrap();

    decrypted_file
}

fn encrypt_gltf(gltf_file: &str) {
    // gltf 파일을 열어 암호화 실행
    let json_string = fs::read_to_string(gltf_file).unwrap();

    let gltf_struct = Gltf {
        content: json_string,
    };

    let serialized_json = serde_json::to_string(&gltf_struct).unwrap();

    let client = reqwest::blocking::Client::new();
    let res = client
        .post("http://localhost:3000/encrypt_json")
        .body(serialized_json) // 데이터 전송을 body에 담아서 보냄
        .header("Content-Type", "application/json") // 데이터 타입을 json으로 지정
        .send()
        .expect("Failed to send request");
    let res = res.text().unwrap();
    println!("{:#?}", res);
}

fn decrypt_gltf(encrypted_file: &str) {
    let mut encrypted_file = File::open(encrypted_file).unwrap();
    let mut encrypted_string = Vec::new();
    encrypted_file.read_to_end(&mut encrypted_string).unwrap();

    let mcrypt = new_magic_crypt!(KEY, 256); //Creates an instance of the magic crypt library/crate.
    let decrypted_string = mcrypt.decrypt_bytes_to_bytes(&encrypted_string).unwrap();

    let decrypted_file = "./src/decrypted_gltf.gltf";
    let mut output_file = File::create(decrypted_file).unwrap();
    output_file.write_all(&decrypted_string).unwrap();
}

fn main() {
    encrypt_gltf("./src/untitled.gltf");
    // decrypt_gltf("./src/encrypted_gltf.gltf");
    // decrypt_image_byte("./src/encrypted_image.jpg");
    // encrypt_image("./src/1024.jpg");
}
