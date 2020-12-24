use std::fs::File;
use std::io::Read;

mod report_repair;
mod password_philosophy;
mod toboggan_trajectory;
mod passport_processing;
mod binary_boarding;
mod custom_customs;
mod handy_haversacks;

fn read_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Failed to open file!");
    let mut contents = String::new();
	file.read_to_string(&mut contents).expect("Failed to read file into String!");
    return contents.trim_end().to_string() // My input files have a trailing newline
}

fn main() {
	//--- Day 1: Report Repair ---
	//https://adventofcode.com/2020/day/1
	
	//https://adventofcode.com/2020/day/1/input
	let file = read_file("res/report_repair_input.txt");
	let expense_report = file.split("\n").collect::<Vec<_>>();
	
	//let expense_report = input_data::report_repair();
	
	print!("Report Repair: Part One");
	let correction = report_repair::part_one(&expense_report);
	match correction {
		Some(x) => println!("\tThe product of the two elements which sum to 2020 is: {:?}", x),
		None => println!("\tError: could not find two elements that sum to 2020 in the following list: {:?}", &expense_report)
	}
	
	print!("Report Repair: Part Two");
	let correction = report_repair::part_two(&expense_report);
	match correction {
		Some(x) => println!("\tThe product of the three elements which sum to 2020 is: {:?}", x),
		None => println!("\tError: could not find three elements that sum to 2020 in the following list: {:?}", &expense_report)
	}
	
	//--- Day 2: Password Philosophy ---
	//https://adventofcode.com/2020/day/2
	
	//https://adventofcode.com/2020/day/2/input
	let file = read_file("res/password_philosophy_input.txt");
	let list = file.split("\n").collect::<Vec<_>>();
	
	print!("Password Philosophy: Part One");
	let mut valid = 0;
	for password in &list {
		if password_philosophy::part_one(&password) {
			valid += 1;
		}
	}
	println!("\tFound {} valid passwords", valid);
	
	print!("Password Philosophy: Part Two");
	let mut valid = 0;
	for password in &list {
		if password_philosophy::part_two(&password) {
			valid += 1;
		}
	}
	println!("\tFound {} valid passwords", valid);
	
	//--- Day 3: Toboggan Trajectory ---
	//https://adventofcode.com/2020/day/3
	
	//https://adventofcode.com/2020/day/3/input
	let file = read_file("res/toboggan_trajectory_input.txt");
	let map = file.split("\n").collect::<Vec<_>>();
	
	print!("Toboggan Trajectory: Part One");
	println!("\tRan into {} trees", toboggan_trajectory::part_one(&map));
	
	print!("Toboggan Trajectory: Part Two");
	println!("\tThe product of the trees I ran into on the slopes is: {}", toboggan_trajectory::part_two(&map));
	
	//--- Day 4: Passport Processing ---
	//https://adventofcode.com/2020/day/4
	
	//https://adventofcode.com/2020/day/4/input
	//split passports by double-newlines
	let file = read_file("res/passport_processing_input.txt");
	let passports = file.split("\n\n").collect::<Vec<_>>();
	
	print!("Passport Processing: Part One");
	let mut valid = 0;
	for x in 0..passports.len() {
		if passport_processing::part_one(&String::from(passports[x])) == true { valid += 1; }
	}
	println!("\tFound {} valid passports", valid);
	
	print!("Passport Processing: Part Two");
	let mut valid = 0;
	for x in 0..passports.len() {
		if passport_processing::part_two(&String::from(passports[x])) == true { valid += 1; }
	}
	println!("\tFound {} valid passports", valid);
	
	//--- Day 5: Binary Boarding ---
	//https://adventofcode.com/2020/day/5
	
	//https://adventofcode.com/2020/day/5/input
	let file = read_file("res/binary_boarding_input.txt");
	let boarding_passes = file.split("\n").collect::<Vec<_>>();
	
	print!("Binary Boarding: Part One");
	println!("\tThe highest seat ID on the boarding passes was: {}", binary_boarding::part_one(&boarding_passes));
	
	print!("Binary Boarding: Part Two");
	println!("\tMy seat ID is: {}", binary_boarding::part_two(&boarding_passes));

	//--- Day 6: Custom Customs ---
	//https://adventofcode.com/2020/day/6
	
	//https://adventofcode.com/2020/day/6/input
	let file = read_file("res/custom_customs_input.txt");
	let list = file.split("\n\n").collect::<Vec<_>>();
	
	print!("Custom Customs: Part One");
	let mut sum = 0;
	for x in 0..list.len() {
		sum += custom_customs::part_one(&String::from(list[x]));
	}
	println!("\tThe number of questions to which anyone answered \"yes\": {}", sum);
	
	print!("Custom Customs: Part Two");
	let mut sum = 0;
	for x in 0..list.len() {
		sum += custom_customs::part_two(&String::from(list[x]));
	}
	println!("\tThe number of questions to which EVERYONE answered \"yes\": {}", sum);
	
	//--- Day 7: Handy Haversacks ---
	//https://adventofcode.com/2020/day/7
	
	//https://adventofcode.com/2020/day/7/input
	let file = read_file("res/handy_haversacks_input.txt");
	let rules = file.split("\n").collect::<Vec<_>>();
	
	print!("Handy Haversacks: Part One");
	println!("\tThe number of bag colors that eventually contain at least one shiny gold bag is: {}", handy_haversacks::part_one(&rules));
	
	print!("Handy Haversacks: Part Two");
	println!("\tThe number of bags required inside your shiny gold bag is: {}", handy_haversacks::part_two(&rules));
}