use std::io::{ stdin };

fn main() {
    let mut input = String::new();
    println!("Enter an integer: ");
    stdin()
	    .read_line(&mut input)
	    .expect("An error occured reading the entered string!");
	let integer: u32 = input
		.trim()
		.parse()
		.expect("Error parsing integer");
	println!("You entered: {:?}", integer);
}
