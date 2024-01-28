fn part1(input: &str) -> i32 {
    let positions: Vec<i32> = input.split(",").filter_map(|x| x.parse::<i32>().ok()).collect();
    let clone = positions.clone();
    positions.iter().map(|pos| {
        clone.iter().map(|fuel| (pos - fuel).abs()).sum()
    }).min().unwrap()
}
fn part2(input: &str) -> i32 {
    let positions: Vec<i32> = input.split(",").filter_map(|x| x.parse::<i32>().ok()).collect();
    let max_position = *positions.iter().max().unwrap();

    (0..=max_position).map(|target_pos| {
        positions.iter().map(|pos| (1..=(pos - target_pos).abs()).fold(0, |acc, num| {
            acc + num
        })).sum()
    }).min().unwrap()
}
fn main() {
    let input = include_str!("input7.txt");
    println!("{}", part2(input));
}