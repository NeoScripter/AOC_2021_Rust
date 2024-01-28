fn transpose_rev<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters = v.into_iter().map(|n| n.into_iter()).collect::<Vec<_>>();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .filter_map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn make_decimal(v: Vec<u32>) -> u32 {
    v.iter().rev().fold((0, 1), |(acc, multp), &el| {
        (acc + el * multp, multp * 2)
    }).0
}
fn part1(input: &str) -> u32 {
    let d_report: Vec<Vec<u32>> = transpose_rev(input.lines()
        .map(|line| line.trim().chars().map(|c| c.to_digit(10)).collect())
        .collect());
    let (gamma_rate, epsilon_rate): (Vec<u32>, Vec<u32>) = d_report
    .iter()
    .map(|v| {
        let sum: usize = v.iter().map(|&x| x as usize).sum();
        if v.len() / 2 <= sum {
            (1, 0)
        } else {
            (0, 1)
        }
    })
    .unzip();

    make_decimal(gamma_rate) * make_decimal(epsilon_rate)
}
fn get_digit_bias(values: &[Vec<u32>], idx: usize) -> i64 {
    values.iter().fold(0, |acc, num| match num[idx] {
        0 => acc - 1,
        1 => acc + 1,
        _ => unreachable!("invalid input"),
    })
}
fn filter_candidates(candidates: Vec<Vec<u32>>, idx: usize, wanted: u32) -> Vec<Vec<u32>> {
    candidates
        .into_iter()
        .filter(|val| val[idx] == wanted)
        .collect()
}
fn to_u32(bits: &[u32]) -> u32 {
    bits.iter()
        .fold(0, |result, &bit| (result << 1) ^ bit as u32)
}
fn part2(input: &str) -> u32 {
    let report: Vec<Vec<u32>> = include_str!("input3.txt").lines()
        .map(|line| line.trim().chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

        let (o2, co2) = (0..report[0].len()).fold(
            (report.clone(), report.clone()),
            |(mut o2_candidates, mut co2_candidates), idx| {
                if o2_candidates.len() > 1 {
                    let bias = get_digit_bias(&o2_candidates, idx);
                    let wanted = if bias >= 0 { 1 } else { 0 };
                    o2_candidates = filter_candidates(o2_candidates, idx, wanted);
                }
                if co2_candidates.len() > 1 {
                    let bias = get_digit_bias(&co2_candidates, idx);
                    let wanted = if bias >= 0 { 0 } else { 1 };
                    co2_candidates = filter_candidates(co2_candidates, idx, wanted);
                }
                (o2_candidates, co2_candidates)
            },
        );

        to_u32(&o2[0]) * to_u32(&co2[0])
}
fn main() {
    let input = include_str!("input3.txt");
    println!("{}, {}", part1(input), part2(input));
}