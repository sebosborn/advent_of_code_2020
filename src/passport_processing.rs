/* Advent of Code 2020

--- Day 4: Passport Processing ---
https://adventofcode.com/2020/day/4 */

// HashMap is Rust's dictionary type
use std::collections::HashMap;

pub fn part_one(passport_raw: &String) -> bool {
	let mut passport: HashMap<String, String> = HashMap::new();
	let fields = passport_raw.split_whitespace().collect::<Vec<_>>();
	for field in fields {
		let field = field.split(':').collect::<Vec<_>>();
		passport.insert(String::from(field[0]), String::from(field[1]));
	}
	
	if passport.contains_key("byr") && 
		passport.contains_key("iyr") && 
		passport.contains_key("eyr") && 
		passport.contains_key("hgt") && 
		passport.contains_key("hcl") && 
		passport.contains_key("ecl") && 
		passport.contains_key("pid") {
		return true
	} else {
		return false
	}
}

pub fn part_two<'a>(passport_raw: &String) -> bool {
	let mut passport: HashMap<String, String> = HashMap::new();
	let fields = passport_raw.split_whitespace().collect::<Vec<_>>();
	for field in fields {
		let field = field.split(':').collect::<Vec<_>>();
		passport.insert(String::from(field[0]), String::from(field[1]));
	}
	
	if ! (passport.contains_key("byr") &&
		passport.contains_key("iyr") &&
		passport.contains_key("eyr") &&
		passport.contains_key("hgt") &&
		passport.contains_key("hcl") &&
		passport.contains_key("ecl") &&
		passport.contains_key("pid")) {
		return false
	}
	
	let mut valid = true;
	// Is there a simple way to parse() the value of
	//   Some() instead of blindly unwrap()-ing first?
	match passport.get("byr").unwrap().parse().unwrap() {
		1920..=2002 => valid = valid,
		_ => valid = false,
	};
	match passport.get("iyr").unwrap().parse().unwrap() {
		2010..=2020 => valid = valid,
		_ => valid = false,
	};
	match passport.get("eyr").unwrap().parse().unwrap() {
		2020..=2030 => valid = valid,
		_ => valid = false,
	};
	
	// hgt
	let hgt_raw = match passport.get("hgt") {
		Some(x) => x,
		_ => "",
	};
	if ! (hgt_raw.contains("cm") || hgt_raw.contains("in")) {
		valid = false;
	} else {
		if hgt_raw.contains("cm") {
			let hgt = match hgt_raw[0..3].parse() {
				Ok(x) => x,
				_ => 0,
			};
			match hgt {
				150..=193 => valid = valid,
				_ => valid = false,
			};
		} else {
			let hgt = match hgt_raw[0..2].parse() {
				Ok(x) => x,
				_ => 0,
			};
			match hgt {
				59..=76 => valid = valid,
				_ => valid = false,
			};
		}
	}

	// hcl
	match passport.get("hcl") {
		Some(x) => {
			let hcl = passport.get("hcl").unwrap();
			if ! hcl.len() == 7 {
				valid = false;
			} else {
				match hcl.chars().nth(0).unwrap() {
					'#' => {
						let mut hcl_numbers = hcl.clone();
						hcl_numbers.remove(0);
						match usize::from_str_radix(&hcl_numbers, 16) {
							Ok(_x) => valid = valid,
							_ => valid = false,
						};
					},
					_ => valid = false,
				}
			}
		}
		_ => valid = false,
	};
	
	// ecl
	match passport.get("ecl") {
		// Is there a way to match Some without "x"?
		Some(x) => match passport.get("ecl").unwrap().as_str() {
					"amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => valid = valid,
					_ => valid = false,
				},
		_ => valid = false,
	};
	
	// pid
	match passport.get("pid") {
		Some(x) => valid = valid,
		_ => valid = false,
	};
	let pid = match passport.get("pid") {
		Some(x) => x,
		_ => "",
	};
	if ! (pid.len() == 9) {
		valid = false;
	} else {
		match pid.parse::<usize>() {
			Ok(_x) => valid = valid,
			_ => valid = false,
		}
	}

	return valid
}

#[cfg(test)]
mod tests {
	// Note this useful idiom: importing names from outer (for mod tests) scope.
	use super::*;
	
	#[test]
	fn test_part_one() {
		let passports = 
			"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
			byr:1937 iyr:2017 cid:147 hgt:183cm\n\
			\n\
			iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
			hcl:#cfa07d byr:1929\n\
			\n\
			hcl:#ae17e1 iyr:2013\n\
			eyr:2024\n\
			ecl:brn pid:760753108 byr:1931\n\
			hgt:179cm\n\
			\n\
			hcl:#cfa07d eyr:2025 pid:166559648\n\
			iyr:2011 ecl:brn hgt:59in";
		
		//split passports by double-newlines
		let passports = passports.split("\n\n").collect::<Vec<_>>();
		
		for x in 0..passports.len() {
			//1st and 3rd are valid
			//2nd and 4th are invalid
			if x == 0 || x == 2 { assert_eq!(part_one(&String::from(passports[x])), true); }
			else { assert_eq!(part_one(&String::from(passports[x])), false); }
		}
	}
	
	#[test]
	fn test_part_two() {
		
		// Added cases where a wrong pid was being accepted
		let passports = 
			"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n\
			hcl:#623a2f\n\
			\n\
			eyr:2029 ecl:blu cid:129 byr:1989\n\
			iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\
			\n\
			hcl:#888785\n\
			hgt:164cm byr:2001 iyr:2015 cid:88\n\
			pid:545766238 ecl:hzl\n\
			eyr:2022\n\
			\n\
			iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719\n\
			\n\
			eyr:1972 cid:100\n\
			hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\
			\n\
			iyr:2019\n\
			hcl:#602927 eyr:1967 hgt:170cm\n\
			ecl:grn pid:012533040 byr:1946\n\
			\n\
			hcl:dab227 iyr:2012\n\
			ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\
			\n\
			hgt:59cm ecl:zzz\n\
			eyr:2038 hcl:74454a iyr:2023\n\
			pid:3556412378 byr:2007\n\
			\n\
			pid:7876582 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n\
			hcl:#623a2f\n\
			\n\
			pid:5837609946 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n\
			hcl:#623a2f";
		
		//split passports by double-newlines
		let passports = passports.split("\n\n").collect::<Vec<_>>();
		
		for x in 0..passports.len() {
			println!("{}: {}", x+1, passports[x]);
			
			//First four are valid
			//Last six are invalid
			if x < 4 { assert_eq!(part_two(&String::from(passports[x])), true); }
			else { assert_eq!(part_two(&String::from(passports[x])), false); }
		}
	}
}