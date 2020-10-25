fn main() {
    if cfg!(target_os = "linux") {
        println!("cargo:rerun-if-changed=src/c/vritual_alloc_linux.c");

        cc::Build::new()
            .file("src/c/vritual_alloc_linux.c")
            .compile("virtual-alloc");
    }
    else {
        panic!("Unsupported platform");
    }
}
