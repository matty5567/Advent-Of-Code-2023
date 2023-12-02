use std::fs::File;
use std::cmp::max;		
use std::io::Read;

fn main() {
	let mut f = File::open("data/input.txt").unwrap();
	let mut input = String::new();
	f.read_to_string(&mut input).unwrap();
	let lines = input.trim().split('\n');

	let mut total = 0;
	for line in lines {
		let mut words = line.split(':');

		let _game_id = words.next().unwrap().chars()
			.filter(|x| x.is_digit(10)).collect::<String>()
			.parse::<u32>().unwrap();
		
		let rounds = words.next().unwrap().split(';');

		let mut max_blue = 0;
		let mut max_red	= 0;
		let mut max_green = 0;
		
		for round in rounds {

			let mut y = round.trim().split(',');

			while let Some(choice) = y.next() {
				let mut x = choice.trim().split(' ');
				let int_count =	x.next().unwrap().parse::<u32>().unwrap();  
				match x.next().unwrap().trim() {
					"blue" => {max_blue = max(int_count, max_blue)},
					"red" => {max_red = max(int_count, max_red)},
					"green" => {max_green = max(int_count, max_green)}
					c  => {panic!("unexpected colour {}", c);}
				}
			}
		}
		total += max_blue * max_red * max_green;
	}
	println!("{}", total);	

	
}
