use crate::prelude::*;

const PREAMBLE_LEN: usize = 25;

#[aoc_generator(day9)]
fn gen(input: &str) -> Result<Vec<u64>, ParseIntError> {
    input.lines()
        .map(u64::from_str)
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[u64]) -> u64 {
    for (i, &line) in input.iter().enumerate().skip(PREAMBLE_LEN) {
        if !input[i - PREAMBLE_LEN..i].iter()
            .enumerate()
            .cartesian_product(input[i - PREAMBLE_LEN..i].iter().enumerate())
            .filter(|((idx1, _), (idx2, _))| idx1 < idx2)
            .any(|((_, n1), (_, n2))| n1 + n2 == line)
        { return line }
    }
    panic!()
}

#[aoc(day9, part2)]
fn part2(input: &[u64]) -> u64 {
    for (i, &line) in input.iter().enumerate().skip(PREAMBLE_LEN) {
        if !input[i - PREAMBLE_LEN..i].iter()
            .enumerate()
            .cartesian_product(input[i - PREAMBLE_LEN..i].iter().enumerate())
            .filter(|((idx1, _), (idx2, _))| idx1 < idx2)
            .any(|((_, n1), (_, n2))| n1 + n2 == line)
        {
            for start in 0..=input.len() - 1 {
                for end in start + 1..input.len() {
                    if input[start..=end].iter().copied().sum::<u64>() == line {
                        return input[start..=end].iter().copied().min().unwrap() + input[start..=end].iter().copied().max().unwrap()
                    }
                }
            }
            panic!()
        }
    }
    panic!()
}
