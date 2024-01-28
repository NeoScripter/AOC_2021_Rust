use std::{
    str::FromStr,
    collections::{HashMap, HashSet},
};

#[derive(Debug, Copy, Clone)]
enum Fold {
    Y(usize),
    X(usize),
}

impl FromStr for Fold {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once('=').unwrap();
        let right = right.parse::<usize>().unwrap();
        match left {
            "y" => Ok(Fold::Y(right)),
            _ => Ok(Fold::X(right)),
        }
    }
}
#[derive(Debug, Clone)]
struct Sheet {
    coord: HashSet<(usize, usize)>,
    itrs: Vec<Fold>,
}

impl Sheet {
    fn new() -> Self {
        Self {
            coord: HashSet::new(),
            itrs: Vec::new(),
        }
    }
    fn fold(&mut self) {
        if let Some(itr) = self.itrs.pop() {
            match itr {
                Fold::Y(f) => {
                    let modified_coords: HashSet<(usize, usize)> = self.coord.iter()
                    .map(|&(x, y)| {
                        if y > f {
                            (x, y - (y - f) * 2)
                        } else {
                            (x, y)
                        }
                    })
                    .collect();
                    self.coord = modified_coords;
                },
                Fold::X(f) => {
                    let modified_coords: HashSet<(usize, usize)> = self.coord.iter()
                    .map(|&(x, y)| {
                        if x > f {
                            (x - (x - f) * 2, y)
                        } else {
                            (x, y)
                        }
                    })
                    .collect();
                    self.coord = modified_coords;
                },
            }
        }
    }
    fn make_grid(&self) {
        let max_x = self.coord.iter().max_by_key(|(x, y)| x).unwrap().0 + 1;
        let max_y = self.coord.iter().max_by_key(|(x, y)| y).unwrap().1 + 1;
        let mut grid = vec![vec!['.'; max_x]; max_y];
        (0..max_y).for_each(|y| {
            (0..max_x).for_each(|x| {
                if self.coord.contains(&(x, y)) {
                    grid[y][x] = '#';
                }
            })
        });
        let mut string = String::new();
        for y in grid.iter() {
            for &x in y.iter() {
                string.push(x)
            }
            string.push_str("\r\n");
        }
        println!("{}", string);
    }
}

fn parse_input() -> Sheet {
    let input = include_str!("input13.txt");
    let mut sheet = Sheet::new();
    let (cds, irs) = input.split_once("\r\n\r\n").unwrap();
    sheet.coord = cds.lines().map(|line| {
        let (x, y) = line.split_once(',').unwrap();
        (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
    }).collect();
    sheet.itrs = irs.lines().map(|line| {
        let (_, intr) = line.rsplit_once(' ').unwrap();
        intr.parse().unwrap()
    }).collect();
    sheet.itrs.reverse();
    sheet
}

fn part1() -> usize {
    let mut sheet = parse_input();
    sheet.fold();
    sheet.coord.len()
}

fn part2() {
    let mut sheet = parse_input();
    while !sheet.itrs.is_empty() {
        sheet.fold();
    }
    sheet.make_grid()
}
fn main() {
    part2();
}
