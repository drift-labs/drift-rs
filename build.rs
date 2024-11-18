use std::{collections::HashMap, fs::File, io::Write, path::Path};

const LIB: &str = "libdrift_ffi_sys";

fn main() {
    let current_dir = std::env::current_dir().unwrap().canonicalize().unwrap();

    // Generate rust types from anchor IDL
    let idl_source_path = &current_dir.join(Path::new("res/drift.json"));
    let idl_mod_rs = match drift_idl_gen::generate_rust_types(idl_source_path) {
        Ok(idl_mod_rs) => idl_mod_rs,
        Err(err) => panic!("generating IDL failed: {err:?}"),
    };
    let idl_mod_path = current_dir.join(Path::new("crates/src/drift_idl.rs"));
    let mut file = File::create(&idl_mod_path).expect("create IDL .rs");
    file.write_all(idl_mod_rs.as_bytes())
        .expect("wrote IDL .rs");

    if std::env::var("CARGO_DRIFT_FFI_STATIC").is_ok()
        || std::env::var("CARGO_DRIFT_FFI_PATH").is_err()
    {
        // Build + Link FFI crate from source
        println!("{LIB}: building from source...");
        let drift_ffi_sys_crate = current_dir.join(Path::new("crates/drift-ffi-sys"));

        // the x86_64 target must exist
        let host_target = std::env::var("TARGET").unwrap();
        let (lib_target, lib_ext) = if host_target.contains("apple") {
            ("x86_64-apple-darwin", "dylib")
        } else if host_target.contains("linux") {
            ("x86_64-unknown-linux-gnu", "so")
        } else {
            eprintln!("Unsupported host platform: {host_target}, please open an issue at: https://github.com/drift-labs/drift-rs/issues");
            fail_build();
        };

        // "RUSTC" is set as the cargo version of the main SDK build, it must be unset for the ffi build
        // "CARGO*" envs are also configured for the main SDK build
        // https://users.rust-lang.org/t/switching-toolchains-in-build-rs-use-nightly-for-data-generation-and-stable-for-main-compilation/114443/5
        let ffi_build_envs: HashMap<String, String> = std::env::vars()
            .filter(|(k, _v)| !k.starts_with("CARGO") && !k.starts_with("RUSTC"))
            .collect();
        println!("{ffi_build_envs:?}");
        let profile = std::env::var("PROFILE").expect("cargo PROFILE set");

        // force drift-ffi-sys to build with specific toolchain (arch=x86_64, version=<=1.76.0)
        // this ensures zero copy deserialization works correctly with the onchain data layout
        let ffi_toolchain = format!("1.76.0-{lib_target}");
        let installed_toolchains_query = std::process::Command::new("rustup")
            .args(["toolchain", "list"])
            .output()
            .expect("rustup installed");
        if !installed_toolchains_query.status.success() {
            println!("Check 'rustup' is installed and discoverable in system PATH");
        }
        let installed_toolchains = String::from_utf8_lossy(&installed_toolchains_query.stdout);
        if !installed_toolchains.contains(&ffi_toolchain) {
            eprintln!("Required toolchain: {ffi_toolchain} is missing. Run: 'rustup install {ffi_toolchain}' to install and retry the build");
            fail_build();
        }

        // install the dylib to system path
        let lib_major_v = std::env::var("CARGO_PKG_VERSION_MAJOR").unwrap();
        let lib_minor_v = std::env::var("CARGO_PKG_VERSION_MINOR").unwrap();
        let libffi_out_path = drift_ffi_sys_crate.join(Path::new(&format!(
            "target/{profile}/{LIB}.{lib_ext}.{lib_major_v}.{lib_minor_v}"
        )));

        if !libffi_out_path.exists() {
            // Build ffi crate and link
            let mut ffi_build = std::process::Command::new("rustup");
            ffi_build
                .env_clear()
                .envs(ffi_build_envs)
                .current_dir(drift_ffi_sys_crate.clone())
                .args(["run", &ffi_toolchain, "cargo", "build"]);

            match profile.as_str() {
                "debug" => (),
                "release" => {
                    ffi_build.arg("--release");
                }
                custom => {
                    ffi_build.arg(format!("--profile={custom}"));
                }
            }

            let output = ffi_build.output().expect("drift-ffi-sys built");
            if !output.status.success() {
                eprintln!(" {}", String::from_utf8_lossy(output.stderr.as_slice()));
                fail_build();
            }

            if !output.status.success() {
                eprintln!(
                    "{LIB} could not be installed: {}",
                    String::from_utf8_lossy(output.stderr.as_slice())
                );
            }

            if let Ok(out_dir) = std::env::var("OUT_DIR") {
                let _output = std::process::Command::new("cp")
                    .args([
                        libffi_out_path.to_str().expect("ffi build path"),
                        out_dir.as_str(),
                    ])
                    .output()
                    .expect("install ok");
                println!("{LIB}: searching for lib at: {out_dir}");
                println!("cargo:rustc-link-search=native={out_dir}");
            }

            let _output = std::process::Command::new("ln")
                .args([
                    "-sf",
                    libffi_out_path.to_str().expect("ffi build path"),
                    "/usr/local/lib/",
                ])
                .output()
                .expect("install ok");

            println!("{LIB}: searching for lib at: /usr/local/lib");
            println!("cargo:rustc-link-search=native=/usr/local/lib");
        }
    }

    if let Ok(lib_path) = std::env::var("CARGO_DRIFT_FFI_PATH") {
        println!("{LIB}: searching for lib at: {lib_path}");
        println!("cargo:rustc-link-search=native={lib_path}");
    }
    println!("cargo:rustc-link-lib=dylib=drift_ffi_sys");
}

fn fail_build() -> ! {
    eprintln!("{LIB} build failed");
    std::process::exit(1);
}
