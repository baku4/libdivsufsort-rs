use std::env;
use std::path::Path;

use git2::Repository;
use cmake::Config;

fn main() {
    // libdivsufsort repository
    let repo_url = "https://github.com/y-256/libdivsufsort";

    // output path
    let out_path_str = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_path_str);
    let repo_path = out_path.join("libdivsufsort");

    // clone repository
    if !repo_path.exists() {
        match Repository::clone(repo_url, repo_path.clone()) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to clone: {}", e),
        };
    }

    // cmake build
    let dst = Config::new(repo_path)
        .define("BUILD_EXAMPLES", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("BUILD_DIVSUFSORT64", "ON")
        .define("CMAKE_INSTALL_LIBDIR", out_path)
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=divsufsort");
}