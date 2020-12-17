/* Advent of Code 2020

--- Day 2: Password Philosophy ---
https://adventofcode.com/2020/day/2 */

pub fn part_one(password: &str) -> bool {
	// split() returns an iter which we must collect()
	let password = password.split_whitespace().collect::<Vec<_>>();
	
	let policy = password[0].split('-').collect::<Vec<_>>();
	let policy_low: usize = policy[0].parse().unwrap(); //parse() returns an enum which we must unwrap()
	let policy_high: usize = policy[1].parse().unwrap();
	let policy_char: char = password[1].chars().nth(0).unwrap(); //char.nth() returns an enum which we must unwrap()
	let password = password[2];
	
	let count = password.matches(policy_char).count();
	if policy_low <= count && count <= policy_high {
		return true
	} else {
		return false
	}
}

pub fn part_two(password: &str) -> bool {
	let password = password.split_whitespace().collect::<Vec<_>>();
	
	let policy = password[0].split('-').collect::<Vec<_>>();
	let pos_one: usize = policy[0].parse().unwrap();
	let pos_two: usize = policy[1].parse().unwrap();
	let policy_char: char = password[1].chars().nth(0).unwrap();
	let password = password[2];
	
	let mut count = 0;
	if password.chars().nth(pos_one-1).unwrap() == policy_char { count += 1; }
	if password.chars().nth(pos_two-1).unwrap() == policy_char { count += 1; }
	if count == 1 {
		return true
	} else {
		return false
	}
}