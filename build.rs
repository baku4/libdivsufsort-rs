use std::env;
use std::path::Path;

use git2::Repository;
use cmake::Config;

fn main() {

    // output path
    let out_path_str = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_path_str);
    let submodule_path = "src/libdivsufsort";

    // open current repository
    match Repository::open("./") {
        Ok(repo) => {
            // update submodule
            let mut submodule = repo.find_submodule(submodule_path).unwrap();
            if let Err(e) = submodule.update(true, None) {
                panic!("failed to update submodule: {}", e)
            };
        },
        Err(_) => {
            // if empty
            if Path::new(submodule_path).read_dir().unwrap().next().is_none() {
                // cloning libdivsufsort repository
                let repo_url = "https://github.com/y-256/libdivsufsort";
                if let Err(e) = Repository::clone(repo_url, submodule_path) {
                    panic!("failed to clone submodule: {}", e);
                };
            }
        },
    };

    // cmake build
    let dst = Config::new(submodule_path)
        .define("BUILD_EXAMPLES", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("BUILD_DIVSUFSORT64", "ON")
        .define("CMAKE_INSTALL_LIBDIR", out_path)
        .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=divsufsort");
}