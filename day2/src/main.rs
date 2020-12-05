use std::ops::RangeInclusive;

fn main() {
    println!("Part 1: {}", solve(include_str!("input").lines().collect()));
    println!("Part 2: {}", solve_2(include_str!("input").lines().collect()));
}

fn solve(passwords: Vec<&str>) -> usize {
    let mut i = 0;
    for entry in passwords {
        if let [range, char, password] = entry.split(" ").collect::<Vec<&str>>().as_slice() {
            if let [min, max] = range.split("-").collect::<Vec<&str>>().as_slice() {
                let range: RangeInclusive<usize> = min.parse().unwrap()..=max.parse().unwrap();
                let char = char.chars().nth(0).unwrap();
                if range.contains(&password.chars().filter(|ch| *ch == char).count()) {
                    i += 1;
                }
            }
        }
    }
    i
}

fn solve_2(passwords: Vec<&str>) -> usize {
    let mut i = 0;
    for entry in passwords {
        if let [range, char, password] = entry.split(" ").collect::<Vec<&str>>().as_slice() {
            if let [min, max] = range.split("-").collect::<Vec<&str>>().as_slice() {
                let (a, b): (usize, usize) = (min.parse().unwrap(), max.parse().unwrap());
                let char = char.chars().nth(0).unwrap();
                let mut occurrences = 0;
                let chars = password.chars().collect::<Vec<char>>();
                if chars[a - 1] == char {
                    occurrences += 1;
                }
                if chars[b - 1] == char {
                    occurrences += 1;
                }
                if occurrences == 1 {
                    i += 1;
                }
            }
        }
    }
    i
}

#[cfg(test)]
mod tests {
    use crate::{solve, solve_2};

    #[test]
    fn tests() {
        assert_eq!(
            solve(vec!("1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc")),
            2
        );
        assert_eq!(
            solve_2(vec!("1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc")),
            1
        );
    }
}
