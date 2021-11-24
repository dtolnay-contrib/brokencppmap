use std::{env, path::PathBuf};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=wrapper.h");
    let src = ["vendor/proj/proj.cpp"];

    cc::Build::new()
        .cpp(true)
        .files(src.iter())
        .flag("-std=c++11")
        // .flag("-shared")
        .compile("proj");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .enable_cxx_namespaces()
        .clang_arg("-xc++")
        .clang_arg("-std=c++11")
        .clang_arg("-Ivendor/proj")
        .derive_default(true)
        .allowlist_type("Configuration")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    Ok(())
}
