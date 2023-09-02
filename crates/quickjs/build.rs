use std::env;
use std::path::PathBuf;
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
    bindgen::Builder::default()
        .header("quickjs/quickjs.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap()
        .write_to_file(PathBuf::from(&output).join("quickjs.rs"))
        .unwrap();

    println!("cargo:rustc-link-search={output}");
    println!("cargo:rustc-link-lib=quickjs");
}
