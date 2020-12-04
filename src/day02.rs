use {
    std::ops::RangeInclusive,
    crate::prelude::*,
};

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new("^([0-9]+)-([0-9]+) (.?): (.*)$").unwrap();
}

#[aoc_generator(day2)]
fn gen(input: &str) -> Vec<(RangeInclusive<usize>, char, String)> {
    input.lines()
        .map(|line| {
            let captures = LINE_REGEX.captures(line).unwrap();
            (captures[1].parse().unwrap()..=captures[2].parse().unwrap(), captures[3].parse().unwrap(), captures[4].to_owned())
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[(RangeInclusive<usize>, char, String)]) -> usize {
    input.iter()
        .filter(|(range, letter, password)| range.contains(&password.matches(*letter).count()))
        .count()
}

#[aoc(day2, part2)]
fn part2(input: &[(RangeInclusive<usize>, char, String)]) -> usize {
    input.iter()
        .filter(|(range, letter, password)| (password.chars().nth(*range.start() - 1).unwrap() == *letter) ^ (password.chars().nth(*range.end() - 1).unwrap() == *letter))
        .count()
}
