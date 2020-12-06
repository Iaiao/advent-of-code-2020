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

fn solve(q: Vec<&str>) -> usize {
    let mut y = 0;
    for group in q {
        let b = group.replace("\n", "");
        let mut a = b.chars().collect::<Vec<char>>();
        a.sort();
        a.dedup();
        y += a.len();
    }
    y
}

fn solve_2(q: Vec<&str>) -> usize {
    let mut y = 0;
    for group in q {
        let b = group.split("\n").filter(|l| !l.is_empty()).collect::<Vec<&str>>();
        let mut a = b.first().unwrap().chars().collect::<Vec<char>>();
        for c in b.iter() {
            for ch in a.clone().iter() {
                if !c.contains(&ch.to_string()) {
                    a.retain(|d| *d != *ch);
                }
            }
        }
        y += a.len()
    }
    y
}

#[cfg(test)]
mod tests {
    use crate::{solve, solve_2};

    #[test]
    fn tests() {
        assert_eq!(solve("abc

a
b
c

ab
ac

a
a
a
a

b
".split("\n\n").collect()), 11);
        assert_eq!(solve_2(
            "
abc

a
b
c

ab
ac

a
a
a
a

b
".split("\n\n").collect()
        ), 6)
    }
}
