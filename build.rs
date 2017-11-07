extern crate cc;

fn main() {
	//! Builds libhello.a file
    cc::Build::new().file("legacy/hello.c").compile("hello");
}