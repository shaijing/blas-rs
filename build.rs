use std::{env, path::Path};

fn feature_enabled(feature: &str) -> bool {
    env::var(format!("CARGO_FEATURE_{}", feature.to_uppercase())).is_ok()
}

fn windows() {
    let target_env =
        std::env::var("CARGO_CFG_TARGET_ENV").expect("Error: can't get target_env from cargo!");
    match target_env.as_str() {
        "msvc" => windows_msvc_system(),
        "gnu" => windows_gnu_system(),
        _ => {
            println!("cargo::error=unsupport env");
        }
    };
}
fn windows_msvc_system() {
    if feature_enabled("intel_mkl") {
        let mkl_dir = Path::new(
            &env::var("MKLROOT")
                .expect("MKLROOT should be set")
                .to_string(),
        )
        .join("lib");
        println!("cargo:rustc-link-search=native={}", mkl_dir.display());
        println!("cargo:rustc-link-lib=mkl_rt");
        println!("cargo::warning=intel-mkl used");
    } else if feature_enabled("openblas") {
        if !feature_enabled("static") {
            unsafe { env::set_var("VCPKGRS_DYNAMIC", "1") };
            if let Ok(_) = vcpkg::find_package("openblas") {
                println!("cargo::warning=vcpkg openblas used");
                return;
            }
            if let Ok(_) = pkg_config::Config::new().statik(false).probe("openblas") {
                println!("cargo::warning=pkg_config openblas used");
                return;
            }
        }
    }
}
fn windows_gnu_system() {}
fn macos() {}

fn linux() {}

fn main() {
    if env::var("DOCS_RS").is_ok() {
        return;
    }
    let target_os =
        std::env::var("CARGO_CFG_TARGET_OS").expect("Error: can't get target_os from cargo!");
    match target_os.as_str() {
        "windows" => windows(),
        "macos" => macos(),
        "linux" => linux(),
        _ => {
            println!("cargo::error=unsupport platform");
        }
    };
}
