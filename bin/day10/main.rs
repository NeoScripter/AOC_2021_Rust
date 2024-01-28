use std::collections::HashMap;

fn replace(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => unreachable!("invalid input"),
    }
}

fn part1(input: &str) -> (u64, Vec<&str>) {
    let mut score = 0;
    let scores = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let mut incomplete = Vec::new();
    for line in input.lines() {
        let mut stack = Vec::new();
        let mut ill_ch = None;

        for ch in line.chars() {
            match ch {
                '(' | '[' | '{' | '<' => stack.push(ch),
                ')' | ']' | '}' | '>' => {
                    if let Some(last) = stack.pop() {
                        if last != replace(ch) {
                        ill_ch = Some(ch);
                        break;
                        }
                    }
                },
                _ => {},
            }
        }

        if let Some(c) = ill_ch {
            score += scores.get(&c).unwrap();
        } else {
            incomplete.push(line);
        }
    }
    (score, incomplete)
}

fn part2(input: &str) -> u64 {
    let incomplete = part1(input).1;
    let scores = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    let mut points = Vec::new();
    for line in incomplete {
        let mut stack = Vec::new();
        for ch in line.chars() {
            match ch {
                '(' | '[' | '{' | '<' => stack.push(ch),
                ')' | ']' | '}' | '>' => {stack.pop();},
                _ => {},
            }
        }
        points.push(stack.iter().rev().map(|&c| *scores.get(&c).unwrap()).fold(0, |acc, k| acc * 5 + k));
    }
    points.sort_unstable();
    points[points.len() / 2]
}
fn main() {
    let input = include_str!("input10.txt");
    println!("{}", part2(input));
}