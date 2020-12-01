use {
    std::num::ParseIntError,
    itertools::Itertools as _,
};

#[aoc_generator(day1)]
fn gen(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[u32]) -> u32 {
    let ((_, n1), (_, n2)) = input.iter()
        .enumerate()
        .cartesian_product(input.iter().enumerate())
        .filter(|((idx1, _), (idx2, _))| idx1 < idx2)
        .filter(|((_, n1), (_, n2))| **n1 + **n2 == 2020)
        .exactly_one().unwrap();
    n1 * n2
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> u32 {
    let (((_, n1), (_, n2)), (_, n3)) = input.iter()
        .enumerate()
        .cartesian_product(input.iter().enumerate())
        .filter(|((idx1, _), (idx2, _))| idx1 < idx2)
        .cartesian_product(input.iter().enumerate())
        .filter(|(((_, _), (idx2, _)), (idx3, _))| idx2 < idx3)
        .filter(|(((_, n1), (_, n2)), (_, n3))| **n1 + **n2 + **n3 == 2020)
        .exactly_one().unwrap();
    n1 * n2 * n3
}
