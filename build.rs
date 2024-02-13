extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    // This is the directory where the `c` library is located.
    let libdir_path = PathBuf::from("parasail")
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()
        .expect("cannot canonicalize path");

 // configure Parasail
    assert!(std::process::Command::new("sh")
        .arg("-c")
        .arg("autoreconf -fi")
        .current_dir(&libdir_path)
        .status()
        .unwrap()
        .success(), "Failed to autoreconf");

    assert!(std::process::Command::new("./configure")
        .arg(format!("--prefix={}", out_dir))
        .current_dir(&libdir_path)
        .status()
        .unwrap()
        .success(), "Failed to configure");

    // build Parasail
    assert!(std::process::Command::new("make")
        .current_dir(&libdir_path)
        .status()
        .unwrap()
        .success(), "Failed to make");

    assert!(std::process::Command::new("make")
        .arg("install")
        .current_dir(&libdir_path)
        .status()
        .unwrap()
        .success(), "Failed to make install");

    // This is the path to the `c` headers file.
    let headers_path = libdir_path.join("parasail.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    // Tell cargo to tell rustc to link the parasail system library.
    println!("cargo:rustc-link-search=native={}/lib", out_dir);
    println!("cargo:rustc-link-lib=static=parasail");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(headers_path_str)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(out_dir);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

