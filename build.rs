extern crate bindgen;
extern crate cc;

use std::process::Command;
use std::path::PathBuf;

use bindgen::CargoCallbacks;

fn main() {

    let libstark = PathBuf::from("../libSTARK/bin/libstark/")
        .canonicalize()
        .expect("Failed to canonicalize libstark");

    println!("cargo:rustc-link-search={}", libstark.display());
    println!("cargo:rustc-link-lib=stark");

    Command::new("ar").args(&["cr", "libBairWitnessChecker_UTEST.a", "../libSTARK/bin/libstark-tests/obj/BairWitnessChecker_UTEST.o"])
        .status().expect("Failed to archive BairWitnessChecker_UTEST");

    let pwd = PathBuf::from("./")
        .canonicalize()
        .expect("Failed to canonicalize libstark_tests");

    println!("cargo:rustc-link-search=native={}", pwd.display());
    println!("cargo:rustc-link-lib=BairWitnessChecker_UTEST");
    println!("cargo:rustc-link-arg=-Wl");

    // This is the path to the `c` headers file.
    let wrapper_header = pwd.join("wrapper.h");
    let header_path_str = wrapper_header.to_str().expect("Path is not a valid string");

    // This is the path to the intemediate object file.
    let bindings = bindgen::Builder::default()
        .header(header_path_str)
        .enable_cxx_namespaces()
        .allowlist_function("wrapper_*")
        .allowlist_function("PCP_UTESTS::generate_valid_constraints*")
        .clang_arg("-I../libSTARK/libstark-tests/")
        .clang_arg("-I../libSTARK/libstark/src/")
        .clang_arg("-I../libSTARK/libstark/src/languages/Bair/")
        .clang_arg("-I../libSTARK/algebra/algebralib/headers/")
        .clang_arg("-I../libSTARK/algebra/algebralib/headers/algebraLib/")
        .clang_arg("-I../libSTARK/algebra/FFT/src/")
        .clang_arg("-I/usr/local/Cellar/libomp/16.0.2/include/")
        .clang_arg("-xc++")
        .clang_arg("-std=c++14")
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("./bindings.rs");
    bindings.write_to_file(out_path).expect("Couldn't write bindings!");
}
