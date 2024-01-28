#[derive(Debug)]
struct School {
    state: u32,
    children: Vec<School>,
    count: u32,
}

impl School {
    fn new(state: u32) -> School {
        School {
            state,
            children: Vec::new(),
            count: 1,
        }
    }
    fn sum_counts(&self) -> u32 {
        self.children.iter().map(|child| child.sum_counts()).sum::<u32>() + self.count
    }

    fn reset(&mut self) {
        for child in &mut self.children {
            child.reset();
        }
        if self.state > 0 {
            self.state -= 1;
        } else {
            self.state = 6;
            self.children.push(School::new(8));
        }
    }
}

fn part1(input: &str, days: u32) -> u32 {
    let list: Vec<u32> = input.split(',')
                              .map(|x| x.parse().unwrap())
                              .collect();
    let mut schools: Vec<School> = list.into_iter()
                                       .map(School::new)
                                       .collect();

    for _ in 0..days {
        for school in &mut schools {
            school.reset();
        }
    }
    schools.iter().map(School::sum_counts).sum()
}
#[derive(Default)]
struct Population([u64; 9]);

impl Population {
    fn tick(&mut self) {
        let reproducing = self.0[0];
        self.0.rotate_left(1);
        self.0[6] += reproducing;
    }
    fn size(&self) -> u64 {
        self.0.iter().sum()
    }
    fn initial_population(timers: &[u32]) -> Population {
        timers.iter().fold(Population::default(), |mut acc, &num| {
            acc.0[num as usize] += 1;
            acc
        })
    }
}
fn part2(input: &str, days: u32) -> u64 {
    let timers = input
        .trim()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut population = Population::initial_population(&timers);

    for _ in 0..days {
        population.tick();
    }

    population.size()
}
fn main() {
    let input = include_str!("input6.txt");
    println!("{}, {}", part1(input, 80), part2(input, 256));
}