use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum CardType {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl CardType {
    fn from_char(c: &char) -> Self {
        match c {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            // 'J' => Self::J (part 1)
            'T' => Self::T,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            'J' => Self::Joker,
            _ => panic!(),
        }
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn from_str(input: &str) -> &Self {
        let mut card_counts: HashMap<char, u32> = HashMap::new();
        let mut num_jokers = 0;

        for c in input.chars() {
            if c == 'J' {
                num_jokers += 1;
            } else {
                match card_counts.get_mut(&c) {
                    Some(count) => {
                        *count += 1;
                    }
                    _ => {
                        card_counts.insert(c, 1);
                    }
                }
            }
        }

        let mut counts: Vec<&u32> = card_counts.values().collect();
        counts.sort();
        counts.reverse();

        if num_jokers == 5 {
            return &Self::FiveOfKind;
        }

        let max_count = counts[0];

        if max_count + num_jokers == 5 {
            &Self::FiveOfKind
        } else if max_count + num_jokers == 4 {
            &Self::FourOfKind
        } else if max_count + num_jokers + counts[1] == 5 {
            &Self::FullHouse
        } else if max_count + num_jokers == 3 {
            &Self::ThreeOfKind
        } else if max_count + num_jokers + counts[1] == 4 {
            &Self::TwoPair
        } else if max_count + num_jokers == 2 {
            &Self::OnePair
        } else {
            &Self::HighCard
        }

        // Part 1 Scoring
        //
        // match max_count {
        //     5 => &Self::FiveOfKind,
        //     4 => &Self::FourOfKind,
        //     3 => {
        //         if *counts[1] == 2 {
        //             &Self::FullHouse
        //         } else {
        //             &Self::ThreeOfKind
        //         }
        //     }
        //     2 => {
        //         if *counts[1] == 2 {
        //             &Self::TwoPair
        //         } else {
        //             &Self::OnePair
        //         }
        //     }
        //     1 => &Self::HighCard,
        //     _ => panic!(),
        // }
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
struct Hand<'a> {
    hand_type: &'a HandType,
    cards: Vec<CardType>,
    bid: u32,
}

fn main() {
    let mut f = File::open("data/input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let mut hands = vec![];
    for hand in input.lines() {
        let mut hand_iter = hand.split(' ');
        let str_cards = hand_iter.next().unwrap();
        let cards: Vec<CardType> = str_cards.chars().map(|c| CardType::from_char(&c)).collect();
        let bid = hand_iter.next().unwrap().parse::<u32>().unwrap();
        let hand_type = HandType::from_str(&str_cards);
        hands.push(Hand {
            hand_type,
            cards,
            bid,
        });
    }
    hands.sort();
    hands.reverse();

    println!(
        "{}",
        hands
            .iter()
            .enumerate()
            .map(|(i, h)| h.bid * (i + 1) as u32)
            .sum::<u32>()
    );
}
