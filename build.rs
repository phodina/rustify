extern crate cc;

fn main() {
	//! Builds libhello.a & libmultiply.a files
    cc::Build::new().define("SAY_HI", "\"Say Hi\\n\"").file("legacy/hello.c").compile("hello");
    cc::Build::new().cpp(true).file("legacy/multiply.cpp").compile("multiply");   
}