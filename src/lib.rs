//! # Introduction
//! [libparasail-sys]() is a Rust crate that provides FFI bindings to
//! [parasail](https://github.com/jeffdaily/parasail), a SIMD C
//! library for pairwise sequence alignment. You may be interested in its parent crate, [parasail-rs](https://github.com/nsbuitrago/parasail-rs).
//!
//! For mor information on parasail, see the original C library [repository](https://github.com/jeffdaily/parasail).

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/parasail_bindings.rs"));
