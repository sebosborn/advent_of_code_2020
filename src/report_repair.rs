/* Advent of Code 2020

--- Day 1: Report Repair ---
https://adventofcode.com/2020/day/1 */

pub fn part_one(list: &Vec<&str>) -> Option<usize> {
	for xx in list {
		let x: usize = xx.parse().unwrap();
		for yy in list {
			let y: usize = yy.parse().unwrap();
			if x + y == 2020 {
				return Some(x * y)
			}
		}
	}
	return None
}

pub fn part_two(list: &Vec<&str>) -> Option<usize> {
	for xx in list {
		let x: usize = xx.parse().unwrap();
		for yy in list {
			let y: usize = yy.parse().unwrap();
			for zz in list {
				let z: usize = zz.parse().unwrap();
				if x + y + z == 2020 {
					return Some(x * y * z)
				}
			}
		}
	}
	return None
}