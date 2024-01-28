use std::{
    cmp::Reverse,
    collections::{HashSet, BinaryHeap},
};

#[derive(Debug, Clone)]
struct Cave {
    cavern: Vec<Vec<u32>>,
}

impl Cave {
    fn nghs(&self, y: usize, x: usize) -> Vec<(usize, usize)> {
        let mut nghs = Vec::new();
        if y > 0 { nghs.push((y - 1, x)) }
        if x > 0 { nghs.push((y, x - 1)) }
        if x < self.cavern[0].len() - 1 { nghs.push((y, x + 1)) }
        if y < self.cavern.len() - 1 { nghs.push((y + 1, x)) }
        nghs
    }
    fn find_path(&self) -> u32 {
        let mut cache = HashSet::new();
        let start = (0, 0);
        let end = (self.cavern.len() - 1, self.cavern[0].len() - 1);
        let mut q = BinaryHeap::new();
        q.push((Reverse(0), start));
        while let Some((Reverse(risk), (y, x))) = q.pop() {
            if (y, x) == end { return risk }
            if !cache.insert((y, x)) { continue }
            for n in self.nghs(y, x) {
                q.push((Reverse(risk + self.cavern[n.0][n.1]), (n.0, n.1)))
            }
        }
        0
    }
    fn expand_cave(&mut self) {
        let mut cave = self.cavern.clone();
        cave = (0..(5 * cave.len())).map(|x| (0..(5 * cave[0].len())).map(|y| {
            let cost = cave[x % cave.len()][y % cave[0].len()]
            + (x / cave.len()) as u32
            + (y / cave[0].len()) as u32;
            if cost < 10 {cost} else {cost - 9}
        }).collect::<Vec<_>>()
        ).collect::<Vec<_>>();
        self.cavern = cave;
    }
}

fn create_cave() -> Cave {
    let input = include_str!("input15.txt");
    let cavern = input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
    let cave = Cave { cavern };
    cave
}
fn part1() -> u32 {
    let cave = create_cave();
    cave.find_path()
}

fn part2() -> u32 {
    let mut cave = create_cave();
    cave.expand_cave();
    //println!("{}", cave);
    cave.find_path()
}
fn main() {
    println!("{}", part2());
}