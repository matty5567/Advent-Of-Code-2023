use std::fs::File;
use std::io::Read;

fn main() {
	let mut f = File::open("data/data.txt").unwrap();
	let mut input = String::new();
	f.read_to_string(&mut input).unwrap();

	let mut total: u32 = 0;

	let mut digits = Vec::<u32>::new();
	
	// For part 2, replace words with their digit representation, 
	// keeping the first in last letters in the cases where there can be overlap, eg "oneight" becomes 18

	let str_digits = vec![
		("one", "o1e"), ("two", "t2o"), ("three", "t3"), 
		("four", "4"), ("five", "5fe"), ("six", "6"), 
		("seven", "7"), ("eight", "e8t"), ("nine", "n9e")];

	for (digit, rep_digit) in str_digits {
		input = input.replace(digit, rep_digit);
	}

	for c in input.chars() {
		if c == '\n' {
			total += digits[0] * 10 + digits[1];
			digits = vec![];
		} else if c.is_numeric() {
			let digit = c.to_digit(10).unwrap();
			if digits.len() == 0 { 
				digits = vec![digit, digit];
			} else {
				digits[1] = digit;
			}
		}
	}
	println!("{}", total);	
}
