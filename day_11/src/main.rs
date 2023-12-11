use std::cmp::max;
use std::cmp::min;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("data/input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let mut points = vec![];

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                points.push((row, col));
            }
        }
    }

    let mut empty_rows = vec![];
    for row in 0..input.lines().count() {
        if points.iter().all(|(y, _)| *y != row) {
            empty_rows.push(row);
        }
    }

    let mut empty_cols = vec![];
    for col in 0..input.lines().next().unwrap().len() {
        if points.iter().all(|(_, x)| *x != col) {
            empty_cols.push(col);
        }
    }

    let mut total_dist = 0;
    for (y1, x1) in &points {
        for (y2, x2) in &points {
            let low_x = min(x1, x2);
            let high_x = max(x1, x2);
            let low_y = min(y1, y2);
            let high_y = max(y1, y2);

            total_dist += (high_x - low_x) + (high_y - low_y);

            for r in &empty_rows {
                if low_y < r && r < high_y {
                    total_dist += 1000000 - 1;
                }
            }
            for c in &empty_cols {
                if low_x < c && c < high_x {
                    total_dist += 1000000 - 1;
                }
            }
        }
    }
    println!("{}", total_dist / 2);
}
