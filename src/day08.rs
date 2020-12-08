use crate::prelude::*;

#[derive(Clone, Copy)]
enum Instruction {
    Nop,
    Acc,
    Jmp,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Instruction, ()> {
        match s {
            "nop" => Ok(Instruction::Nop),
            "acc" => Ok(Instruction::Acc),
            "jmp" => Ok(Instruction::Jmp),
            _ => Err(()),
        }
    }
}

fn run(program: &[(Instruction, i32)]) -> Result<i32, i32> {
    let mut visited = HashSet::new();
    let mut ip = 0;
    let mut acc = 0;
    loop {
        if ip >= program.len() { break Ok(acc) }
        if !visited.insert(ip) { break Err(acc) }
        let (instr, arg) = program[ip];
        match instr {
            Instruction::Nop => ip += 1,
            Instruction::Acc => {
                acc += arg;
                ip += 1;
            }
            Instruction::Jmp => ip = (ip as i32 + arg) as usize,
        }
    }
}

#[aoc_generator(day8)]
fn gen(input: &str) -> Vec<(Instruction, i32)> {
    input.lines()
        .map(|line| {
            let (instr, arg) = line.split_whitespace().collect_tuple().unwrap();
            (instr.parse().unwrap(), arg.parse().unwrap())
        })
        .collect()
}

#[aoc(day8, part1)]
fn part1(input: &[(Instruction, i32)]) -> i32 {
    run(input).unwrap_err()
}

#[aoc(day8, part2)]
fn part2(input: &[(Instruction, i32)]) -> i32 {
    let original = input.to_owned();
    for (i, (instr, _)) in original.iter().enumerate() {
        match instr {
            Instruction::Nop => {
                let modified = original.iter().enumerate().map(|(j, (instr, arg))| (if i == j { Instruction::Jmp } else { *instr }, *arg)).collect_vec();
                if let Ok(acc) = run(&modified) { return acc }
            }
            Instruction::Jmp => {
                let modified = original.iter().enumerate().map(|(j, (instr, arg))| (if i == j { Instruction::Nop } else { *instr }, *arg)).collect_vec();
                if let Ok(acc) = run(&modified) { return acc }
            }
            _ => {}
        }
    }
    panic!()
}
