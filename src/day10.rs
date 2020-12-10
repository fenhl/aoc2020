use {
    std::iter,
    crate::prelude::*,
};

fn combinations(low: u32, adapters: &[u32], high: u32) -> u64 {
    match adapters.len() {
        0 => if high - low <= 3 { 1 } else { 0 },
        1 => {
            if high - adapters[0] <= 3 && adapters[0] - low <= 3 {
                if high - low <= 3 { 2 } else { 1 }
            } else {
                0
            }
        }
        2 => {
            let diffs = (adapters[0] - low, adapters[1] - adapters[0], high - adapters[1]);
            if diffs.0 > 3 || diffs.1 > 3 || diffs.2 > 3 { 0 } else {
                match diffs {
                    (1, 1, 1) => 4,
                    (1, 1, 2) | (1, 2, 1) | (2, 1, 1) => 3,
                    (1, 1, 3) | (1, 2, _) | (2, 1, _) | (_, 1, 2) | (_, 2, 1) | (3, 1, 1) => 2,
                    (_, _, _) => 1,
                }
            }
        }
        len => {
            let original_middle = len / 2;
            let mut middle = original_middle;
            loop {
                let diffs = (adapters[middle] - adapters[middle - 1], adapters[middle + 1] - adapters[middle]);
                match diffs {
                    (1, 1) => {
                        // not a suitable midpoint, try elsewhere
                        if middle == original_middle {
                            middle -= 1;
                            if middle == 0 { break }
                        } else if middle < original_middle {
                            middle = 2 * original_middle - middle;
                            if middle >= len - 1 { break }
                        } else {
                            if 2 * original_middle - middle <= 1 { break }
                            middle = 2 * original_middle - middle - 1;
                        }
                    }
                    (1, 2) | (2, 1) => {
                        // the middle adapter can be used or skipped
                        return combinations(low, &adapters[0..middle - 1], adapters[middle - 1]) * combinations(adapters[middle + 1], &adapters[middle + 2..len], high)
                        + combinations(low, &adapters[0..middle], adapters[middle]) * combinations(adapters[middle], &adapters[middle + 1..len], high)
                    }
                    (_, _) => {
                        // the middle adapter is required
                        return combinations(low, &adapters[0..middle], adapters[middle]) * combinations(adapters[middle], &adapters[middle + 1..len], high)
                    }
                }
            }
            // no suitable midpoints found, i.e. all adapters are next to each other (the only suitable midpoints, if any, are at the start or end)
            // combinations without first adapter
            (if adapters[1] - low <= 3 { combinations(low, &adapters[1..len], high) } else { 0 })
            // combinations with first adapter
            + combinations(adapters[0], &adapters[1..len], high)
        }
    }
}

#[aoc_generator(day10)]
fn gen(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines()
        .map(str::parse)
        .try_collect()
}

#[aoc(day10, part1)]
fn part1(input: &[u32]) -> u32 {
    let mut ratings = input.to_owned();
    ratings.sort();
    let device_rating = ratings.last().copied().unwrap_or_default() + 3;
    let (num_ones, num_threes) = iter::once(0)
        .chain(ratings)
        .chain(iter::once(device_rating))
        .tuple_windows()
        .fold((0, 0), |(num_ones, num_threes), (first, second)| match second - first {
            1 => (num_ones + 1, num_threes),
            3 => (num_ones, num_threes + 1),
            _ => (num_ones, num_threes),
        });
    num_ones * num_threes
}

#[aoc(day10, part2)]
fn part2(input: &[u32]) -> u64 {
    let mut ratings = input.to_owned();
    ratings.sort();
    let device_rating = ratings.last().copied().unwrap_or_default() + 3;
    combinations(0, &ratings, device_rating)
}

#[test]
fn part2_tests() {
    assert_eq!(part2(&[]), 1);
    assert_eq!(part2(&[3]), 1);
    assert_eq!(part2(&[3, 4, 6]), 2);
    assert_eq!(part2(&[1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19]), 8);
    assert_eq!(part2(&[28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3]), 19208);
}
