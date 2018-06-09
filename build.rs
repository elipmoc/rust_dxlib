use std::env;

fn main() {
    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}\\DxLib", out_dir);
}
