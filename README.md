# libparasail-sys

This crate provides unsafe Rust bindings for [parasail](https://github.com/jeffdaily/parasail).
libparasail-sys uses an autotools build for the C library and bindgen to generate Rust bindings. Safe bindings are planned and will be available in the future.

Note that [parasail-sys](https://github.com/anp/parasail-sys) similarly provides unsafe Rust bindings, but has been archived since 2020.
The goal of libparasail-sys is to provide an up to date set of bindings for parasail.

## Building

The parasail C library is built using the preferred autools-based method. You will need a compatible toolchain for these steps.
Assuming you have [cargo](https://doc.rust-lang.org/stable/cargo/) setup, you can build libparasail-sys to check that the bindings compile with `cargo build`.

## Testing

Bindgen provides tests to verify that the layout, size, and alignment of the generated FFI structs match what bindgen thinks they should be.
To check, run `cargo test`. Assuming no modification has been made to the `bindings.rs`, this should pass successfully. This crate has been
tested with cargo 1.77.0-nightly.

## Contributing

Contributions are more than welcome. Please file an issue if you have any trouble with this crate. You can also send an email for
other issues or feedback at nsb5 [at] rice.edu.

## License

libparasail-sys is licensed under the BSD-3-clause license, however, parasail is licensed under a very similar Batelle BSD-style license and was developed by [Jeff Daily](https://github.com/jeffdaily) along with other contributors.

Nicolas Buitrago \<nsb5@rice.edu\>

