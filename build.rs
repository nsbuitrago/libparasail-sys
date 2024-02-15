extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn try_system_parasail() -> Result<pkg_config::Library, pkg_config::Error> {
    let mut cfg = pkg_config::Config::new();
    match cfg.atleast_version("2.4.2").probe("parasail") {
        Ok(lib) => {
            for include in &lib.include_paths {
                println!("cargo:root={}", include.display());
            }
            Ok(lib)
        },
        Err(e) => {
            println!("cargo:warning=Could not find system parasail: {e}",);
            Err(e)
        }
    }
}

fn main() {
    // Use `PARASAIL_NO_VENDOR` to try and force to use system parasail.
    // println!("cargo:rerun-if-env-changed=PARASAIL_NO_VENDOR");
    // let force_no_vendor = env::var("PARASAIL_NO_VENDOR").is_ok();
    // if force_no_vendor {
    //  if try_system_parasail().is_err() {
    //    panic!("Could not find system parasail");
    //  }
    //  return;
    // }
    //

    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let parasail_dir = PathBuf::from(&project_dir).join("parasail")
        .canonicalize()
        .expect("Failed to find parasail directory");
    let parasail_build = PathBuf::from(&out_dir).join("parasail_build");
    std::fs::create_dir_all(&parasail_build).unwrap();

    let headers_path = parasail_dir.join("parasail.h");
    let headers_path_str = headers_path.to_str().unwrap();

    let configure_script = parasail_dir.join("configure");
    assert!(Command::new(configure_script)
        .arg(format!("--prefix={}", out_dir))
        .current_dir(&parasail_build)
        .status()
        .unwrap()
        .success(), "Failed to configure");

    assert!(std::process::Command::new("make")
        .current_dir(&parasail_build)
        .status()
        .unwrap()
        .success(), "Failed to make");

    assert!(std::process::Command::new("make")
        .arg("install")
        .current_dir(&parasail_build)
        .status()
        .unwrap()
        .success(), "Failed to make install");

    assert!(std::process::Command::new("make")
        .arg("clean")
        .current_dir(&parasail_build)
        .status()
        .unwrap()
        .success(), "Failed to make clean");

    println!("cargo:rustc-link-search=native={}/lib", out_dir);
    println!("cargo:rustc-link-lib=static=parasail");

    let bindings = bindgen::Builder::default()
        .header(headers_path_str)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(out_dir);
    bindings
        .write_to_file(out_path.join("parasail_bindings.rs"))
        .expect("Couldn't write bindings!");
}

