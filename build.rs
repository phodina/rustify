extern crate cc;

fn main() {
	//! Builds libhello.a & libmultiply.a files
    cc::Build::new().file("legacy/hello.c").compile("hello");
    cc::Build::new().cpp(true).file("legacy/multiply.cpp").compile("multiply");   
}