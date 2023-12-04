use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

fn get_copies(card_scores: HashMap<u32, u32>) -> u32 {
    let cards_to_scratch: Vec<u32> = card_scores.keys().map(|x| x.to_owned()).collect();

    fn _get_copies(num: u32, card_scores: &HashMap<u32, u32>) -> u32 {
        let score = card_scores.get(&num).unwrap().to_owned();
        return 1
            + (0..score)
                .map(|x| _get_copies(num + x + 1, &card_scores))
                .sum::<u32>();
    }
    cards_to_scratch
        .iter()
        .map(|x| _get_copies(*x, &card_scores))
        .sum()
}

fn main() {
    let mut f = File::open("data/input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let mut card_scores: HashMap<u32, u32> = HashMap::new();

    let mut total_score = 0;
    for line in input.lines() {
        let mut x = line.split(":");

        let card_num = x
            .next()
            .unwrap()
            .chars()
            .filter(|x| x.is_digit(10))
            .collect::<String>()
            .parse::<u32>()
            .unwrap();

        let mut scratch_nums = x.next().unwrap().split("|");

        let mut winning_numbers: HashSet<u32> = HashSet::new();
        let mut found_numbers: HashSet<u32> = HashSet::new();

        scratch_nums
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .for_each(|num| {
                if num != "" {
                    winning_numbers.insert(num.parse::<u32>().unwrap());
                }
            });

        scratch_nums
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .for_each(|num| {
                if num != "" {
                    found_numbers.insert(num.parse::<u32>().unwrap());
                }
            });

        let my_winnings: Vec<&u32> = winning_numbers.intersection(&found_numbers).collect();

        card_scores.insert(card_num, my_winnings.len() as u32);
        if my_winnings.len() > 0 {
            total_score += 2u32.pow(my_winnings.len() as u32 - 1);
        }
    }
    println!(
        "part 1: {}, part 2: {}",
        total_score,
        get_copies(card_scores)
    );
}
