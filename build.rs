use std::path::Path;

fn main() {
    let current_dir = std::env::current_dir().unwrap();

    // generate types from IDL
    let idlgen_crate_dir = current_dir.join(Path::new("crates/drift-idl-gen"));
    let output = std::process::Command::new("make")
        .current_dir(idlgen_crate_dir)
        .args(["build"])
        .output()
        .expect("idl-gen built");

    println!("idl-gen build: {output:?}");

    // generate ffi lib
    let profile = "release";
    let ffi_crate_dir = current_dir.join(Path::new("crates/drift-ffi"));
    let output = std::process::Command::new("cargo")
        .current_dir(ffi_crate_dir)
        .args(["build", &format!("--profile={profile}")])
        .output()
        .expect("ffi built");

    println!("ffi build: {output:?}");

    // build sdk crate
    println!(
        "cargo:rustc-link-search=native={}/crates/drift-ffi/target/{profile}/",
        current_dir.to_string_lossy()
    );
    println!("cargo:rustc-link-lib=dylib=drift_ffi");
}
