use std::path::{Path, PathBuf};

fn main() {
    let path = Path::new("https://abler-dev-gltf-encryptor.s3.ap-northeast-2.amazonaws.com/gltf-encryptor/2c59d36b-38ed-4fa7-9e9f-5fcefad33886/untitled.bin").to_str().unwrap();
    println!("{}", path.ends_with(".bin"));
}
