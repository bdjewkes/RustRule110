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

fn read_width(error: bool) -> u8{
	if error {
		println!("Invalid entry. Please specify your board with.");
	} else {
		println!("Please specify your board witdth");
	}
	let mut width = String::new();
	io::stdin().read_line(&mut width)
			   .expect("Failed to read line");

	match width.trim().parse::<u8>() {
 		Ok(n) => return n,	
 		Err(n) => return read_width(true),
 	}	
}

fn read_generations(error: bool) -> u32
{
	if error {
		println!("Invalid entry. Please specify how many generations to run");
	} else {
		println!("Please specify how many generations to run");
	}
	let mut gen = String::new();

	io::stdin().read_line(&mut gen)
			   .expect("Failed to read line");
	match gen.trim().parse::<u32>() {
 		Ok(n) => return n,	
 		Err(n) => read_generations(true),
  	}
}



fn generate_ca(gens: u32, board_width: u8){
	let ruleset = [false,true,true,true,false,true,true,false];

	let width = board_width as usize;
	let mut cells = [false; 256];
	let mut next_cells = [false; 256];

	cells[width - 2] = true;

	let mut current_gen = 0;

	write_generation(cells, width);

	while current_gen < gens {
		current_gen+=1;
		for x in 1..width - 1 {
		
			let left_cell = cells[x - 1];
			let current_cell = cells[x];
			let right_cell = cells[x + 1];
			
			next_cells[x] = ruleset[get_rule_index(
					left_cell, current_cell, right_cell) 
					as usize];
		}
		cells = next_cells;	
		write_generation(next_cells, width);
		thread::sleep_ms(50);
	}
}

fn get_rule_index(left: bool, current: bool, right: bool) -> u8 {
	let prev_gen = (left, current, right);
	match prev_gen
	{
		(true, true, true) => return 0,
		(true, true, false) => return 1,
		(true, false, true) => return 2,
		(false, true, true) => return 3,
		(true, false, false) => return 4,
		(false, true, false) => return 5,
		(false, false, true) => return 6,
		(false, false, false) => return 7
	}
}

fn write_generation(cells: [bool; 256], width: usize)
{
	let alive = "|";
	let dead = " ";
	//leave a buffer cell on either bound of the array
	//the borders of the 'board' should always be 'dead's
	for x in 1..width - 1{
		if cells[x]{
			print!("{}", alive)
		}
		else { print!("{}", dead)}
	}
	println!("");
}

