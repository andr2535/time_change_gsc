use std::io;

fn poke_char_value(letter:char) -> u16 {
	match letter {
		' ' => 0x7F,'!' => 0xE7,'(' => 0x9A,')' => 0x9B, '*' => 0xF1,',' => 0xF4,'-' => 0xE3,'.' => 0xE8,
		'/' => 0xF3,':' => 0x9C,';' => 0x9D,'<' => 0xE1, '>' => 0xE2,'?' => 0xE6,'A' => 0x80,'B' => 0x81,
		'C' => 0x82,'D' => 0x83,'E' => 0x84,'F' => 0x85, 'G' => 0x86,'H' => 0x87,'I' => 0x88,'J' => 0x89,
		'K' => 0x8A,'L' => 0x8B,'M' => 0x8C,'N' => 0x8D, 'O' => 0x8E,'P' => 0x8F,'Q' => 0x90,'R' => 0x91,
		'S' => 0x92,'T' => 0x93,'U' => 0x94,'V' => 0x95, 'W' => 0x96,'X' => 0x97,'Y' => 0x98,'Z' => 0x99,
		'[' => 0x9E,']' => 0x9F,'a' => 0xA0,'b' => 0xA1, 'c' => 0xA2,'d' => 0xA3,'e' => 0xA4,'f' => 0xA5,
		'g' => 0xA6,'h' => 0xA7,'i' => 0xA8,'j' => 0xA9, 'k' => 0xAA,'l' => 0xAB,'m' => 0xAC,'n' => 0xAD,
		'o' => 0xAE,'p' => 0xAF,'q' => 0xB0,'r' => 0xB1, 's' => 0xB2,'t' => 0xB3,'u' => 0xB4,'v' => 0xB5,
		'w' => 0xB6,'x' => 0xB7,'y' => 0xB8,'z' => 0xB9, _ => {println!("Invalid character {}", letter); 0}
	}
}

fn calculate_name_value(name:&str) -> u16 {
	name.chars().take(5).take_while(|ch| *ch != '\n')
	    .fold(0u16, |acc, letter| acc + poke_char_value(letter))
}

fn calculate_id_or_money(question:&str) -> u16 {
	let stdin = io::stdin();
	let input_string = &mut String::new();

	loop {
		println!("{}",question);
		input_string.clear(); // Zero String.
		stdin.read_line(input_string)
		    .expect("error getting input_string");
		input_string.pop(); // Remove endline char.

		let parsed_value = input_string.parse::<u32>();
		match parsed_value {
			Ok(value) => {
				let reduced_value:u16 = (value%65535) as u16;
				return (reduced_value/256)+(reduced_value%256);
			},
			Err(error) => println!("There was an error parsing value, verify \
			                        that you only typed in numbers. \nerror = {}", error)
		};
	}

}

fn main() {
	let stdin = io::stdin();
	let input_string = &mut String::new();
	let mut time_change_value:u16 = 0;

	println!("Pokémon Gold Silver and Crystal time change password generator V2.0");
	stdin.read_line(input_string) // Pause until enter
	    .expect("read_line error");

	println!("To use this in Pokémon Silver or Gold,\n\
	    press Down, Select and B simultaneously at the title screen.\n\n\
	    If you want to use this in Pokémon Crystal\n\
	    the instructions are more complicated: \n\
	    Hold down+select+B\nRelease down+B, leaving select still pressed\n\
	    Hold left+up\nLet go of select\n\n\
	    for characters PK you should type < for MN type >.\n");

	println!("Enter your character name here");
	input_string.clear(); // Zero String.
	stdin.read_line(input_string)
	    .expect("Failed to get character name");
	time_change_value += calculate_name_value(input_string);


	// Calculate trainer ID value.
	time_change_value += calculate_id_or_money("Enter your trainer ID(Only Numbers)");
	// Calculate trainer money value.
	time_change_value += calculate_id_or_money("Enter your money amount(Only Numbers)");

	println!("Your code to change the in-game time is {:05}",time_change_value);
	stdin.read_line(input_string) // Wait for user to click enter.
	    .expect("error getting input_string");
}
