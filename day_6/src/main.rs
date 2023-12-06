use std::fs::File;
use std::io::Read;

fn calc_num_winning_options(time: u64, top_score: u64) -> u64 {
    let mut winning_count = 0;
    for hold_time in 0..=time {
        let distance = hold_time * (time - hold_time);
        if distance > top_score {
            winning_count += 1
        }
    }
    winning_count
}

fn part_1(input: &String) -> u64 {
    let mut parse_lines = input.lines().map(|line| {
        line.split(' ')
            .skip(1)
            .filter_map(|x| x.parse::<u64>().ok())
    });

    let time = parse_lines.next().unwrap();
    let distance = parse_lines.next().unwrap();

    time.zip(distance)
        .map(|(x, y)| calc_num_winning_options(x, y))
        .product()
}

fn part_2(input: &String) -> u64 {
    let mut parse_lines = input.lines().map(|line| {
        line.chars()
            .filter(|x| x.is_digit(10))
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
    });

    let time = parse_lines.next().unwrap();
    let top_score = parse_lines.next().unwrap();

    calc_num_winning_options(time, top_score)
}

fn main() {
    let mut f = File::open("data/input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}
