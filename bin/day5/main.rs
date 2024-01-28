use std::collections::HashMap;
use regex::Regex;

fn part1(input: &str) -> usize {
    let mut map = HashMap::new();
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    
    for line in input.lines() {
        for cap in re.captures_iter(line) {
            let x1: usize = cap[1].parse().unwrap();
            let y1: usize = cap[2].parse().unwrap();
            let x2: usize = cap[3].parse().unwrap();
            let y2: usize = cap[4].parse().unwrap();

            let (x_start, x_end) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            let (y_start, y_end) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

            if y1 == y2 {
                for val in x_start..=x_end {
                    *map.entry((val, y1)).or_insert(0) += 1;
                }
            }

            if x1 == x2 {
                for val in y_start..=y_end {
                    *map.entry((x1, val)).or_insert(0) += 1;
                }
            }
        }
    }

    map.values().filter(|&&x| x > 1).count()
}
fn part2(input: &str) -> usize {
    let mut map = HashMap::new();

    for line in input.lines() {
        let nums: Vec<i32> = line.split(|c| c == ' ' || c == ',')
                                 .filter_map(|num| num.parse().ok())
                                 .collect();
        let (mut x1, mut y1, x2, y2) = (nums[0], nums[1], nums[2], nums[3]);

        while (x1, y1) != (x2, y2) {
            *map.entry((x1, y1)).or_insert(0) += 1;

            if x1 < x2 { x1 += 1; }
            else if x1 > x2 { x1 -= 1; }

            if y1 < y2 { y1 += 1; }
            else if y1 > y2 { y1 -= 1; }
        }

        *map.entry((x2, y2)).or_insert(0) += 1;
    }

    map.values().filter(|&&x| x > 1).count()
}

fn main() {
    let input = include_str!("input5.txt");
    println!("{}, {}", part1(input), part2(input));
}