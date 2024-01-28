use std::str::FromStr;
fn part1(input: &str) -> u32 {
    let (h, d) = include_str!("input2.txt").lines().fold((0, 0), |(hor_pos, depth), line| {
        let (direction, unit) = line.trim().split_once(" ").unwrap();
        let unit = u32::from_str(unit).unwrap();
        match direction {
            "forward" => (hor_pos + unit, depth),
            "down" => (hor_pos, depth + unit),
            "up" => (hor_pos, depth.saturating_sub(unit)),
            _ => panic!("Invalid input"),
        }
    });
    h * d
}
fn part2(input: &str) -> u32 {
    let (h, d, a) = include_str!("input2.txt").lines().fold((0, 0, 0), |(hor_pos, depth, aim), line| {
        let (direction, unit) = line.trim().split_once(" ").unwrap();
        let unit = u32::from_str(unit).unwrap();
        match direction {
            "forward" => (hor_pos + unit, depth + aim * unit, aim),
            "down" => (hor_pos, depth, aim + unit),
            "up" => (hor_pos, depth, aim.saturating_sub(unit)),
            _ => panic!("Invalid input"),
        }
    });
    h * d
}
fn main() {
    let input = include_str!("input2.txt");
    println!("{}, {}", part1(input), part2(input));
}