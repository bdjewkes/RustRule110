use std::thread;

fn main(){
	let generations = 250;
	//if the board is too wide for your console, decrease the board width.
	let board_width = 150;

	println!("Rule 110 implemented in Rust.");
	println!("If the board is too wide for your console, it can be shortened by setting board_width.");

	generate_ca(generations, board_width);
	println!("<--------------------FIN-------------------->");
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

