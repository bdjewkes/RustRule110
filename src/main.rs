use std::thread;
use std::io;
use std::time::Duration;
use std::collections::HashMap;


fn main(){
	let mut rules: HashMap<u8, [bool; 8]> = HashMap::new();
	rules.insert(30,  [false,false,false,true,true,true,true,false]);
	rules.insert(54,  [false,false,true,true,false,true,true,false]);
	rules.insert(60,  [false,false,true,true,true,true,false,false]);
	rules.insert(62,  [false,false,true,true,true,true,true,false]);
	rules.insert(90,  [false,true,false,true,true,false,true,false]);
	rules.insert(94,  [false,true,false,true,true,true,true,false]);
	rules.insert(102, [false,true,true,false,false,true,true,false]);
	rules.insert(110, [false,true,true,false,true,true,true,false]);
	rules.insert(122, [false,true,true,true,true,false,true,false]);
	rules.insert(126, [false,true,true,true,true,true,true,false]);
	rules.insert(150, [true,false,false,true,false,true,true,false]);
	rules.insert(182, [true,false,true,true,false,true,true,false]);
	rules.insert(188, [true,false,true,true,true,true,false,false]);
	rules.insert(190, [true,false,true,true,true,true,true,false]);
	rules.insert(220, [true,true,false,true,true,true,false,false]);
	rules.insert(222, [true,true,false,true,true,true,true,false]);
	rules.insert(250, [true,true,true,true,true,false,true,false]);
	
	
	let width = read_width(false);
	let rule_key = select_rule(false);
	let rule = rules.get(&rule_key).unwrap();
	generate_ca(rule, read_generations(false), width);
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
 		Err(_) => return read_width(true),
 	}	
}
fn select_rule(error: bool) -> u8{
	if error {
		println!("Invalid entry. Please select your rule.");
	} else {
		println!("Please select a rule.");
	}
	let mut rule = String::new();
	io::stdin().read_line(&mut rule)
			   .expect("Failed to read line");

	match rule.trim().parse::<u8>() {
 		Ok(n) => return n,	
 		Err(_) => return select_rule(true),
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
 		Err(_) => read_generations(true),
  	}
}



fn generate_ca(rules: &[bool; 8], gens: u32, board_width: u8){
	let width = board_width as usize;
	let mut cells = [false; 256];
	let mut next_cells = [false; 256];

	cells[width/2] = true;

	let mut current_gen = 0;

	for x in 1..width
	{
		write_cell(cells[x]);
	}
	println!("");

	while current_gen < gens {
		current_gen+=1;
		for x in 1..width {
			let left_cell = cells[x - 1];
			let center_cell = cells[x];
			let right_cell = cells[x + 1];
			
			next_cells[x] = rules[get_rule_index(
					left_cell, center_cell, right_cell) 
					as usize];
			write_cell(next_cells[x]);
		}
		println!("");
		cells = next_cells;	

		let sleep_duration = Duration::from_millis(30);
		thread::sleep(sleep_duration);
	}
}

fn get_rule_index(left: bool, center: bool, right: bool) -> u8 {
	let prev_gen = (left, center, right);
	match prev_gen
	{
		(true, true, true) => return 0,
		(true, true, false) => return 1,
		(true, false, true) => return 2,
		(true, false, false) => return 3,
		(false, true, true) => return 4,
		(false, true, false) => return 5,
		(false, false, true) => return 6,
		(false, false, false) => return 7
	}
}

fn write_cell(alive: bool)
{
	let alive_char = "|";
	let dead_char = " ";
	if alive{
		print!("{}", alive_char)
	}	
	else { 
		print!("{}", dead_char)
	}
}

