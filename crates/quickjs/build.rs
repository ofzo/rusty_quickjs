use std::env;
use std::process::Command;

fn main() {
    let output = env::var("OUT_DIR").unwrap();
    Command::new("make")
        .args(&["libquickjs.a"])
        .current_dir("./quickjs/")
        .output()
        .unwrap();
    Command::new("cp")
        .args(&["./quickjs/libquickjs.a", &output])
        .output()
        .unwrap();
    Command::new("bindgen")
        .current_dir("./quickjs/")
        .args(&["./quickjs.h", "-o", &format!("{}/quickjs.rs", output)])
        .output()
        .unwrap();

    println!("cargo:rustc-link-search={output}");
    println!("cargo:rustc-link-lib=quickjs");
}
