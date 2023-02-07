extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut cfg = cc::Build::new();
    cfg.warnings(false);

    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    cfg.include("Texture2DDecoderNative")
        .file("Texture2DDecoderNative/astc.cpp")
        .file("Texture2DDecoderNative/atc.cpp")
        .file("Texture2DDecoderNative/bcn.cpp")
        .file("Texture2DDecoderNative/crunch.cpp")
        .file("Texture2DDecoderNative/dllmain.cpp")
        .file("Texture2DDecoderNative/etc.cpp")
        .file("Texture2DDecoderNative/pvrtc.cpp")
        .file("Texture2DDecoderNative/unitycrunch.cpp")
        .out_dir(dst.join("lib"))
        .compile("libTexture2DDecoderNative.a");

    let src = env::current_dir().unwrap().join("Texture2DDecoderNative");
    println!("cargo:root={}", src.display());
    println!("cargo:include={}", src.display());

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("Texture2DDecoderNative/dllmain.cpp")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
