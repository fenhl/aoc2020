use crate::prelude::*;

const DIR_FNS: [fn([usize; 2], [usize; 2]) -> Option<[usize; 2]>; 8] = [
    |[_, _], [x, y]| if y == 0 || x == 0 { None } else { Some([x - 1, y - 1]) },
    |[_, _], [x, y]| if y == 0 { None } else { Some([x, y - 1]) },
    |[w, _], [x, y]| if y == 0 || x == w - 1 { None } else { Some([x + 1, y - 1]) },
    |[_, _], [x, y]| if x == 0 { None } else { Some([x - 1, y]) },
    |[w, _], [x, y]| if x == w - 1 { None } else { Some([x + 1, y]) },
    |[_, h], [x, y]| if y == h - 1 || x == 0 { None } else { Some([x - 1, y + 1]) },
    |[_, h], [x, y]| if y == h - 1 { None } else { Some([x, y + 1]) },
    |[w, h], [x, y]| if y == h - 1 || x == w - 1 { None } else { Some([x + 1, y + 1]) },
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Floor,
    Empty,
    Occupied,
}

impl Cell {
    fn from_char(c: char) -> Result<Cell, char> {
        match c {
            '.' => Ok(Cell::Floor),
            'L' => Ok(Cell::Empty),
            '#' => Ok(Cell::Occupied),
            _ => Err(c),
        }
    }

    fn is_occupied(&self) -> bool {
        *self == Cell::Occupied
    }
}

fn num_occupied_direct_neighbors(grid: &[Vec<Cell>], [x, y]: [usize; 2]) -> usize {
    (if y == 0 { 0 } else {
        (if x == 0 { 0 } else if grid[y - 1][x - 1].is_occupied() { 1 } else { 0 })
        + if grid[y - 1][x].is_occupied() { 1 } else { 0 }
        + if grid[y - 1].get(x + 1).map_or(false, Cell::is_occupied) { 1 } else { 0 }
    })
    + (
        (if x == 0 { 0 } else if grid[y][x - 1].is_occupied() { 1 } else { 0 })
        + if grid[y].get(x + 1).map_or(false, Cell::is_occupied) { 1 } else { 0 }
    )
    + grid.get(y + 1).map_or(0, |row| {
        (if x == 0 { 0 } else if row[x - 1].is_occupied() { 1 } else { 0 })
        + if row[x].is_occupied() { 1 } else { 0 }
        + if row.get(x + 1).map_or(false, Cell::is_occupied) { 1 } else { 0 }
    })
}

fn num_occupied_visible_neighbors(grid: &[Vec<Cell>], [x, y]: [usize; 2]) -> usize {
    let size = [grid[0].len(), grid.len()];
    DIR_FNS.iter()
        .map(|dir_fn| {
            let mut coords = [x, y];
            while let Some([x, y]) = dir_fn(size, coords) {
                coords = [x, y];
                match grid[y][x] {
                    Cell::Floor => {}
                    Cell::Empty => return 0,
                    Cell::Occupied => return 1,
                }
            }
            0
        })
        .sum()
}

#[aoc_generator(day11)]
fn gen(input: &str) -> Vec<Vec<Cell>> {
    input.lines()
        .map(|line| line.chars().map(Cell::from_char).try_collect())
        .try_collect()
        .unwrap()
}

#[aoc(day11, part1)]
fn part1(input: &[Vec<Cell>]) -> usize {
    let mut prev = input.to_owned();
    loop {
        let next = prev.iter()
            .enumerate()
            .map(|(y, row)| row.iter()
                .enumerate()
                .map(|(x, cell)| match cell {
                    Cell::Floor => Cell::Floor,
                    Cell::Empty => if num_occupied_direct_neighbors(&prev, [x, y]) == 0 { Cell::Occupied } else { Cell::Empty },
                    Cell::Occupied => if num_occupied_direct_neighbors(&prev, [x, y]) >= 4 { Cell::Empty } else { Cell::Occupied },
                })
                .collect()
            )
            .collect();
        if next == prev { break } else { prev = next; }
    }
    prev.into_iter()
        .map(|row| row.into_iter().filter(Cell::is_occupied).count())
        .sum()
}

#[aoc(day11, part2)]
fn part2(input: &[Vec<Cell>]) -> usize {
    let mut prev = input.to_owned();
    loop {
        let next = prev.iter()
            .enumerate()
            .map(|(y, row)| row.iter()
                .enumerate()
                .map(|(x, cell)| match cell {
                    Cell::Floor => Cell::Floor,
                    Cell::Empty => if num_occupied_visible_neighbors(&prev, [x, y]) == 0 { Cell::Occupied } else { Cell::Empty },
                    Cell::Occupied => if num_occupied_visible_neighbors(&prev, [x, y]) >= 5 { Cell::Empty } else { Cell::Occupied },
                })
                .collect()
            )
            .collect();
        if next == prev { break } else { prev = next; }
    }
    prev.into_iter()
        .map(|row| row.into_iter().filter(Cell::is_occupied).count())
        .sum()
}
