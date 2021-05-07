fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/blazevm/vm.cpp")
        .flag_if_supported("-std=c++14")
        .compile("blazevm");

    println!("cargo:rerun-if-changed=src/blazevm/vm.cpp");
    println!("cargo:rerun-if-changed=src/blazevm/vm.h");
}
