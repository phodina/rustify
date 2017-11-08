extern crate clap;

use std::fs::File;
use clap::{Arg, App};
use std::process;

mod core;

fn main () {

    let matches = App::new("Rustify")
        .version("0.1.0")
        .author("Petr Hodina <hodinapetr46@gmail.com>")
        .about("Calls functions from C/C++ libs")
        .arg(Arg::with_name("hello")
            .short("h")
            .long("hello")
            .help("Calls hello function from C lib"))
        .arg(Arg::with_name("place")
            .short("p")
            .long("place")
            .min_values(0)
            .help("Calls place function from C lib"))
        .arg(Arg::with_name("multiply")
            .short("m")
            .long("multiply")
            .help("Calls multiply function from C++ lib"))
        .get_matches();

    if let Err(e) = core::run(matches) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
