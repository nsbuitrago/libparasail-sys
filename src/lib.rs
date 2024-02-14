//! # Introduction
//! [libparasail-sys]() is a Rust crate that provides FFI bindings to [parasail](), a SIMD C
//! library for pairwise sequence alignment. Safe Rust bindings are planned
//! and will be provided as a seperate crate in the future.
//!
//! For mor information on parasail, see the original C library [documentation](https://github.com/jeffdaily/parasail).

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
mod parasail_bindings {
    include!(concat!(env!("OUT_DIR"), "/parasail_bindings.rs"));
}
