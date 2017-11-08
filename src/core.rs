extern crate clap;

use std::ffi::CString;
use std::io::Error;
use std::os::raw::c_char;


extern {
    #[no_mangle]
    fn hello();
    #[no_mangle]
    fn place(place: *const c_char);
    #[no_mangle]
    fn multiply(x: i32, y: i32) -> i32;
}


pub fn run(matches: clap::ArgMatches) -> Result<(), Error> {
    
    if matches.is_present("hello") {

        unsafe { hello() }
        }

    if matches.is_present("place") {
    
        let c_place; 

        match matches.value_of("place") {
            None => {
                eprintln!("Missing name of the place. Falling to default.");
                c_place = CString::new("Prague")?
                },
            Some(city) =>c_place = CString::new(city)?,
            }

        unsafe { place(c_place.as_ptr()) }
        }

    if matches.is_present("multiply") {

        let m;

        unsafe {  
            m = multiply(2, 3); 
        }

        println!("Multiply value: {}", m);
        }

    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn multiply_compute () {

        let m;
        unsafe {
            m = multiply(2,3);
            }

        assert_eq!(6, m);
    }
}

