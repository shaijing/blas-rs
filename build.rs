use std::{env, path::Path};

fn feature_enabled(feature: &str) -> bool {
    env::var(format!("CARGO_FEATURE_{}", feature.to_uppercase())).is_ok()
}

// --- Windows Platform Logic ---
#[cfg(target_os = "windows")]
fn build_system() {
    #[cfg(target_env = "msvc")]
    {
        if feature_enabled("intel_mkl") {
            let mkl_root = env::var("MKLROOT").expect("MKLROOT should be set");
            let mkl_dir = Path::new(&mkl_root).join("lib");
            println!("cargo:rustc-link-search=native={}", mkl_dir.display());
            println!("cargo:rustc-link-lib=mkl_rt");
            println!("cargo::warning=intel-mkl used (msvc)");
        } else if feature_enabled("openblas") && !feature_enabled("static") {
            unsafe { env::set_var("VCPKGRS_DYNAMIC", "1") };
            if vcpkg::find_package("openblas").is_ok() {
                println!("cargo::warning=vcpkg openblas used");
                return;
            }
            if pkg_config::Config::new()
                .statik(false)
                .probe("openblas")
                .is_ok()
            {
                println!("cargo::warning=pkg_config openblas used");
                return;
            }
        }
    }

    #[cfg(target_env = "gnu")]
    {
        // Add Windows GNU (MinGW) logic here if needed
        println!("cargo::warning=windows gnu detected");
    }
}

// --- Linux Platform Logic ---
#[cfg(target_os = "linux")]
fn build_system() {
    if feature_enabled("intel_mkl") {
        let mkl_root = env::var("MKLROOT").expect("MKLROOT should be set");
        let mkl_dir = Path::new(&mkl_root).join("lib/intel64");
        println!("cargo:rustc-link-search=native={}", mkl_dir.display());
        println!("cargo:rustc-link-lib=mkl_rt");
        println!("cargo::warning=intel-mkl used (linux)");
    } else if feature_enabled("openblas") && !feature_enabled("static") {
        // Try native openblas first
        if pkg_config::Config::new()
            .statik(false)
            .probe("openblas")
            .is_ok()
        {
            println!("cargo::warning=pkg_config openblas used");
        }
        // Fallback to FlexiBLAS (common on Fedora/RHEL)
        else if pkg_config::Config::new()
            .statik(false)
            .probe("flexiblas")
            .is_ok()
        {
            println!("cargo::warning=pkg_config flexiblas used as openblas fallback");
        } else {
            panic!("Error: Could not find OpenBLAS or FlexiBLAS via pkg-config.");
        }
    }
}

// --- macOS Platform Logic ---
#[cfg(target_os = "macos")]
fn build_system() {
    println!("cargo::warning=macos build logic here");
}

// --- Unsupported Platforms ---
#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
fn build_system() {
    println!("cargo:warning=unsupported platform");
}

fn main() {
    // Skip build logic when generating documentation on docs.rs
    if env::var("DOCS_RS").is_ok() {
        return;
    }

    // The compiler picks the correct build_system() version based on the target OS
    build_system();
}
