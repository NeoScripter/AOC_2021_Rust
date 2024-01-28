use itertools::Itertools;
#[derive(Debug, Clone, PartialEq, Eq)]
struct Board(Vec<Vec<u32>>);

impl FromIterator<Vec<u32>> for Board {
    fn from_iter<I: IntoIterator<Item = Vec<u32>>>(iter: I) -> Self {
        let vectors: Vec<Vec<u32>> = iter.into_iter().collect();
        Board(vectors)
    }
}

fn parse(input: &str) -> Option<(Vec<u32>, Vec<Board>)> {
    let (random_order, rest) = input.split_once("\r\n\r\n")?;
    let random_order = random_order
        .split(',')
        .filter_map(|num| num.parse().ok())
        .collect::<Vec<u32>>();
    let mut all_combinations: Vec<Board> = Vec::new();
    for board in rest.split("\r\n\r\n") {
        let numbers = board
            .trim()
            .split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect::<Vec<u32>>();

        let rows: Vec<_> = numbers.chunks(5).map(|chunk| chunk.to_vec()).collect();

        let columns = rows.chunks(5)
        .flat_map(|chunk| (0..5).map(|i| chunk.iter().map(|nums| nums[i]).collect::<Vec<u32>>()))
        .collect::<Vec<_>>();

        let interleaved: Board = rows.into_iter().interleave(columns.into_iter()).collect();
        all_combinations.push(interleaved);

    }
    Some((random_order, all_combinations))
}
fn part1(input: &str) -> Option<u32> {
    let (order, mut combs) = parse(input)?;

    for &num in &order {
        for board in &mut combs {
            for vec in &mut board.0 {
                vec.retain(|&x| x != num);
            }
            
            if board.0.iter().any(|vec| vec.is_empty()) {
                let board_sum: u32 = board.0.iter().flatten().copied().sum();
                return Some(board_sum / 2 * num);
            }
        }
    }
    None
}
fn part2(input: &str) -> Option<u32> {
    let (order, mut combs) = parse(input)?;
    let mut remaining_boards = combs.clone();

    for (idx, &num) in order.iter().enumerate() {
        combs.iter_mut().for_each(|board| {
            board.0.iter_mut().for_each(|vec| {
                vec.retain(|&x| x != num);
            });
        });

        remaining_boards.clear();
        remaining_boards.extend(
            combs.iter()
                .filter(|&board| !board.0.iter().any(|vec| vec.is_empty()))
                .cloned()
        );

        if remaining_boards.len() == 1 {
            let next_numbers = &order[(idx + 1)..];
            for &next_num in next_numbers {
                remaining_boards[0].0.iter_mut().for_each(|vec| {
                    vec.retain(|&x| x != next_num);
                });
                if remaining_boards[0].0.iter().any(|vec| vec.is_empty()) {
                    let board_sum: u32 = remaining_boards[0].0.iter().flatten().copied().sum();
                    return Some(board_sum / 2 * next_num);
                }
            }
        }
    }
    None
}


fn main() {
    let input = include_str!("input4.txt");
    println!("{}, {}", part1(input).unwrap(), part2(input).unwrap());
}