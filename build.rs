extern crate bindgen;

use std::env;
use std::path::PathBuf;

use bindgen::CargoCallbacks;

fn main() {
    let profile = env::var("PROFILE").unwrap();

    let srcdir_path = PathBuf::from(".")
        // Canonicalize the path as `rustc-link-search` requires an absolute
        // path.
        .canonicalize()
        .expect("cannot canonicalize path");
    let bindir_path = srcdir_path.join(format!("cmake-build-{}", profile));

    // This is the directory where the `c` library is located.
    let libsrc_path = srcdir_path.join("parser");
    let libbin_path = bindir_path.join("parser");

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", libbin_path.to_str().unwrap());

    // Tell cargo to tell rustc to link our `parser` library. Cargo will
    // automatically know it must look for a `libparser.a` file.
    println!("cargo:rustc-link-lib=parser");

    // Tell cargo to invalidate the built crate whenever the header changes.
    println!("cargo:rerun-if-changed={}", libsrc_path.to_str().unwrap());

    if !std::process::Command::new("cmake")
        .arg(format!("-DCMAKE_BUILD_TYPE={}", profile))
        .arg("-S")
        .arg(&srcdir_path)
        .arg("-B")
        .arg(&bindir_path)
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
        .arg(&bindir_path)
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
        .header(libbin_path.join("lex.yy.c").to_str().unwrap())
        .header(libbin_path.join("y.tab.c").to_str().unwrap())
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
