/* Advent of Code 2020

--- Day 8: Handheld Halting ---
https://adventofcode.com/2020/day/8 */

fn test_program(program: &Vec<(&str, isize)>) -> bool {
	let mut pass = true;
	let mut line = 0;
	let mut visited: Vec<usize> = Vec::new();
	while line < program.len() {
		visited.push(line);
		match program[line].0 {
			"acc" => line += 1,
			"jmp" => {
				if program[line].1.is_negative() {
					line -= program[line].1.abs() as usize;
				} else {
					line += program[line].1.abs() as usize;
				}
			},
			"nop" => line += 1,
			_ => panic!("op was not one of: acc, jmp, nop"),
		}
		if visited.contains(&line) {
			pass = false;
			break;
		}
	}
	return pass
}

pub fn part_one(program_raw: &Vec<&str>) -> isize {
	let mut program: Vec<(&str, isize)> = Vec::new();
	for line in program_raw {
		let op = line.split(' ').collect::<Vec<_>>()[0];
		let arg: isize = line.split(' ').collect::<Vec<_>>()[1].parse().unwrap();
		program.push((op, arg));
	}
	
	let mut acc: isize = 0;
	let mut line = 0;
	let mut visited: Vec<usize> = Vec::new();
	while ! visited.contains(&line) {
		visited.push(line);
		match program[line].0 {
			"acc" => {
				acc += program[line].1;
				line += 1;
			},
			"jmp" => {
				if program[line].1.is_negative() {
					line -= program[line].1.abs() as usize;
				} else {
					line += program[line].1.abs() as usize;
				}
			},
			"nop" => line += 1,
			_ => panic!("op was not one of: acc, jmp, nop"),
		}
	}
	return acc;
}

pub fn part_two(program_raw: &Vec<&str>) -> isize {
	let mut program: Vec<(&str, isize)> = Vec::new();
	for line in program_raw {
		let op = line.split(' ').collect::<Vec<_>>()[0];
		let arg: isize = line.split(' ').collect::<Vec<_>>()[1].parse().unwrap();
		program.push((op, arg));
	}
	
	let mut line = 0;
	let mut visited: Vec<usize> = Vec::new();
	while (! visited.contains(&line)) && line < program.len() {
		visited.push(line);
		match program[line].0 {
			"acc" => line += 1,
			"jmp" => {
				if program[line].1.is_negative() {
					line -= program[line].1.abs() as usize;
				} else {
					line += program[line].1.abs() as usize;
				}
			},
			"nop" => line += 1,
			_ => panic!("op was not one of: acc, jmp, nop"),
		}
	}
	
	for x in 0..(program.len() - 1) {
		match program[x].0 {
			"jmp" => program[x].0 = "nop",
			"nop" => program[x].0 = "jmp",
			"acc" => (),
			_ => panic!("op was not one of: acc, jmp, nop"),
		}
		if test_program(&program) {
			break;
		} else {
			match program[x].0 {
				"jmp" => program[x].0 = "nop",
				"nop" => program[x].0 = "jmp",
				"acc" => (),
				_ => panic!("op was not one of: acc, jmp, nop"),
			}
		}
	}
	
	let mut acc: isize = 0;
	let mut line = 0;
	let mut visited: Vec<usize> = Vec::new();
	while (! visited.contains(&line)) && line < program.len() {
		visited.push(line);
		match program[line].0 {
			"acc" => {
				acc += program[line].1;
				line += 1;
			},
			"jmp" => {
				if program[line].1.is_negative() {
					line -= program[line].1.abs() as usize;
				} else {
					line += program[line].1.abs() as usize;
				}
			},
			"nop" => line += 1,
			_ => panic!("op was not one of: acc, jmp, nop"),
		}
	}
	return acc;
}

#[cfg(test)]
mod tests {
	// Note this useful idiom: importing names from outer (for mod tests) scope.
	use super::*;
	
	#[test]
	fn test_part_one() {
		let program = vec![
			"nop +0",
			"acc +1",
			"jmp +4",
			"acc +3",
			"jmp -3",
			"acc -99",
			"acc +1",
			"jmp -4",
			"acc +6"
		];
		assert_eq!(part_one(&program), 5);
	}
	
	#[test]
	fn test_part_two() {
		let program = vec![
			"nop +0",
			"acc +1",
			"jmp +4",
			"acc +3",
			"jmp -3",
			"acc -99",
			"acc +1",
			"jmp -4",
			"acc +6"
		];
		assert_eq!(part_two(&program), 8);
	}
}