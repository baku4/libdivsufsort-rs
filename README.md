# libdivsufsort-rs
`libdivsufsort-rs` is rust wrapper of [`libdivsufsort`](https://github.com/y-256/libdivsufsort)  
## Feature
Including **all APIs** of both **32- and 64-bit** version.
 - More details are in included in original `C` codes of [`libdivsufsort`](https://github.com/y-256/libdivsufsort)  
 - I referred to [`pzip-bwt`](https://crates.io/crates/pzip-bwt) crate, which is more simpler version for wrapping around bwt function of `libdivsufsort`
## Building
For building, source code needs [`libdivsufsort`](https://github.com/y-256/libdivsufsort) as submoudle.
If you are cloned only this repository, submodule updates may be necessary.
```git
git submodule init
git submodule update
```
Or you can clone this repository with mode
```git
git clone --recursive https://github.com/baku4/libdivsufsort-rs.git
```