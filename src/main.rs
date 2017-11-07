use std::ffi::CString;
use std::io::Error;
use std::os::raw::c_char;
use std::process;

extern {
    fn hello();
    fn place(place: *const c_char);
    fn multiply(x: i32, y: i32) -> i32;
}

fn main () {

 	if let Err(e) = run() {
        eprintln!("Application error: {}", e);
		process::exit(1);
	}
}

fn run() -> Result<(), Error> {
    
    unsafe { hello() }

    let city = "Prague";
    
    let c_place = CString::new(city)?;

    unsafe { place(c_place.as_ptr()) }
    

    let m;

    unsafe {  
    	m = multiply(2, 3); 
    }

    println!("Multiply value: {}", m);

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