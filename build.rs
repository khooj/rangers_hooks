use std::env::current_dir;

pub fn main() {
    let p = current_dir().expect("current dir");
    let p = p.join("src");
    let p = p.join("dsound.def");
    let s = p.as_path().to_str().expect("to_str").to_string();
    println!("cargo:rustc-link-arg=/DEF:{}", s);
    println!("cargo:rerun-if-changed=build.rs");
}