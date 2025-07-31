extern crate bindgen;
extern crate pkg_config;

use cmake;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let vendored = env::var("CARGO_FEATURE_VENDORED").is_ok();
    let zlib_ng_compat = env::var("CARGO_FEATURE_ZLIB_NG_COMPAT").is_ok();

    // Use `PARASAIL_NO_VENDOR` to try and force to use system parasail.
    println!("cargo:rerun-if-env-changed=PARASAIL_NO_VENDOR");
    let force_no_vendor = env::var("PARASAIL_NO_VENDOR").map_or(false, |v| v != "0");

    if force_no_vendor {
        if try_system_parasail().is_err() {
            panic!(
                "The environment variable `PARASAIL_NO_VENDOR` is set, but no system parasail was found.
                Please install parasail or unset `PARASAIL_NO_VENDOR` or use `PARASAIL_NO_VENDOR=0`."
            );
        }

        return;
    }

    let use_system_parasail = !vendored && !zlib_ng_compat;
    if use_system_parasail && try_system_parasail().is_ok() {
        // no issues with system parasail
        return;
    }

    build_parasail();
}

fn bindgen_build(header: &str) {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings = bindgen::Builder::default()
        .header(header)
        .allowlist_item("parasail_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(out_path.join("parasail_bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn try_system_parasail() -> Result<pkg_config::Library, pkg_config::Error> {
    let mut cfg = pkg_config::Config::new();
    match cfg.atleast_version("2.4.2").probe("parasail") {
        Ok(lib) => {
            // tell cargo to look for shared libraries in the specified directory
            println!(
                "cargo:rustc-link-search=native={}",
                lib.link_paths[0].display()
            );
            // tell cargo to link the system parasail shared lib
            println!("cargo:rustc-link-lib=parasail");

            bindgen_build("wrapper.h");
            Ok(lib)
        }
        Err(e) => {
            println!("cargo:warning=Could not find system parasail: {e}",);
            Err(e)
        }
    }
}

fn build_parasail() {
    println!("cargo:rustc-cfg=parasail_vendored");
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    if !Path::new("parasail/src").exists() {
        let _ = Command::new("git")
            .args(["submodule", "update", "--init", "parasail"])
            .status();
    }

    let parasail_dir = PathBuf::from(&project_dir)
        .join("parasail")
        .canonicalize()
        .expect("Failed to find parasail directory");

    let out_dir = env::var("OUT_DIR").unwrap();
    let parasail_build = PathBuf::from(&out_dir).join("parasail_build");
    std::fs::create_dir_all(&parasail_build).unwrap();

    let headers_path = parasail_dir.join("parasail.h");
    let headers_path_str = headers_path.to_str().unwrap();

    // build parasail
    let dst = cmake::Config::new(&parasail_dir)
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("POSITION_INDEPENDENT_CODE", "ON")
        .build_target("parasail")
        .very_verbose(true)
        .out_dir(parasail_build)
        .build();

    let lib_path = dst.join("build");

    // Add the library search path
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=static=parasail");

    bindgen_build(headers_path_str);
}
