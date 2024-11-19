# Change Log

All notable changes are detailed here with headings of \<Version\> - \<YYYY.MM.DD\> in reverse chronological order.

## 0.1.8 - 2024.11.18

### Fixed

- Bindgen >= v0.70 introduced unstable library feature offset_of, causing
Rust < 1.81.0 compilation errors.

## 0.1.7 - 2024.11.1

### Changed

- Add better missing CMake errors
- bump pkg-config, bindgen deps

## 0.1.5/0.1.6 - 2024.04.16

### Features

- Add -DCMAKE_BUILD_TYPE=release and -DCMAKE_POSITION_INDEPENDENT_CODE=ON to
fix some linking issues with PyO3.

## 0.1.4 - 2024.02.25

- Allowlist only items with "parasail" prefix

## 0.1.3 - 2024.02.17

### Changed

- Use standard lib crate type for allowing use as dependency from other projects

## 0.1.2 - 2024.02.17

### Changed

- Moved bindings out of private parasail_bindings module (now removed) to be able to access public functions.

## 0.1.1 - 2024.02.16

### Changed

- Try and use system parasail first by default before building from source
- Add option to set env variable to force using system parasail

## Closed Issues

- Add option for using system lib [#1](https://gitlab.com/nsbuitrago/libparasail-sys/-/issues/1)

## 0.1.0 - 2024.02.15

- First release of libparasail-sys for unsafe bindings to parasail

### Closed Issues

- Use CMake instead of autotools for building parasail from source [#2](https://gitlab.com/nsbuitrago/libparasail-sys/-/issues/2)


