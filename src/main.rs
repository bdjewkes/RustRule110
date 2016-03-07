use std::thread;
use std::io;

fn main(){
	let generations = 250;
	//if the board is too wide for your console, decrease the board width.
	println!("Rule 110 implemented in Rust.");
	
	let width = read_width(false);
	generate_ca(read_generations(false), width);
	println!("<--------------------FIN-------------------->");
}

fn read_width(error: bool) -> usize{
	if error {
		println!("Invalid entry. Please specify your board with.");
	} else {
		println!("Please specify your board witdth");
	}
	let mut width = String::new();
	io::stdin().read_line(&mut width)
			   .expect("Failed to read line");

	match width.trim().parse::<usize>() {
 		Ok(n) => return n,	
 		Err(n) => return read_width(true),
 	}	
}

fn read_generations(error: bool) -> i32
{
	if error {
		println!("Invalid entry. Please specify how many generations to run");
	} else {
		println!("Please specify how many generations to run");
	}
	let mut gen = String::new();

	io::stdin().read_line(&mut gen)
			   .expect("Failed to read line");
	match gen.trim().parse::<i32>() {
 		Ok(n) => return n,	
 		Err(n) => read_generations(true),
  	}
}



fn generate_ca(gens: i32, width: usize){
	let ruleset = [0,1,1,1,0,1,1,0];
	let mut cells = [0; 150];
	let mut next_cells = [0; 150];
	
	cells[width-1] = 1;

	let mut current_gen = 0;

	let alive = "░";
	let dead = "█";

	//print the first generation
	for x in 1..width-1{
		if cells[x] == 1{
			print!("{}", alive)
		}
		else { print!("{}", dead)}
	}
	println!("");

	while current_gen < gens {
		current_gen+=1;
		for x in 1..width-1{
		
			let left_cell = cells[x-1];
			let current_cell = cells[x];
			let right_cell = cells[x+1];
				
			let i = rules(left_cell, current_cell, right_cell);
			next_cells[x] = ruleset[i as usize];
				
			if ruleset[i as usize] == 1{
				print!("{}", alive);
			}
			else{ print!("{}", dead); }

		}
		cells = next_cells;		
		println!("");
		thread::sleep_ms(50);
	}
}

fn rules(left: i32, current: i32, right: i32) -> u8 {
	let s = left.to_string() + &current.to_string() + &right.to_string();
	let x = u8::from_str_radix(&s, 2).unwrap();
	return x;
}

