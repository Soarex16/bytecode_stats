fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    // println!("cargo:rerun-if-changed=src/hello.c");
    // Use the `cc` crate to build a C file and statically link it.

    // let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // // println!("cargo:rustc-link-search=native={}", Path::new(&dir).join("runtime").display());
    // println!("cargo:rustc-link-lib={}", Path::new(&dir).join("runtime/runtime.a").display());
    // println!("cargo:rustc-link-search=runtime/");
    // cc::Build::new()
    //     .file("src/hello.c")
    //     .compile("hello");

    use std::env::var;
    use std::path::Path; 
    if var("TARGET").map(|target| target == "i686-unknown-linux-gnu").unwrap_or(false) {
        let dir = var("CARGO_MANIFEST_DIR").unwrap();
        println!("cargo:rustc-link-search=native={}", Path::new(&dir).join("runtime").display());
    }
}