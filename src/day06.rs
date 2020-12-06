use {
    std::{
        collections::HashSet,
        convert::identity,
    },
    hashbag::HashBag,
};

#[aoc_generator(day6)]
fn gen(input: &str) -> Vec<Vec<Vec<char>>> {
    input.split("\n\n")
        .map(|paragraph| paragraph.lines().map(|line| line.chars().collect()).collect())
        .collect()
}

#[aoc(day6, part1)]
fn part1(input: &[Vec<Vec<char>>]) -> usize {
    input.iter()
        .map(|group| group.iter().flat_map(identity).collect::<HashSet<_>>().len())
        .sum()
}

#[aoc(day6, part2)]
fn part2(input: &[Vec<Vec<char>>]) -> usize {
    input.iter()
        .map(|group| {
            let mut counts = HashBag::new();
            for person in group {
                for c in person {
                    counts.insert(c);
                }
            }
            counts.into_iter().filter(|(_, yes_answers)| *yes_answers == group.len()).count()
        })
        .sum()
}
