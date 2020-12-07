use {
    std::str::FromStr,
    crate::prelude::*,
};

lazy_static! {
    static ref HCL_REGEX: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
    static ref PID_REGEX: Regex = Regex::new("^[0-9]{9}$").unwrap();
}

#[derive(Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn valid1(&self) -> bool {
        self.byr.is_some()
        && self.iyr.is_some()
        && self.eyr.is_some()
        && self.hgt.is_some()
        && self.hcl.is_some()
        && self.ecl.is_some()
        && self.pid.is_some()
    }

    fn valid2(&self) -> bool {
        self.byr.as_ref().map_or(false, |byr| byr.len() == 4 && byr.parse().map_or(false, |byr| (1920..=2002).contains(&byr)))
        && self.iyr.as_ref().map_or(false, |iyr| iyr.len() == 4 && iyr.parse().map_or(false, |iyr| (2010..=2020).contains(&iyr)))
        && self.eyr.as_ref().map_or(false, |eyr| eyr.len() == 4 && eyr.parse().map_or(false, |eyr| (2020..=2030).contains(&eyr)))
        && self.hgt.as_ref().map_or(false, |hgt|
            (hgt.ends_with("cm") && hgt[..hgt.len() - 2].parse().map_or(false, |hgt| (150..=193).contains(&hgt)))
            || (hgt.ends_with("in") && hgt[..hgt.len() - 2].parse().map_or(false, |hgt| (59..=76).contains(&hgt)))
        )
        && self.hcl.as_ref().map_or(false, |hcl| HCL_REGEX.is_match(&hcl))
        && self.ecl.as_ref().map_or(false, |ecl| matches!(&ecl[..], "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"))
        && self.pid.as_ref().map_or(false, |pid| PID_REGEX.is_match(&pid))
    }
}

impl FromStr for Passport {
    type Err = ();

    fn from_str(s: &str) -> Result<Passport, ()> {
        let mut passport = Passport::default();
        for field in s.split_whitespace() {
            let (name, value) = field.split(':').collect_tuple().ok_or(())?;
            match name {
                "byr" => passport.byr = Some(value.to_owned()),
                "iyr" => passport.iyr = Some(value.to_owned()),
                "eyr" => passport.eyr = Some(value.to_owned()),
                "hgt" => passport.hgt = Some(value.to_owned()),
                "hcl" => passport.hcl = Some(value.to_owned()),
                "ecl" => passport.ecl = Some(value.to_owned()),
                "pid" => passport.pid = Some(value.to_owned()),
                "cid" => passport.cid = Some(value.to_owned()),
                _ => return Err(()),
            }
        }
        Ok(passport)
    }
}

#[aoc_generator(day4)]
fn gen(input: &str) -> Vec<Passport> {
    input.split("\n\n").map(Passport::from_str).try_collect().unwrap()
}

#[aoc(day4, part1)]
fn part1(input: &[Passport]) -> usize {
    input.iter().filter(|passport| passport.valid1()).count()
}

#[aoc(day4, part2)]
fn part2(input: &[Passport]) -> usize {
    input.iter().filter(|passport| passport.valid2()).count()
}
