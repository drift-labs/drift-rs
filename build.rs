use std::{collections::HashMap, path::Path};

const LIB: &str = "libdrift_ffi_sys";
const SUPPORTED_PLATFORMS: &[(&str, &str, &str)] = &[
    ("apple", "x86_64-apple-darwin", "dylib"),
    ("linux", "x86_64-unknown-linux-gnu", "so"),
];
const FFI_TOOLCHAIN_VERSION: &str = "1.76.0";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?.canonicalize()?;

    // Generate IDL types from 'res/drift.json'
    let idl_source_path = current_dir.join("res/drift.json");
    let idl_mod_path = current_dir.join("crates/src/drift_idl.rs");
    generate_idl_types(&idl_source_path, idl_mod_path.as_path())?;

    // Only build FFI lib if static or no lib path provided
    if should_build_from_source() {
        build_ffi_lib(&current_dir)?;
    }

    link_library()?;
    Ok(())
}

fn generate_idl_types(
    idl_source_path: &Path,
    idl_mod_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let idl_mod_rs = drift_idl_gen::generate_rust_types(idl_source_path)
        .map_err(|err| format!("generating IDL failed: {err:?}"))?;

    std::fs::write(idl_mod_path, idl_mod_rs)?;
    Ok(())
}

fn should_build_from_source() -> bool {
    if std::env::var("CARGO_DRIFT_FFI_STATIC").is_ok() {
        println!("cargo:warning=CARGO_DRIFT_FFI_STATIC on");
        return true;
    }

    match std::env::var("CARGO_DRIFT_FFI_PATH") {
        Err(_) => {
            println!("cargo:warning=CARGO_DRIFT_FFI_PATH not set");
            true
        }
        Ok(ref path) => {
            println!("cargo:warning=CARGO_DRIFT_FFI_PATH set: {path}");
            false
            // check version
            // if let Some(soname) = extract_soname(&format!("{path}/{LIB}")) {
            //     if soname.contains(LIB_VERSION.as_str()) {
            //         println!("cargo:warning=compatible {LIB} version detected ✅");
            //         return false;
            //     } else {
            //         println!("cargo:warning=incompatible {LIB} version detected ❌: {soname}");
            //         println!(
            //             "cargo:warning=update {LIB} to latest release: {}",
            //             LIB_VERSION.as_str()
            //         );
            //         return true;
            //     }
            // }
            // println!("cargo:warning=no compatible {LIB} detected");
            // true
        }
    }
}

fn build_ffi_lib(current_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:warning={LIB}: building from source...");

    let host_target = std::env::var("TARGET")?;
    let (lib_target, lib_ext) = get_platform_details(&host_target)?;

    verify_toolchain(lib_target)?;

    // Build the library
    let profile = std::env::var("PROFILE")?;
    let drift_ffi_sys_crate = current_dir.join("crates/drift-ffi-sys");

    build_with_toolchain(&drift_ffi_sys_crate, lib_target, &profile)?;
    install_library(&drift_ffi_sys_crate, &profile, lib_ext)?;

    Ok(())
}

fn get_platform_details(
    host_target: &str,
) -> Result<(&'static str, &'static str), Box<dyn std::error::Error>> {
    for (platform, target, ext) in SUPPORTED_PLATFORMS {
        if host_target.contains(platform) {
            return Ok((target, ext));
        }
    }

    println!("cargo:warning=Unsupported host platform: {host_target}");
    println!(
        "cargo:warning=Please open an issue at: https://github.com/drift-labs/drift-rs/issues"
    );
    Err("Unsupported platform".into())
}

fn verify_toolchain(lib_target: &str) -> Result<(), Box<dyn std::error::Error>> {
    let ffi_toolchain = format!("{FFI_TOOLCHAIN_VERSION}-{lib_target}");
    let output = std::process::Command::new("rustup")
        .args(["toolchain", "list"])
        .output()?;

    if !output.status.success() {
        return Err("Failed to query rustup toolchains".into());
    }

    let installed_toolchains = String::from_utf8_lossy(&output.stdout);
    if !installed_toolchains.contains(&ffi_toolchain) {
        println!("cargo:warning=Required toolchain {ffi_toolchain} is missing");
        println!("cargo:warning=Run: 'rustup install {ffi_toolchain}' to install");
        return Err("Missing required toolchain".into());
    }

    Ok(())
}

fn build_with_toolchain(
    drift_ffi_sys_crate: &Path,
    lib_target: &str,
    profile: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Filter out cargo and rustc environment variables
    let ffi_build_envs: HashMap<String, String> = std::env::vars()
        .filter(|(k, _v)| !k.starts_with("CARGO") && !k.starts_with("RUSTC"))
        .collect();

    let ffi_toolchain = format!("{FFI_TOOLCHAIN_VERSION}-{lib_target}");
    let mut ffi_build = std::process::Command::new("rustup");
    ffi_build
        .env_clear()
        .envs(ffi_build_envs)
        .current_dir(drift_ffi_sys_crate)
        .args(["run", &ffi_toolchain, "cargo", "build"]);

    match profile {
        "debug" => (),
        "release" => {
            ffi_build.arg("--release");
        }
        custom => {
            ffi_build.arg(format!("--profile={custom}"));
        }
    }

    let output = ffi_build.output()?;
    if !output.status.success() {
        println!("cargo:warning={}", String::from_utf8_lossy(&output.stderr));
        return Err("FFI build failed".into());
    }

    Ok(())
}

fn install_library(
    drift_ffi_sys_crate: &Path,
    profile: &str,
    lib_ext: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let libffi_out_path = drift_ffi_sys_crate.join(format!("target/{profile}/{LIB}.{lib_ext}"));

    if let Ok(out_dir) = std::env::var("OUT_DIR") {
        std::process::Command::new("cp")
            .args([
                libffi_out_path.to_str().ok_or("Invalid path")?,
                out_dir.as_str(),
            ])
            .output()?;
        println!("cargo:warning={LIB}: searching for lib at: {out_dir}");
        println!("cargo:rustc-link-search=native={out_dir}");
    } else {
        // Install to system library path
        std::process::Command::new("ln")
            .args([
                "-sf",
                libffi_out_path.to_str().ok_or("Invalid path")?,
                "/usr/local/lib/",
            ])
            .output()?;

        println!("cargo:warning={LIB}: searching for lib at: /usr/local/lib");
        println!("cargo:rustc-link-search=native=/usr/local/lib");
    }

    Ok(())
}

fn link_library() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(lib_path) = std::env::var("CARGO_DRIFT_FFI_PATH") {
        println!("cargo:rustc-link-search=native={lib_path}");
    }
    println!("cargo:rustc-link-lib=dylib=drift_ffi_sys");
    Ok(())
}

// try to pull the SONAME from drift_ffi_sys lib
// fn extract_soname(path: &str) -> Option<String> {
//     use std::{fs::File, io::Read};
//     use goblin::{elf::Elf, mach::MachO};
//     let mut buffer = Vec::new();

//     // try linux
//     let _ = File::open(format!("{path}.so")).and_then(|mut f| f.read_to_end(&mut buffer));
//     if let Ok(elf) = Elf::parse(&buffer) {
//         return elf.soname.map(|s| s.to_string());
//     }

//     // try mac OS
//     buffer.clear();
//     let _ = File::open(format!("{path}.dylib")).and_then(|mut f| f.read_to_end(&mut buffer));
//     // Parse the macho file
//     if let Ok(macho) = MachO::parse(&buffer, 0) {
//         return macho.name.map(|n| n.to_string());
//     }

//     None
// }
