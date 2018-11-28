use std::io::{ stdin };
use std::error::Error;
use std::process;

fn get_character() -> Result<char, Box<Error>> {
	let mut input = String::new();
	let mut character_count;
	let mut word;
	loop {
		println!("Enter a character: ");
		stdin().read_line(&mut input)?;
	    word = input.trim().to_string();
	    character_count = word.len();
	    if character_count == 0 {
			println!("You did not enter a character!");
			word.clear();
		} else if character_count > 1 {
			println!("You entered multiple characters!");
			word.clear();
		} else {
		    break;
		}
	}
    let character = word.chars().next().unwrap();
	Ok(character)
}

fn character_to_bytes(character: char) -> Vec<u8> {
	character.to_string().as_bytes().to_vec()
}

fn run() -> Result<(), Box<Error>> {
	let character = get_character()?;
	let bytes = character_to_bytes(character);
	println!("ASCII value of {} = {:?}", character, bytes);
	Ok(())
}

fn main() {
    if let Err(e) = run() {
    	eprintln!("Application error: {}", e);
    	process::exit(1);
    }
}
