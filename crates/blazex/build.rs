use std::{env, fs};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let files = rerunners("stdlib".to_string());
    let mut out_dir = env::current_exe().ok().unwrap();
    out_dir.pop();
    out_dir.pop();
    out_dir.pop();
    out_dir.push("stdlib");

    if !out_dir.exists() {
        fs::create_dir(&out_dir).unwrap();
    }
    cc::Build::new()
        .files(files)
        .warnings(true)
        .extra_warnings(true)
        .flag_if_supported("-Wno-unused-result")
        .out_dir(out_dir.as_path())
        .opt_level(3)
        .warnings_into_errors(true)
        .compile("libblazex.a");
}

fn rerunners(path: String) -> Vec<String> {
    let mut vec = vec![];
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            vec.push(path.to_str().unwrap().to_string());
            println!("cargo:rerun-if-changed={}", path.display());
        } else {
            vec.extend(rerunners(entry.path().to_str().unwrap().to_string()));
        }
    }

    vec
}
