// parasail symbols do not follow Rust style conventions, so we suppress warnings for them.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

/// # Introduction
/// [libparasail-sys]() is a Rust crate that provides FFI bindings to [parasail](), a SIMD C
/// library for pairwise sequence alignment. This sub-crate uses an autotools build for the C
/// library, and [bindgen]() to automatically generate bindings. Safe bindings are planned
/// and will be provided as a seperate crate in the future.
///
/// For mor information on parasail, see the original C library [documentation](https://github.com/jeffdaily/parasail).

mod parasail_bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
