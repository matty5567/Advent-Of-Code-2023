use std::fs::File; 
use std::io::Read;
use std::collections::HashMap;
use std::cmp::{min, max};

fn main() {
	let mut f = File::open("data/input.txt").unwrap();
	let mut input = String::new();
	f.read_to_string(&mut input).unwrap();

	let manual:Vec<&str> = input.lines().collect();	
	let mut total_valid = 0; 

	let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

	for (row_id, row) in manual.iter().enumerate() {
		let mut num: Vec<char> = vec![];
		let mut is_valid = false;		
		let mut local_gears = vec![];
		for (col_id, c) in row.chars().enumerate() {
			if c.is_digit(10) {
				num.push(c);

				fn valid_char(c: char) -> bool {
					return !(c == '.' || c.is_digit(10));

				}

				let idxs_to_check: Vec<(i32, i32)> = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 0), (0, 1), (1, -1), (1, 0), (1, 1)];
				for (col_diff, row_diff) in idxs_to_check {
					let col_id_to_check = min(max(0, col_id as i32 + col_diff), manual.len() as i32 - 1) as usize;
					let row_id_to_check = min(max(0, row_id as i32 + row_diff), manual.len() as i32 - 1) as usize;
					let char = manual[row_id_to_check].chars().nth(col_id_to_check).unwrap();	
					if char == '*' {local_gears.push((row_id_to_check, col_id_to_check));}
					if valid_char(char) {is_valid=true;}
				}
			}
			if num.len() > 0 && (!c.is_digit(10) || col_id == manual.len() - 1) {
				if num.len() > 0 {
					let valid_num = num.iter().collect::<String>().parse::<u32>().unwrap();
					
					if is_valid {
						total_valid += valid_num;
						local_gears.dedup();
						for gear in &local_gears {
							if gears.contains_key(&gear) {
								gears.get_mut(&gear).map(|val| val.push(valid_num));
							} else {
								gears.insert(*gear, vec![valid_num]);
							}
						}
					}
					num.clear();
					local_gears.clear();
					is_valid=false;
				}
			}
		}
		
	}
	let total_gears = gears.iter().filter(|(_, v)| v.len() == 2).fold(0, |sum, (_, v)| sum + v[0] * v[1]); 
	println!("valid: {} num gears: {}", total_valid, total_gears);
}
