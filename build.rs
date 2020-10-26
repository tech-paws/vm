fn main() {
    if cfg!(target_os = "linux") {
        println!("cargo:rerun-if-changed=src/c/vritual_alloc_linux.c");
        println!("cargo:rerun-if-changed=src/c/commands.c");
        println!("cargo:rerun-if-changed=src/c/memory.c");

        cc::Build::new()
            .file("src/c/vritual_alloc_linux.c")
            .file("src/c/commands.c")
            .file("src/c/memory.c")
            .compile("virtual-alloc");
    }
    else {
        panic!("Unsupported platform");
    }
}
