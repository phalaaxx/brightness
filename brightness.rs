use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

/* read_brightness reads integer value from a text file */
fn read_brightness(name: &str) -> i32 {
	// open file for reading
	let mut f = match File::open(name) {
		Err(why) => panic!("File::open({}): {}", name, why.description()),
		Ok(file) => file,
	};
	// read file contents
	let mut contents = String::new();
	match f.read_to_string(&mut contents) {
		Err(why) => panic!("read_to_string: {}", why.description()),
		Ok(_) => (),
	};
	// convert to int and return
	let brightness:i32 = contents.trim_matches('\n').parse().unwrap();
	brightness
}

/* adjust_brightness updates a brightness value into a file */
fn adjust_brightness(name: &str, delta: i32, max: i32) {
	let mut brightness = read_brightness(name);
	// calculate next brightness value
	brightness += delta;
	if brightness > max {
		brightness = max;
	}
	if brightness < 0 {
		brightness = 0;
	}
	// save new value to file
	let mut f = match File::create(name) {
		Err(why) => panic!("could not open {}: {}", name, why.description()),
		Ok(file) => file,
	};
	match f.write_all(brightness.to_string().as_bytes()) {
		Err(why) => panic!("could not write to file: {}", why.description()),
		Ok(_) => (),
	};
}

/* main program */
fn main() {
	let args:Vec<String> = env::args().collect();
	let srcfile = &args[1];
	let max:i32 = (&args[2]).parse().unwrap();
	let delta:i32 = (&args[3]).parse().unwrap();
	if delta != 0 {
		adjust_brightness(srcfile, delta, max);
	}
}
