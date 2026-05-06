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

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn call_parasail_nw() {
        unsafe {
            let alphabet = CString::new("ACGT").unwrap();
            let matrix = parasail_matrix_create(alphabet.as_ptr(), 2, -1);
            assert!(!matrix.is_null());

            let s1 = CString::new("ACGT").unwrap();
            let s2 = CString::new("ACGT").unwrap();
            let result = parasail_nw(s1.as_ptr(), 4, s2.as_ptr(), 4, 5, 1, matrix);
            assert!(!result.is_null());
            assert_eq!((*result).score, 8);

            parasail_result_free(result);
            parasail_matrix_free(matrix);
        }
    }
}
