use std::env;
use cmake::Config;

fn main() {
    // output path
    let out_path = env::var("OUT_DIR").unwrap();

    // cmake build
    let dst = Config::new("src/libdivsufsort")
        .define("BUILD_EXAMPLES", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("BUILD_DIVSUFSORT64", "ON")
        .define("CMAKE_INSTALL_LIBDIR", out_path)
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=divsufsort");
}