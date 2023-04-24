// use aes::Aes256;
// use block_modes::block_padding::Pkcs7;
// use block_modes::{BlockMode, Cbc};
// use std::convert::TryInto;
// use std::fs;

// type Aes256Cbc = Cbc<Aes256, Pkcs7>;

// fn main() {
//     println!("hi!");
//     // let key = b"secretkey";
//     let secret_key = "7365637265746b6579";
//     let key_bytes = secret_key.as_bytes().try_into().unwrap();
//     let key = bytes::Bytes::from(key_bytes);
//     let iv = b"\xda\xe4\x87\xa7i\x8arbOC\xbd \x02\xbc*[";
//     let cipher = Aes256Cbc::new_from_slices(key, iv).unwrap();
//     // let cipher = Aes128Cbc::new_from_slices(&key, iv).unwrap();

//     let encrypted_image_bytes = fs::read("encrypted_image1024.jpg").unwrap();
//     let decrypted_image_bytes = cipher.decrypt_vec(&encrypted_image_bytes[..]).unwrap();

//     fs::write("decrypted_image1024.jpg", decrypted_image_bytes).unwrap();
// }

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use base64;
use crypto::aes::{cbc_decryptor, KeySize};
use crypto::blockmodes::NoPadding;
use crypto::buffer::{RefReadBuffer, RefWriteBuffer};

fn main() {
    println!("{}", Path::new("./src/encrypted_image1024.jpg").exists());
    // 암호화된 이미지 파일 읽기
    let mut file = File::open("./src/encrypted_image1024.jpg").unwrap();
    let mut encrypted_image_bytes = Vec::new();
    file.read_to_end(&mut encrypted_image_bytes).unwrap();

    // Base64 디코딩
    // let encrypted_image_bytes = base64::decode(&encrypted_image_bytes).unwrap();
    println!("{:?}", &encrypted_image_bytes);

    // 복호화
    let key = b"oingisprettyintheworld1234567890"; // 임의의 문자열
    let iv = b"\xda\xe4\x87\xa7i\x8arbOC\xbd \x02\xbc*[";
    let mut decryptor = cbc_decryptor(KeySize::KeySize256, key, iv, NoPadding);
    // let mut decrypted_image = Vec::<u8>::new();
    let length = encrypted_image_bytes.len();
    let mut decrypted_image = vec![0; length];
    let mut read_buffer = RefReadBuffer::new(&encrypted_image_bytes);
    let mut write_buffer = RefWriteBuffer::new(&mut decrypted_image);
    decryptor
        .decrypt(&mut read_buffer, &mut write_buffer, true)
        .unwrap();

    println!("{:?}", &decrypted_image);

    // 복호화된 이미지 파일 저장
    let path = Path::new("decrypted_image1024.jpg");
    let mut file = File::create(&path).unwrap();
    file.write_all(&decrypted_image).unwrap();
}
