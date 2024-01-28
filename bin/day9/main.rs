use std::collections::*;
use std::cmp::*;

fn find_basins(grid: &[Vec<u32>], x: usize, y: usize) -> u32 {
    let mut frontier: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    frontier.push_back((y, x));

    while let Some((i, j)) = frontier.pop_front() {
        if !visited.insert((i, j)) {
            continue;
        }

        let location = grid[i][j];
        if i > 0 && location < grid[i - 1][j] && grid[i - 1][j] != 9 {
            frontier.push_back((i - 1, j));
        }
        if i < grid.len() - 1 && location < grid[i + 1][j] && grid[i + 1][j] != 9 {
            frontier.push_back((i + 1, j));
        }
        if j > 0 && location < grid[i][j - 1] && grid[i][j - 1] != 9 {
            frontier.push_back((i, j - 1));
        }
        if j < grid[0].len() - 1 && location < grid[i][j + 1] && grid[i][j + 1] != 9 {
            frontier.push_back((i, j + 1));
        }
    }

    visited.len() as u32
}


fn check_neighbors(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let mut is_valid = true;
    let location = grid[y][x];
    if y > 0 {if location >= grid[y - 1][x] {is_valid = false}}
    if y < grid.len() - 1 {if location >= grid[y + 1][x] {is_valid = false}}
    if x > 0 {if location >= grid[y][x - 1] {is_valid = false}}
    if x < grid[0].len() - 1 {if location >= grid[y][x + 1] {is_valid = false}}
    is_valid
}
fn run(input: &str) -> (u32, u32) {
    let grid: Vec<Vec<u32>> = input
    .lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
    .collect();

    let mut sizes = Vec::new();
    let mut risk_level = 0;
    (0..grid.len()).for_each(|y| {
        (0..grid[0].len()).for_each(|x| {
            if check_neighbors(&grid, x, y) {
                risk_level += grid[y][x] + 1;
                sizes.push(find_basins(&grid, x, y))
            }
        })
    });
    sizes.sort_by(|a, b| b.cmp(a));
    println!("{:?}", sizes);
    let three_basins = sizes.iter().take(3).product();
    (risk_level, three_basins)
}
fn main() {
    let input = include_str!("input9.txt");
    println!("{}", run(input).1);
}