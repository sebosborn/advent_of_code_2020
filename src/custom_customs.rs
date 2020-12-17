/* Advent of Code 2020

--- Day 6: Custom Customs ---
https://adventofcode.com/2020/day/6 */

// HashSet is good for a simple list of unique elements
use std::collections::HashSet;
// HashMap is good for key-value pairs
use std::collections::HashMap;

// Sum the number of questions to which anyone answered "yes".
pub fn part_one(group: &String) -> usize {
	let mut yes = HashSet::new();
	for person in group.lines() {
		for x in 0..person.len() {
			yes.insert(person.chars().nth(x).unwrap());
		}
	}
	return yes.len()
}

// Sum the number of questions to which EVERYONE answered "yes".
pub fn part_two(group: &String) -> usize {
	let mut yes = HashMap::new();
	let group = group.split("\n").collect::<Vec<_>>();
	for person in &group {
		for x in 0..person.len() {
			let key = person.chars().nth(x).unwrap();
			if yes.contains_key(&key) {
				yes.insert(key, yes[&key] + 1);
			} else {
				yes.insert(key, 1);
			}
		}
	}
	
	let mut all = 0;
	for val in yes.values() {
		if val == &group.len() {
			all += 1;
		}
	}
	return all
}

#[cfg(test)]
mod tests {
	// Note this useful idiom: importing names from outer (for mod tests) scope.
	use super::*;
	
	#[test]
	fn test_part_one() {
		let file =
			"abc\n\
			\n\
			a\n\
			b\n\
			c\n\
			\n\
			ab\n\
			ac\n\
			\n\
			a\n\
			a\n\
			a\n\
			a\n\
			\n\
			b";
		
		let list = file.split("\n\n").collect::<Vec<_>>();
		
		let mut sum = 0;
		for x in 0..list.len() {
			sum += part_one(&String::from(list[x]));
		}
		assert_eq!(sum, 11);
	}
	
	#[test]
	fn test_part_two() {
		let file =
			"abc\n\
			\n\
			a\n\
			b\n\
			c\n\
			\n\
			ab\n\
			ac\n\
			\n\
			a\n\
			a\n\
			a\n\
			a\n\
			\n\
			b";
		
		let list = file.split("\n\n").collect::<Vec<_>>();
		
		let mut sum = 0;
		for x in 0..list.len() {
			sum += part_two(&String::from(list[x]));
		}
		
		assert_eq!(sum, 6);
	}
}