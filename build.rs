use std::env;
use std::path::PathBuf;

use cmake::Config;

fn main() {
    // output path
    let out_path = env::var("OUT_DIR").unwrap();
    let out_path_buf = PathBuf::from(out_path.clone());

    // cmake build
    let dst = Config::new("libdivsufsort")
        .define("BUILD_EXAMPLES", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("BUILD_DIVSUFSORT64", "ON")
        .define("CMAKE_INSTALL_LIBDIR", out_path.clone())
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=divsufsort");

    // bindgen
    let bind = |is_64: bool| {
        let header_file = out_path_buf.join(
            if is_64 {"include/divsufsort64.h"} else {"include/divsufsort.h"}
        ).into_os_string().into_string().unwrap();
        let binding_file = out_path_buf.join(
            if is_64 {"bindings64.rs"} else {"bindings.rs"}
        );
        let bindings = bindgen::Builder::default()
            .header(header_file)
            .generate()
            .expect("Unable to generate bindings");
        bindings
            .write_to_file(binding_file)
            .expect("Couldn't write bindings!");
    };

    // generate both(64 & 32) bindings
    bind(true);
    bind(false);
}