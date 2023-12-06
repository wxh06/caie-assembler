extern crate bindgen;

use std::env;
use std::path::PathBuf;

use bindgen::CargoCallbacks;

fn main() {
    // This is the directory where the `c` library is located.
    let libsrc_path = PathBuf::from("parser")
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()
        .expect("cannot canonicalize path");
    let libdir_path = libsrc_path.join("build");

    // This is the path to the `c` headers file.
    let headers_path = libsrc_path.join("parser.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());

    // Tell cargo to tell rustc to link our `parser` library. Cargo will
    // automatically know it must look for a `libparser.a` file.
    println!("cargo:rustc-link-lib=parser");

    // Tell cargo to invalidate the built crate whenever the header changes.
    println!("cargo:rerun-if-changed={}", headers_path_str);

    if !std::process::Command::new("cmake")
        .arg(format!("-DCMAKE_BUILD_TYPE={}", env::var("PROFILE").unwrap()))
        .arg("-S")
        .arg(&libsrc_path)
        .arg("-B")
        .arg(&libdir_path)
        .output()
        .expect("could not spawn `cmake`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not generate buildsystem");
    }

    if !std::process::Command::new("cmake")
        .arg("--build")
        .arg(&libdir_path)
        .output()
        .expect("could not spawn `cmake`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not build project");
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(headers_path_str)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
