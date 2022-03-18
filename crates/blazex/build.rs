use std::fs;

fn main() {
    let files = rerunners("stdlib".to_string());
    cc::Build::new()
        .files(files)
        .warnings(true)
        .extra_warnings(true)
        .flag_if_supported("-Wno-unused-result")
        .compile("blazex");
    println!("cargo:rerun-if-changed=build.rs");
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