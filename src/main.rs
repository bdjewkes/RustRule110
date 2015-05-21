use std::io;
use std::u8;

fn main(){
	println!("Rule 110.");

	println!("How many generations would you like to run?");

	let gens = take_num_input();

	println!("How wide would you like each line to be?");
	
	let width = take_num_input();	


	println!("Running {} generations with a line width of {}", gens, width);

	generate_ca(gens, width);
}

fn take_num_input() -> u32
{
	let mut usr_input = String::new();
	io::stdin().read_line(&mut usr_input)
		.ok()
		.expect("Failed to read line.");

	let usr_input: u32 = usr_input.trim().parse()
            .ok()
            .expect("Please type a number!");

    usr_input        
}

fn generate_ca(gens: u32, width: u32){
	let ruleset = [0,1,1,0,1,1,1,0];
	let mut cells = [1, 0, 1, 1, 0, 1, 1, 1, 0, 1];
	let mut next_cells = [0; 10];
	
	let mut current_gen = 0;
	let mut current_width = 0;
	while current_gen < gens {
		for x in 1..cells.len()-1 {

			let left_cell = cells[x-1];
			
			let current_cell = cells[x];

			let right_cell = cells[x+1];
			
			let i = rules(left_cell, current_cell, right_cell);
			next_cells[x] = ruleset[i as usize];
			print!("{}", ruleset[i as usize]);
			current_width+= 1;
			if current_width >= width  {
				println!("");
				current_width = 0;
			}
		}
		cells = next_cells;
		current_gen+=1;
	}
}

fn rules(left: i32, current: i32, right: i32) -> u8 {
	let s = left.to_string() + &current.to_string() + &right.to_string();
	let x = u8::from_str_radix(&s, 2);
	return x.unwrap();
}

