use std::path::Path;

fn main() {
    let current_dir = std::env::current_dir().unwrap();

    // generate types from IDL
    let abigen_crate_dir = current_dir.join(Path::new("crates/drift-abi-gen"));
    let output = std::process::Command::new("make")
        .current_dir(abigen_crate_dir)
        .args(&["build"])
        .output()
        .expect("abi-gen built");

    println!("abi-gen build: {output:?}");

    // generate ffi lib
    let ffi_crate_dir = current_dir.join(Path::new("crates/drift-ffi"));
    let output = std::process::Command::new("cargo")
        .current_dir(ffi_crate_dir)
        .args(&["build", "--release"])
        .output()
        .expect("ffi built");

    println!("ffi build: {output:?}");

    // build sdk crate
    /*
       MACOSX_DEPLOYMENT_TARGET="14.4" RUSTFLAGS="-L native=$(shell pwd)/../drift-ffi/target/release -l dylib=drift_ffi" cargo build
    */
    println!(
        "cargo::rustc-link-search=native={}/crates/drift-ffi/target/release/",
        current_dir.to_string_lossy()
    );
    println!("cargo::rustc-link-lib=dylib=drift_ffi");
}
