use num_integer::lcm;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("data/input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let mut lines_iter = input.lines();

    let mut moves = lines_iter.next().unwrap().chars().cycle();

    let mut network: HashMap<&str, [&str; 2]> = HashMap::new();

    for line in lines_iter.skip(1) {
        network.insert(&line[..3], [&line[7..=9], &line[12..=14]]);
    }

    let cur_pos: Vec<&str> = network
        .keys()
        .filter(|k| k.chars().nth(2).unwrap() == 'A')
        .copied()
        .collect();

    let mut common_divisors = vec![];

    for pos in cur_pos {
        let mut num_steps: u64 = 0;
        let mut junction = pos;
        while junction.chars().nth(2).unwrap() != 'Z' {
            let index = match moves.next().unwrap() {
                'L' => 0,
                'R' => 1,
                _ => panic!(),
            };
            junction = network.get(junction).unwrap()[index];
            num_steps += 1;
        }
        common_divisors.push(num_steps);
    }

    println!("{}", common_divisors.iter().fold(1, |a, b| lcm(a, *b)));
}
