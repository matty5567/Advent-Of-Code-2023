use std::{collections::HashMap, fs::File, io::Read};

fn main() {
    let mut f = File::open("data/input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let map: Vec<&str> = input.lines().collect();

    let width = map[0].len();
    let height = map.len();

    let mut s_row = 0;
    let mut s_col = 0;
    for (row, line) in map.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == 'S' {
                s_row = row;
                s_col = col;
            }
        }
    }

    let mut seen: HashMap<(i32, i32), u32> = HashMap::new();
    let mut to_visit: Vec<(i32, i32, u32)> = vec![(s_row as i32, s_col as i32, 0)];
    seen.insert((s_row as i32, s_col as i32), 0);

    while let Some((cur_row, cur_col, dist)) = to_visit.pop() {
        seen.insert((cur_row, cur_col), dist);
        let cur_char = map[cur_row as usize].chars().nth(cur_col as usize).unwrap();
        let chars_to_check = match cur_char {
            '-' => vec![(0, -1), (0, 1)],
            '|' => vec![(-1, 0), (1, 0)],
            'J' => vec![(0, -1), (-1, 0)],
            '7' => vec![(0, -1), (1, 0)],
            'F' => vec![(0, 1), (1, 0)],
            'L' => vec![(0, 1), (-1, 0)],
            'S' => vec![(-1, 0), (1, 0), (0, 1), (0, -1)],
            _ => panic!(),
        };

        for (r, c) in chars_to_check {
            let new_col = cur_col + c;
            let new_row = cur_row + r;
            let new_dist = dist + 1;

            if 0 <= new_row && new_row < height as i32 {
                if 0 <= new_col && new_col < width as i32 {
                    let seen_dist = seen.get(&(new_row, new_col)).unwrap_or(&u32::MAX);
                    if new_dist < *seen_dist {
                        let valid =
                            match map[new_row as usize].chars().nth(new_col as usize).unwrap() {
                                '|' => c == 0,
                                '-' => r == 0,
                                'L' => (r == 1 && c == 0) | (r == 0 && c == -1),
                                'J' => (r == 1 && c == 0) | (r == 0 && c == 1),
                                '7' => (r == -1 && c == 0) | (r == 0 && c == 1),
                                'F' => (r == -1 && c == 0) | (r == 0 && c == -1),
                                _ => false,
                            };
                        if valid {
                            to_visit.push((new_row, new_col, new_dist));
                        }
                    }
                }
            }
        }
    }
    println!("max dist: {}", seen.values().max().unwrap());

    let mut num_inside = 0;
    let mut inside = vec![];
    for (row, line) in map.iter().enumerate() {
        for (col, _) in line.chars().enumerate() {
            if !seen.contains_key(&(row as i32, col as i32)) {
                let mut num_crossed = 0;
                let mut below = None;
                for (pos, c) in line.chars().enumerate().skip(col + 1) {
                    if seen.contains_key(&(row as i32, pos as i32)) {
                        match c {
                            'F' => {
                                below = Some(true);
                            }
                            'L' => {
                                below = Some(false);
                            }
                            '|' => {
                                num_crossed += 1;
                            }
                            '7' => {
                                if below == Some(false) {
                                    num_crossed += 1;
                                    below = None;
                                }
                            }
                            'J' => {
                                if below == Some(true) {
                                    num_crossed += 1;
                                    below = None;
                                }
                            }
                            'S' => {
                                if (seen.contains_key(&(row as i32 + 1, pos as i32))
                                    && below == Some(false))
                                    || (seen.contains_key(&(row as i32 - 1, pos as i32))
                                        && below == Some(true))
                                {
                                    num_crossed += 1;
                                    below = None;
                                }
                            }
                            '-' => {}
                            x => {
                                println!("{}", x);
                            }
                        }
                    }
                }
                if num_crossed % 2 != 0 {
                    num_inside += 1;
                    inside.push((row, col));
                }
            }
        }
    }

    // for (row, line) in map.iter().enumerate() {
    //     for (col, c) in line.chars().enumerate() {
    //         if inside.contains(&(row, col)) {
    //             print!("{}", 'I');
    //         } else if seen.contains_key(&(row as i32, col as i32)) {
    //             print!("{}", c);
    //         } else {
    //             print!("{}", "O");
    //         }
    //     }
    //     print!("\n");
    // }

    println!("num inside {}", num_inside);
}
