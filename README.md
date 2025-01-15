# libparasail-sys

[![GitHub Actions Workflow
Status](https://img.shields.io/github/actions/workflow/status/nsbuitrago/libparasail-sys/test.yml)](https://github.com/nsbuitrago/parasail-rs/actions/workflows/test.yml)
[![docs.rs](https://img.shields.io/docsrs/libparasail-sys)](https://docs.rs/libparasail-sys/latest/libparasail_sys/index.html)
[![Crates.io
Version](https://img.shields.io/crates/v/libparasail-sys)](https://crates.io/crates/libparasail-sys)

This crate provides unsafe Rust bindings for
[parasail](https://github.com/jeffdaily/parasail). You might be interested in
[parasail-rs](https://github.com/nsbuitrago/parasail-rs), which provides safe
Rust bindings and a wrapper to parasail.

Note that [parasail-sys](https://github.com/anp/parasail-sys) similarly provides
unsafe Rust bindings, but has been archived since 2020.  The intention of
libparasail-sys is to provide an up to date set of bindings for parasail and
slightly different API.

## Building

Assuming you have [cargo](https://doc.rust-lang.org/stable/cargo/) setup, you
can build libparasail-sys to check that the bindings compile with `cargo build`.
Note that we try and use system parasail by default and then try to build using
CMake if no system package is found (see
[build.rs](https://gitlab.com/nsbuitrago/libparasail-sys/-/blob/main/build.rs?ref_type=heads)
or the [CMake build
instructions](https://github.com/jeffdaily/parasail/tree/master?tab=readme-ov-file#cmake-build)
in the original library repo). To force using system parasail, set the
`PARASAIL_NO_VENDOR=1` environment variable.

## Testing

Bindgen provides tests to check the generated FFI structs. For verification, run
`cargo test`. Assuming no modification has been made to the `bindings.rs`, this
should pass successfully. This crate has been tested with cargo 1.77.0 - 1.83.0-nightly

## Contributing

Contributions are more than welcome. Please submit an issue or pull request.

## License

libparasail-sys is licensed under the BSD-3-clause license, however, parasail is
licensed under a very similar Batelle BSD-style license and was developed by
[Jeff Daily](https://github.com/jeffdaily) along with other contributors.
