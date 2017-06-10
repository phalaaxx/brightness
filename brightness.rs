use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

/* read_brightness reads integer value from a text file */
fn read_brightness(name: &str) -> Result<i32, std::io::Error> {
	// open file for reading
	let mut f = match File::open(name) {
		Err(e) => return Err(e),
		Ok(file) => file,
	};
	// read file contents
	let mut contents = String::new();
	match f.read_to_string(&mut contents) {
		Err(e) => return Err(e),
		Ok(_) => (),
	};
	// convert to int and return
	let brightness:i32 = contents.trim_matches('\n').parse().unwrap();
	Ok(brightness)
}

/* adjust_brightness updates a brightness value into a file */
fn adjust_brightness(name: &str, delta: i32, max: i32) -> Result<i32, std::io::Error> {
	// calculate next brightness value
	let mut brightness = match read_brightness(name) {
		Err(e) => return Err(e),
		Ok(b) => b + delta,
	};
	brightness = std::cmp::min(brightness, max);
	brightness = std::cmp::max(brightness, 0);
	// save new value to file
	let mut f = match File::create(name) {
		Err(e) => return Err(e),
		Ok(file) => file,
	};
	match f.write_all(brightness.to_string().as_bytes()) {
		Err(e) => return Err(e),
		Ok(_) => (brightness),
	};
	Ok(brightness)
}

/* main program */
fn main() {
	let args:Vec<String> = std::env::args().collect();
	let srcfile = &args[1];
	let max:i32 = (&args[2]).parse().unwrap();
	let delta:i32 = (&args[3]).parse().unwrap();
	let verbose:bool = if args.len() == 5 { (&args[4]).starts_with("v") } else { false };
	if delta != 0 {
		match adjust_brightness(srcfile, delta, max) {
			Err(why) => println!("adjust_brightness: {}", why.description()),
			Ok(brightness) => if verbose {
				// only show new brightness value for verbose output
				println!("new brightness: {}", brightness);
			},
		};
	}
}
