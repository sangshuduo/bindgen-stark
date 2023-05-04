extern crate bindgen;
extern crate cc;

use std::path::PathBuf;
use std::process::Command;

use bindgen::CargoCallbacks;

fn main() {
    let libstark = PathBuf::from("../libSTARK/bin/libstark/")
        .canonicalize()
        .expect("Failed to canonicalize libstark");

    println!("cargo:rustc-link-search={}", libstark.display());
//    println!("cargo:rustc-link-lib=stark");

    Command::new("ar")
        .args(&[
            "cr",
            "libBairWitnessChecker_UTEST.a",
            "../libSTARK/bin/libstark-tests/obj/BairWitnessChecker_UTEST.o",
        ])
        .status()
        .expect("Failed to archive BairWitnessChecker_UTEST");

    let pwd = PathBuf::from("./")
        .canonicalize()
        .expect("Failed to canonicalize libstark_tests");

    let libstark_src = PathBuf::from("../libSTARK/libstark/src/");
    let libstark_tests = PathBuf::from("../libSTARK/libstark-tests/");
    let bin_libstark_tests = PathBuf::from("../libSTARK/bin/libstark-tests/");
    let bair_path = PathBuf::from("../libSTARK//libstark/src/languages/Bair/");
    let algebralib_headers_path = PathBuf::from("../libSTARK/algebra/algebralib/headers");
    let fft_header_path = PathBuf::from("../libSTARK/algebra/FFT/src/");
    let omp_header_path = PathBuf::from("/usr/local/Cellar/libomp/16.0.2/include/");
    cc::Build::new()
        .file("./wrapper.c")
        .file("../libSTARK/libstark-tests/BairWitnessChecker_UTEST.cpp")
        .file("../libSTARK/libstark-tests/lightCircLib/lightCircPoly.cpp")
        .file("../libSTARK/libstark-tests/lightCircLib/lightCircuit.cpp")
        .file("../libSTARK/libstark-tests/lightCircLib/lightCircGate.cpp")
        .compiler("g++")
        .flag("-O3")
        .flag("-g")
        .flag("-Wall")
        .flag("-std=c++14")
        .flag("-fmessage-length=0")
        .flag("-Xpreprocessor")
        .flag("-fopenmp")
        .flag("-maes")
        .flag("-msse4")
        .flag("-mtune=native")
        .include(libstark_src)
        .include(libstark_tests)
        .include(bin_libstark_tests)
        .include(bair_path)
        .include(algebralib_headers_path)
        .include(fft_header_path)
        .include(omp_header_path)
        .cpp(true)
        .compile("wrapper");
    println!("cargo:rerun-if-changed=./wrapper.c");

    Command::new("ar")
        .args(&["cr", "libwrapper.a", "wrapper.o", "../libSTARK/bin/libstark-tests/obj/BairWitnessChecker_UTEST.o"])
        .status()
        .expect("Failed to archive wrapper.a");

    println!("cargo:rustc-link-search=native={}", pwd.display());
//    println!("cargo:rustc-link-lib=BairWitnessChecker_UTEST");
    println!("cargo:rustc-link-lib=wrapper");
    println!("cargo:rustc-link-search=../libSTARK/bin/libstark");
    println!("cargo:rustc-link-lib=stark");
    println!("cargo:rustc-link-search=../libSTARK/bin/fft");
    println!("cargo:rustc-link-lib=FFT");
    println!("cargo:rustc-link-search=../libSTARK/bin/algebralib");
    println!("cargo:rustc-link-lib=algebralib");
    println!("cargo:rustc-link-search=/usr/local/Cellar/libomp/16.0.2/lib");
    println!("cargo:rustc-link-lib=omp");
    println!("cargo:rustc-link-lib=gtest");
    println!("cargo:rustc-link-arg=-Wl");

    // This is the path to the `c` headers file.
    let wrapper_header = pwd.join("wrapper.h");
    let header_path_str = wrapper_header.to_str().expect("Path is not a valid string");

    // This is the path to the intemediate object file.
    let bindings = bindgen::Builder::default()
        .header(header_path_str)
        .enable_cxx_namespaces()
        .allowlist_type("libstark::BairWitness")
        .allowlist_function("wrapper_*")
        .allowlist_function("PCP_UTESTS::generate_valid_constraints*")
        .blocklist_function("PCP_UTESTS::generate_invalid*")
        .blocklist_function("verify_*")
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
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
