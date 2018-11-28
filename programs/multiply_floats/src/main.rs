use std::io::{ stdin };
use std::error::Error;
use std::process;

fn get_float() -> Result<f32, Box<Error>> {
	let mut input = String::new();
    println!("Enter a float: ");
    stdin().read_line(&mut input)?;
	let float: f32 = input.trim().parse()?;
	Ok(float)
}

fn run() -> Result<(), Box<Error>> {
	let float_1 = get_float()?;
	let float_2 = get_float()?;
	println!(
		"The product of {} and {} is {}", float_1, float_2, float_1 * float_2
	);
	Ok(())
}

fn main() {
    if let Err(e) = run() {
    	eprintln!("Application error: {}", e);
    	process::exit(1);
    }
}
