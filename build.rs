use std::path::PathBuf;

fn main() {
    // Vendored libchdb lives in vendor/chdb/. Point the linker there so
    // `#[link(name = "chdb")]` resolves against vendor/chdb/libchdb.so.
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let vendor = manifest_dir.join("vendor").join("chdb");

    println!("cargo:rustc-link-search=native={}", vendor.display());

    // Embed an rpath so the produced binaries/tests find libchdb.so at runtime
    // without requiring LD_LIBRARY_PATH to be set.
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", vendor.display());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=vendor/chdb/libchdb.so");
}
