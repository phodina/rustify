extern crate clap;
use std::ffi::CString;
use std::io::Error;
use std::os::raw::c_char;




mod lib {

    use super::c_char;

    extern {
        pub fn hello();
        pub fn multiply(x: i32, y: i32) -> i32;
        pub fn place(city: *const c_char);
    }
}

fn hello () {

    unsafe { lib::hello(); }
    }

fn multiply (x: i32, y: i32) -> i32 {

    let m;
    
    unsafe { m = lib::multiply(x, y); }

    m
    }

fn place (city: *const c_char) {

    unsafe { lib::place(city) }
}

pub fn run(matches: clap::ArgMatches) -> Result<(), Error> {
    
    if matches.is_present("hello") {

        hello();
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

        place(c_place.as_ptr());
        }

    if matches.is_present("multiply") {

        let m = multiply(2, 3); 

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

