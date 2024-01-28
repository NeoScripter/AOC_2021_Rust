use std::collections::HashMap;

fn part1(input: &str) -> usize {
    input.lines().map(|line| {
        let (input, output) = line.split_once(" | ").unwrap();
        output.split_whitespace().filter(|&digit| {
            match digit.len() {
                2 | 4 | 3 | 7 => true,
                _ => false,
            }
        }).count()
    }).sum::<usize>()
}
fn count_frequencies() -> HashMap<u32, u32> {
    let (top, top_left, top_right, mid, bottom_left, bottom_right, bottom) = (8, 6, 8, 7, 4, 9, 7);

    let digit_frequencies = [
        top + top_left + top_right + bottom_left + bottom_right + bottom, // 0
        top_right + bottom_right,                                       // 1
        top + top_right + mid + bottom_left + bottom,                   // 2
        top + top_right + mid + bottom_right + bottom,                  // 3
        top_left + mid + top_right + bottom_right,                      // 4
        top + top_left + mid + bottom_right + bottom,                   // 5
        top + top_left + mid + bottom_left + bottom_right + bottom,     // 6
        top + top_right + bottom_right,                                 // 7
        top + top_left + top_right + mid + bottom_left + bottom_right + bottom, // 8
        top + top_left + top_right + mid + bottom_right + bottom,       // 9
    ];

    let mut frequencies = HashMap::new();
    for (digit, &frequency) in digit_frequencies.iter().enumerate() {
        frequencies.insert(frequency, digit as u32);
    }
    frequencies
}
// This is not my solution
fn part2(input: &str) -> u32 {
    let frequences = count_frequencies();
    let result = input.lines().map(|line| {
        let (left, right) = line.split_once(" | ").unwrap();
        let segment_frequencies = left
        .split_whitespace()
        .flat_map(|comb| comb.chars())
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_default() += 1;
            acc
        });
        right
        .split_whitespace()
        .map(|digit| {
            let digit_score = digit
            .chars()
            .map(|c| segment_frequencies.get(&c).unwrap())
            .sum();
        frequences.get(&digit_score).unwrap()
        }).fold(0, |acc, digit| acc * 10 + digit)
    }).sum();
    result
}
fn main() {
    let input = include_str!("input8.txt");
    println!("{}", part2(input));
}