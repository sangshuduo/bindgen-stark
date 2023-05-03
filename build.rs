extern crate bindgen;

use std::path::PathBuf;

use bindgen::CargoCallbacks;

fn main() {
    let libstark_tests_path = PathBuf::from("./")
        .canonicalize()
        .expect("Failed to canonicalize libstark_tests");

    // This is the path to the `c` headers file.
    let header_libstark_test_path = libstark_tests_path.join("wrapper.h");
    let header_libstark_test_path_str = header_libstark_test_path.to_str().expect("Path is not a valid string");

    // This is the path to the intemediate object file.
    // let libstark_test_obj_path = libstark_tests_path.join("BairWitnessChecker_UTEST.o")
    let bindings = bindgen::Builder::default()
        .header(header_libstark_test_path_str)
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

    let out_path = PathBuf::from("./").join("bindings.rs");
    bindings.write_to_file(out_path).expect("Couldn't write bindings!");
}
