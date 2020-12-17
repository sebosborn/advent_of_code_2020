/* Advent of Code 2020

--- Day 3: Toboggan Trajectory ---
https://adventofcode.com/2020/day/3 */

pub fn part_one(map: &Vec<&str>) -> usize {
	let mut pos = 0;
	let mut trees = 0;
	for line in map {
		
		// Don't look at the first spot
		if pos == 0 {
			pos += 3;
			continue;
		}
		
		// '#' is a tree, '.' is open
		if line.chars().nth(pos % line.len()).unwrap() == '#' { trees += 1; }
		pos += 3;
	}
	return trees
}

pub fn part_two(map: &Vec<&str>) -> usize {
	let mut slopes: Vec<usize> = vec![];
	let mut product = 1;
	for motion in &[[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]] {
		let right = motion[0];
		let down = motion[1];
	
		let mut pos = right;
		let mut skip = down;
		let mut trees = 0;
		for line in map {
			
			// Don't look at the first spot
			// Also, skip if required
			if 0 < skip {
				skip -= 1;
				continue;
			} else {
				skip = down-1;
			}
			
			// '#' is a tree, '.' is open
			if line.chars().nth(pos % line.len()).unwrap() == '#' { trees += 1; }
			pos += right;
		}
		slopes.push(trees);
	}
	for slope in slopes {
		product = product * slope;
	}
	return product
}

//https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
	// Note this useful idiom: importing names from outer (for mod tests) scope.
	use super::*;
	
	#[test]
	fn test_part_one() {
		let map = vec![
			"..##.......",
			"#...#...#..",
			".#....#..#.",
			"..#.#...#.#",
			".#...##..#.",
			"..#.##.....",
			".#.#.#....#",
			".#........#",
			"#.##...#...",
			"#...##....#",
			".#..#...#.#"
		];
		println!("{}", map.get(0).unwrap().len());
		assert_eq!(part_one(&map), 7);
	}
	
	#[test]
	fn test_part_two() {
		let map = vec![
			"..##.......",
			"#...#...#..",
			".#....#..#.",
			"..#.#...#.#",
			".#...##..#.",
			"..#.##.....",
			".#.#.#....#",
			".#........#",
			"#.##...#...",
			"#...##....#",
			".#..#...#.#"
		];
		println!("You should see:\
		\n	Ran into 2 trees on slope: Right 1, down 1\
		\n	Ran into 7 trees on slope: Right 3, down 1\
		\n	Ran into 3 trees on slope: Right 5, down 1\
		\n	Ran into 4 trees on slope: Right 7, down 1\
		\n	Ran into 2 trees on slope: Right 1, down 2");
		assert_eq!(part_two(&map), 336);
	}
}