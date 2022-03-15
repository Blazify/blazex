use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    println!("cargo:rerun-if-changed=stdlib/main.c");
    Command::new("gcc").args(&["-c", "stdlib/main.c", "-o", &format!("{}/main.o", out_dir)])
        .status().unwrap();
    Command::new("ar").args(&["crus", &format!("{}/libblazex.a", out_dir), &format!("{}/main.o", out_dir)])
        .status().unwrap();
}
