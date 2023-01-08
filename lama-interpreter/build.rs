fn main() {
    use std::env::var;
    use std::path::Path;
    if var("TARGET")
        .map(|target| target == "i686-unknown-linux-gnu")
        .unwrap_or(false)
    {
        let dir = var("CARGO_MANIFEST_DIR").unwrap();

        // Tell Cargo that if the given file changes, to rerun this build script.
        println!("cargo:rerun-if-changed={}", Path::new(&dir).join("runtime/libruntime.a").display());

        println!(
            "cargo:rustc-link-search=native={}",
            Path::new(&dir).join("runtime").display()
        );
    }
}
