# libdivsufsort-rs
`libdivsufsort-rs` is the Rust wrapper of [`libdivsufsort`](https://github.com/y-256/libdivsufsort)  
## Feature
This crate includes **all APIs** of both **32- and 64-bit** versions.
 - More details are included in the original `C` codes of [`libdivsufsort`](https://github.com/y-256/libdivsufsort)  
 - I referred to the [`pzip-bwt`](https://crates.io/crates/pzip-bwt) crate, which is simpler version for wrapping around the BWT function of `libdivsufsort`
## Requirements
 - `rustc` >= 1.57.0
 - `cmake`
## Build
For building, the source code needs [`libdivsufsort`](https://github.com/y-256/libdivsufsort) as a submodule.
  - Method (1). If you cloned only this repository, updating the submodule is necessary.
    ```git
    git submodule init
    git submodule update
    ```
  - Method (2). You can clone this repository with `--recursive` option.
    ```git
    git clone --recursive https://github.com/baku4/libdivsufsort-rs.git
    ```
## Docs
[`libdivsufsort-rs`](https://docs.rs/libdivsufsort-rs/)