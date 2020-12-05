use std::collections::BTreeSet;

#[aoc_generator(day5)]
fn gen(input: &str) -> Vec<u16> {
    input.lines()
        .map(|line| u16::from_str_radix(&line.chars().map(|c| match c {
            'F' | 'L' => '0',
            'B' | 'R' => '1',
            _ => panic!(),
        }).collect::<String>(), 2).unwrap())
        .collect()
}

#[aoc(day5, part1)]
fn part1(input: &[u16]) -> u16 {
    *input.iter().max().unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &[u16]) -> u16 {
    let all_seats = input.iter().copied().collect::<BTreeSet<_>>();
    (0..).find(|n| all_seats.contains(&(n - 1)) && !all_seats.contains(n) && all_seats.contains(&(n + 1))).unwrap()
}
