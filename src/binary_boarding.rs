/* Advent of Code 2020

--- Day 5: Binary Boarding ---
https://adventofcode.com/2020/day/5 */

pub fn part_one(boarding_passes: &Vec<&str>) -> usize {
	let mut highest_id = 0;
	
	for pass in boarding_passes {
		
		// This is basically a binary number represented with characters
		// col is a 7-bit number 0b0000000
		// row is a 3-bit number 0b000
		
		// I'm sure there's a better way to build binary numbers from String input
		let mut row_raw = String::from("0");
		for x in 0..7 {
			match &pass.chars().nth(x).unwrap() {
				'F' => row_raw.push('0'),
				'B' => row_raw.push('1'),
				_ => panic!("Error: column not F nor B"),
			};
		}
		let row = match usize::from_str_radix(&row_raw, 2) {
			Ok(x) => x,
			_ => panic!("Could not parse row"),
		};
		
		let mut col_raw = String::from("00000");
		for x in 7..10 {
			match &pass.chars().nth(x).unwrap() {
				'L' => col_raw.push('0'),
				'R' => col_raw.push('1'),
				_ => panic!("Error: row not L nor R"),
			};
		}
		let col = match usize::from_str_radix(&col_raw, 2) {
			Ok(x) => x,
			_ => panic!("Could not parse column"),
		};

		let id = (row * 8) + col;
		if id > highest_id { highest_id = id; }
	}
	return highest_id;
}

pub fn part_two(boarding_passes: &Vec<&str>) -> usize {
	let mut ids = Vec::new();

	for pass in boarding_passes {
		
		// I'm sure there's a better way to build binary numbers from String input
		let mut row_raw = String::from("0");
		for x in 0..7 {
			match &pass.chars().nth(x).unwrap() {
				'F' => row_raw.push('0'),
				'B' => row_raw.push('1'),
				_ => panic!("Error: row must be F or B"),
			};
		}
		let row = match usize::from_str_radix(&row_raw, 2) {
			Ok(x) => x,
			_ => panic!("Could not parse row"),
		};
		
		let mut col_raw = String::from("00000");
		for x in 7..10 {
			match &pass.chars().nth(x).unwrap() {
				'L' => col_raw.push('0'),
				'R' => col_raw.push('1'),
				_ => panic!("Error: column must be L or R"),
			};
		}
		let col = match usize::from_str_radix(&col_raw, 2) {
			Ok(x) => x,
			_ => panic!("Could not parse column"),
		};
		let id = (row * 8) + col;
		ids.push(id);
	}

	for id in &ids {
		// Is the next ID missing from the list?
		if ! ids.contains(&(id+1)) {
			// Is the ID after the missing one in the list?
			if ids.contains(&(id+2)) {
				return *id+1;
			}
		}
	}
	panic!("Could not find pass!");
}

#[cfg(test)]
mod tests {
	// Note this useful idiom: importing names from outer (for mod tests) scope.
	use super::*;
	
	#[test]
	fn test_part_one() {
		let boarding_passes = vec![
			"FBFBBFFRLR",
			"BFFFBBFRRR",
			"FFFBBBFRRR",
			"BBFFBBFRLL"
		];
		assert_eq!(part_one(&boarding_passes), 820);
	}
	
	#[test]
	fn test_part_two() {
		
		// GENERATE ALL THE PASSES!
		// I assume using a Vec is more efficient, but how do I change a &Vec<String> into a &Vec<&str>?
		//let mut boarding_passes = Vec::new();
		let mut boarding_passes = String::new();
		for x in 0..128 {
			let mut xx = x;
			let mut row = String::new();
			if xx - 64 >= 0 { row.push('B'); xx -= 64; } else { row.push('F'); }
			if xx - 32 >= 0 { row.push('B'); xx -= 32; } else { row.push('F'); }
			if xx - 16 >= 0 { row.push('B'); xx -= 16; } else { row.push('F'); }
			if xx -  8 >= 0 { row.push('B'); xx -=  8; } else { row.push('F'); }
			if xx -  4 >= 0 { row.push('B'); xx -=  4; } else { row.push('F'); }
			if xx -  2 >= 0 { row.push('B'); xx -=  2; } else { row.push('F'); }
			if xx -  1 >= 0 { row.push('B');           } else { row.push('F'); }
			for y in 0..8 {
				let mut yy = y;
				let mut col = String::new();
				if yy - 4 >= 0 { col.push('R'); yy -= 4; } else { col.push('L'); }
				if yy - 2 >= 0 { col.push('R'); yy -= 2; } else { col.push('L'); }
				if yy - 1 >= 0 { col.push('R');          } else { col.push('L'); }
				//boarding_passes.push(format!("{}{}", &row, &col));
				boarding_passes.push_str(&(row.clone() + &col + "\n"));
			}
		}
		boarding_passes = boarding_passes.trim_end().to_string();
		let mut boarding_passes = boarding_passes.split("\n").collect::<Vec<_>>();

		boarding_passes.remove(boarding_passes.iter().position(|&x| x == "FBFBBFFRLR".to_string()).unwrap());
		assert_eq!(part_two(&boarding_passes), 357);
	}
}