extern crate cc;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
	//! Builds libhello.a & libmultiply.a files
    cc::Build::new().define("SAY_HI", "\"Say Hi\\n\"").file("legacy/hello.c").compile("hello");
    cc::Build::new().cpp(true).file("legacy/multiply.cpp").compile("multiply");  

    let bindings = bindgen::Builder::default()
        .header("legacy/multiply.hpp")
        .header("legacy/hello.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!"); 
}