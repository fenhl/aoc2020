#[derive(Clone, Copy, PartialEq, Eq)]
enum Instruction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl Instruction {
    fn from_char(c: char) -> Result<Instruction, char> {
        match c {
            'N' => Ok(Instruction::North),
            'S' => Ok(Instruction::South),
            'E' => Ok(Instruction::East),
            'W' => Ok(Instruction::West),
            'L' => Ok(Instruction::Left),
            'R' => Ok(Instruction::Right),
            'F' => Ok(Instruction::Forward),
            _ => Err(c),
        }
    }

    fn turn_left(&self) -> Instruction {
        match self {
            Instruction::North => Instruction::West,
            Instruction::East => Instruction::North,
            Instruction::South => Instruction::East,
            Instruction::West => Instruction::South,
            _ => unreachable!(),
        }
    }

    fn turn_right(&self) -> Instruction {
        match self {
            Instruction::North => Instruction::East,
            Instruction::East => Instruction::South,
            Instruction::South => Instruction::West,
            Instruction::West => Instruction::North,
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day12)]
fn gen(input: &str) -> Vec<(Instruction, u32)> {
    input.lines()
        .map(|line| (Instruction::from_char(line.chars().next().unwrap()).unwrap(), line[1..].parse().unwrap()))
        .collect()
}

#[aoc(day12, part1)]
fn part1(input: &[(Instruction, u32)]) -> i32 {
    let mut pos = [0i32, 0i32];
    let mut facing = Instruction::East;
    for (mut instr, mut arg) in input.iter().copied() {
        if instr == Instruction::Forward { instr = facing; }
        match instr {
            Instruction::North => pos[1] -= arg as i32,
            Instruction::South => pos[1] += arg as i32,
            Instruction::East => pos[0] += arg as i32,
            Instruction::West => pos[0] -= arg as i32,
            Instruction::Left => while arg >= 90 {
                facing = facing.turn_left();
                arg -= 90;
            },
            Instruction::Right => while arg >= 90 {
                facing = facing.turn_right();
                arg -= 90;
            },
            Instruction::Forward => unreachable!(),
        }
    }
    pos[0].abs() + pos[1].abs()
}

#[aoc(day12, part2)]
fn part2(input: &[(Instruction, u32)]) -> i32 {
    let mut pos = [0i32, 0];
    let mut waypoint = [10i32, -1];
    for (instr, mut arg) in input.iter().copied() {
        match instr {
            Instruction::North => waypoint[1] -= arg as i32,
            Instruction::South => waypoint[1] += arg as i32,
            Instruction::East => waypoint[0] += arg as i32,
            Instruction::West => waypoint[0] -= arg as i32,
            Instruction::Left => while arg >= 90 {
                waypoint = [waypoint[1], -waypoint[0]];
                arg -= 90;
            },
            Instruction::Right => while arg >= 90 {
                waypoint = [-waypoint[1], waypoint[0]];
                arg -= 90;
            },
            Instruction::Forward => for _ in 0..arg {
                pos[0] += waypoint[0];
                pos[1] += waypoint[1];
            },
        }
    }
    pos[0].abs() + pos[1].abs()
}
