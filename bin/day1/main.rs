use itertools::Itertools;
fn part1(input: &str) -> usize {
    let result = input.lines().tuple_windows().filter(|(left, right)| (left.trim().parse::<u32>().unwrap() < right.trim().parse::<u32>().unwrap())).count();
    result
}
fn part2(input: &str) -> usize {
    let result = input.lines()
    .map(|line| line.trim().parse::<u32>().unwrap())
    .tuple_windows::<(_,_,_)>()
    .map(|(a, b, c)| a + b + c)
    .tuple_windows()
    .filter(|&(left, right)| left < right)
    .count();
    result
}
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input = include_str!("input1.txt");
    println!("{}, {}", part1(input), part2(input));

    Ok(())
}