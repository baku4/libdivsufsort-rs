use std::env;
use std::path::PathBuf;

use cmake::Config;

fn main() {
    // output path
    let out_path = env::var("OUT_DIR").unwrap();
    let out_path_buf = PathBuf::from(out_path.clone());

    // cmake build
    #[cfg(target_pointer_width = "64")]
    let dst = Config::new("libdivsufsort")
        .define("BUILD_EXAMPLES", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("BUILD_DIVSUFSORT64", "ON")
        .define("CMAKE_INSTALL_LIBDIR", out_path.clone())
        .build();
    #[cfg(not(target_pointer_width = "64"))]
    let dst = Config::new("libdivsufsort")
        .define("BUILD_EXAMPLES", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_INSTALL_LIBDIR", out_path.clone())
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=divsufsort");

    // bindgen
    #[cfg(target_pointer_width = "64")]
    let bindings = bindgen::Builder::default()
        .header(out_path_buf.join("include/divsufsort64.h").into_os_string().into_string().unwrap())
        .generate()
        .expect("Unable to generate bindings");
    #[cfg(not(target_pointer_width = "64"))]
    let bindings = bindgen::Builder::default()
        .header(out_path_buf.join("include/divsufsort.h").into_os_string().into_string().unwrap())
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(out_path_buf.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}