use itertools::Itertools;
use std::collections::HashMap;
use nom::{
    bytes::complete::{take_until, tag},
    combinator::rest,
    sequence::tuple,
    IResult,
};

fn split_once_nom<'a>(input: &'a str, flag: &'a str) -> IResult<&'a str, (&'a str, char)> {
    let (rest, (st, _, end)) = tuple((take_until(flag), tag(flag), rest))(input)?;
    Ok((rest, (st, end.chars().next().unwrap())))
}

#[derive(Debug, Clone)]
struct Polymer {
    tmpl: HashMap<(char, char), usize>,
    rules: HashMap<(char, char), char>,
    char_map: HashMap<char, usize>
}

impl Polymer {
    fn new() -> Self {
        Self { tmpl: HashMap::new(), rules: HashMap::new(), char_map: HashMap::new() }
    }
    fn step(&mut self, steps: usize) {
        for _ in 0..steps {
            let mut output = HashMap::new();

            for (&pair, &count) in &self.tmpl {
                if let Some(&added) = self.rules.get(&pair) {
                    *output.entry((pair.0, added)).or_insert(0) += count;
                    *output.entry((added, pair.1)).or_insert(0) += count;
                    *self.char_map.entry(added).or_insert(0) += count;
                } else {
                    output.insert(pair, count);
                }
            }

            self.tmpl = output;
        }
    }
    fn find_minmax(&mut self) -> usize {
        let (min, max) = self.char_map.values().minmax().into_option().unwrap();
        max - min
    }
}

fn parse_input() -> Polymer {
    let input = include_str!("input14.txt");
    let (temp, cmds) = input.split_once("\r\n\r\n").unwrap();

    let mut plm = Polymer::new();
    let chars: Vec<char> = temp.chars().collect();

    for win in chars.windows(2) { *plm.tmpl.entry((win[0], win[1])).or_insert(0) += 1 }
    for &c in &chars { *plm.char_map.entry(c).or_insert(0) += 1 }

    plm.rules = cmds.lines().map(|l| {
        let (_, (key, v)) = split_once_nom(l, " -> ").unwrap();
        let key: Vec<char> = key.chars().collect();
        ((key[0], key[1]), v)
    }).collect();

    plm
}
fn part1() -> usize {
    let mut plm = parse_input();
    plm.step(10);
    plm.find_minmax()
}

fn part2() -> usize {
    let mut plm = parse_input();
    plm.step(40);
    plm.find_minmax()
}

fn main() {
    println!("{}", part2());
}