fn main() {
    println!(
        "Part 1: {}",
        solve(include_str!("input").split("\n\n").collect())
    );
    println!(
        "Part 2: {}",
        solve_2(include_str!("input").split("\n\n").collect())
    );
}

const NECESSARY_FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const OPTIONAL_FIELDS: &[&str] = &["cid"];
const EYE_COLORS: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn solve(passports: Vec<&str>) -> usize {
    let mut valid = 0;
    for passport in passports {
        let passport = passport.replace("\n", " ");
        let data = passport.split(" ").collect::<Vec<&str>>();
        let mut fields = 0;
        for field in data {
            if let [field, data] = field.split(":").collect::<Vec<&str>>().as_slice() {
                if NECESSARY_FIELDS.contains(field) && !data.is_empty() {
                    fields += 1;
                } else if !OPTIONAL_FIELDS.contains(field) {
                    continue;
                }
            }
        }
        if fields == NECESSARY_FIELDS.len() {
            valid += 1;
        }
    }
    valid
}

fn solve_2(passports: Vec<&str>) -> usize {
    let mut valid = 0;
    for passport in passports {
        let passport = passport.replace("\n", " ");
        let data = passport.split(" ").collect::<Vec<&str>>();
        let mut fields = 0;
        for field in data {
            if let [field, data] = field.split(":").collect::<Vec<&str>>().as_slice() {
                let valid = match *field {
                    "byr" => {
                        data.parse::<usize>().is_ok()
                            && data.parse::<usize>().unwrap() >= 1920
                            && data.parse::<usize>().unwrap() <= 2002
                    }
                    "iyr" => {
                        data.parse::<usize>().is_ok()
                            && data.parse::<usize>().unwrap() >= 2010
                            && data.parse::<usize>().unwrap() <= 2020
                    }
                    "eyr" => {
                        data.parse::<usize>().is_ok()
                            && data.parse::<usize>().unwrap() >= 2020
                            && data.parse::<usize>().unwrap() <= 2030
                    }
                    "hgt" => {
                        data.len() > 3
                            && if data.ends_with("cm") {
                                data[0..3].parse::<usize>().is_ok()
                                    && data[0..3].parse::<usize>().unwrap() >= 150
                                    && data[0..3].parse::<usize>().unwrap() <= 193
                            } else if data.ends_with("in") {
                                data[0..2].parse::<usize>().is_ok()
                                    && data[0..2].parse::<usize>().unwrap() >= 59
                                    && data[0..2].parse::<usize>().unwrap() <= 75
                            } else {
                                false
                            }
                    }
                    "hcl" => {
                        data.len() == 7
                            && data.starts_with("#")
                            && data.chars().skip(1).all(|ch| {
                                ch.is_numeric()
                                    || (ch.is_ascii_lowercase() && (ch as u8) <= ('f' as u8))
                            })
                    }
                    "ecl" => EYE_COLORS.contains(data),
                    "pid" => data.len() == 9 && data.chars().all(|ch| ch.is_numeric()),
                    "cid" => true,
                    _ => false,
                };
                if valid && !OPTIONAL_FIELDS.contains(field) {
                    fields += 1;
                }
            }
        }
        if fields == NECESSARY_FIELDS.len() {
            valid += 1;
        }
    }
    valid
}

#[cfg(test)]
mod tests {
    use crate::{solve, solve_2};

    #[test]
    fn tests() {
        assert_eq!(
            solve(
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
            "
                .split("\n\n")
                .collect()
            ),
            2
        );
        assert_eq!(
            solve_2(
                "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
            "
                .split("\n\n")
                .collect()
            ),
            0
        );
        assert_eq!(
            solve_2(
                "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
            "
                .split("\n\n")
                .collect()
            ),
            4
        );
    }
}
