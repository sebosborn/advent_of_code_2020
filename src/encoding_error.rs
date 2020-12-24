/* Advent of Code 2020

--- Day 9: Encoding Error ---
https://adventofcode.com/2020/day/9 */

pub fn part_one(data: &Vec<&str>, preamble_len: Option<usize>) -> usize {
	let mut preamble: Vec<usize> = Vec::new();
	let preamble_len = preamble_len.unwrap_or(25);
	
	for line in &data[0..preamble_len] {
		preamble.push(line.parse().unwrap());
	}
	
	let mut line = 0;
	let mut x = preamble_len;
	let mut y: usize;
	let mut z: usize;
	let mut valid: bool;
	while x < data.len() {
		line = data[x].parse().unwrap();
		valid = false;
		y = x - preamble_len;
		'outer:while y < x {
			let line_y: usize = data[y].parse().unwrap();
			z = x - preamble_len;
			while z < x {
				let line_z: usize = data[z].parse().unwrap();
				if (line_y + line_z) == line {
					valid = true;
					break 'outer;
				}
				z += 1;
			}
			y += 1;
		}
		if ! valid {
			break;
		}
		x += 1;
	}
	return line
}

pub fn part_two(data: &Vec<&str>, preamble_len: Option<usize>) -> usize {
	let line = part_one(data, preamble_len);

	let mut contig: Vec<usize> = Vec::new();
	let mut sum = 0;
	let mut x = 0;
	while x < data.len() {
		sum += data[x].parse::<usize>().unwrap();
		contig.push(data[x].parse().unwrap());
		
		if contig.len() == 1 {
			x += 1;
			continue;
		}
		
		if sum == line {
			break;
		}
		
		if sum > line {
			sum = 0;
			x -= (contig.len() - 1);
			contig.clear();
		}
		x += 1;
	}
	
	contig.sort_unstable();
	contig.reverse();
	return contig[0] + contig[contig.len() - 1]
}

#[cfg(test)]
mod tests {
	// Note this useful idiom: importing names from outer (for mod tests) scope.
	use super::*;
	
	#[test]
	fn test_part_one() {
		let data = vec![
			"35",
			"20",
			"15",
			"25",
			"47",
			"40",
			"62",
			"55",
			"65",
			"95",
			"102",
			"117",
			"150",
			"182",
			"127",
			"219",
			"299",
			"277",
			"309",
			"576"
		];
		assert_eq!(part_one(&data, Some(5)), 127);
	}
	
	#[test]
	fn test_part_two() {
		let data = vec![
			"35",
			"20",
			"15",
			"25",
			"47",
			"40",
			"62",
			"55",
			"65",
			"95",
			"102",
			"117",
			"150",
			"182",
			"127",
			"219",
			"299",
			"277",
			"309",
			"576"
		];
		assert_eq!(part_two(&data, Some(5)), 62);
	}
}