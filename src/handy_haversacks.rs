/* Advent of Code 2020

--- Day 7: Handy Haversacks ---
https://adventofcode.com/2020/day/7 */

use std::collections::HashMap;

fn shiny_gold(bag: &str, rules: &HashMap<&str, Vec<(usize, &str)>>, has_shiny_gold: Option<bool>) -> bool {
	let mut has_shiny_gold = has_shiny_gold.unwrap_or(false);
	for subbag in &rules[bag] {
		if subbag.1 == "shiny gold" {
			has_shiny_gold = true;
		} else {
			has_shiny_gold = shiny_gold(&subbag.1, &rules, Some(has_shiny_gold));
		}
	}
	return has_shiny_gold
}

fn count_bags(bag: &str, rules: &HashMap<&str, Vec<(usize, &str)>>) -> usize {
	let mut count = 0;
	if rules[bag].len() > 0 {
		for subbag in &rules[bag] {
			count += subbag.0;
			count += subbag.0 * count_bags(subbag.1, &rules);
		}
	}
	return count
}

pub fn part_one(rules_raw: &Vec<&str>) -> usize {
	let mut valid = 0;
	let mut rules: HashMap<&str, Vec<(usize, &str)>> = HashMap::new();
	
	for rule in rules_raw {
		//"light red bags contain 1 bright white bag, 2 muted yellow bags."
		let rule = rule.split("contain").collect::<Vec<_>>();
		//["light red bags ", " 1 bright white bag, 2 muted yellow bags."]
		let bag = rule[0].split_at(rule[0].trim().rfind(' ').unwrap()).0;
		
		let mut contains: Vec<_> = Vec::new();
		let mut contains_tmp = Vec::new();
		if ! (rule[1] == " no other bags.") {
			// [" 1 bright white bag", " 2 muted yellow bags."];
			contains_tmp = rule[1].trim().split(", ").collect::<Vec<_>>();
		}
		for x in contains_tmp {
			let quantity: usize = x.split_at(x.find(' ').unwrap()).0.parse().unwrap();
			// " muted yellow bags."
			let kind = x.split_at(x.find(' ').unwrap()).1;
			// "muted yellow"
			let kind = kind.split_at(kind.trim().rfind(' ').unwrap() + 1).0.trim();
			contains.append(&mut vec![(quantity, kind)]);
		}
		rules.insert(bag, contains);
	}
	let rules = rules;
	
	for bag in &rules {
		if shiny_gold(&bag.0, &rules, None) {
			valid += 1;
		}
	}
	return valid
}

pub fn part_two(rules_raw: &Vec<&str>) -> usize {
	let mut rules: HashMap<&str, Vec<(usize, &str)>> = HashMap::new();
	
	for rule in rules_raw {
		//"light red bags contain 1 bright white bag, 2 muted yellow bags."
		let rule = rule.split("contain").collect::<Vec<_>>();
		//["light red bags ", " 1 bright white bag, 2 muted yellow bags."]
		let bag = rule[0].split_at(rule[0].trim().rfind(' ').unwrap()).0;
		
		let mut contains: Vec<_> = Vec::new();
		let mut contains_tmp = Vec::new();
		if ! (rule[1] == " no other bags.") {
			// [" 1 bright white bag", " 2 muted yellow bags."];
			contains_tmp = rule[1].trim().split(", ").collect::<Vec<_>>();
		}
		for x in contains_tmp {
			let quantity: usize = x.split_at(x.find(' ').unwrap()).0.parse().unwrap();
			// " muted yellow bags."
			let kind = x.split_at(x.find(' ').unwrap()).1;
			// "muted yellow"
			let kind = kind.split_at(kind.trim().rfind(' ').unwrap() + 1).0.trim();
			contains.append(&mut vec![(quantity, kind)]);
		}
		rules.insert(bag, contains);
	}
	let rules = rules;
	
	return count_bags("shiny gold", &rules)
}

#[cfg(test)]
mod tests {
	// Note this useful idiom: importing names from outer (for mod tests) scope.
	use super::*;
	
	#[test]
	fn test_part_one() {
		let rules = vec![
			"light red bags contain 1 bright white bag, 2 muted yellow bags.",
			"dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
			"bright white bags contain 1 shiny gold bag.",
			"muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
			"shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
			"dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
			"vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
			"faded blue bags contain no other bags.",
			"dotted black bags contain no other bags."
		];
		assert_eq!(part_one(&rules), 4);
	}
	
	#[test]
	fn test_part_two() {
		let rules = vec![
			"light red bags contain 1 bright white bag, 2 muted yellow bags.",
			"dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
			"bright white bags contain 1 shiny gold bag.",
			"muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
			"shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
			"dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
			"vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
			"faded blue bags contain no other bags.",
			"dotted black bags contain no other bags."
		];
		let rules_2 = vec![
			"shiny gold bags contain 2 dark red bags.",
			"dark red bags contain 2 dark orange bags.",
			"dark orange bags contain 2 dark yellow bags.",
			"dark yellow bags contain 2 dark green bags.",
			"dark green bags contain 2 dark blue bags.",
			"dark blue bags contain 2 dark violet bags.",
			"dark violet bags contain no other bags.",
		];
		assert_eq!(part_two(&rules), 32);
		assert_eq!(part_two(&rules_2), 126);
	}
}