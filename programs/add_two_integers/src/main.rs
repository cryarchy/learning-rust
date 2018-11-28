use std::io::{ stdin };
use std::error::Error;
use std::process;

fn get_integer() -> Result<i32, Box<Error>> {
	let mut input = String::new();
    println!("Enter an integer: ");
    stdin().read_line(&mut input)?;
	let integer: i32 = input.trim().parse()?;
	Ok(integer)
}

fn run() -> Result<(), Box<Error>> {
	let integer_1 = get_integer()?;
	let integer_2 = get_integer()?;
	println!(
		"The sum of {} and {} is {}", integer_1, integer_2, integer_1 + integer_2
	);
	Ok(())
}

fn main() {
    if let Err(e) = run() {
    	eprintln!("Application error: {}", e);
    	process::exit(1);
    }
}
