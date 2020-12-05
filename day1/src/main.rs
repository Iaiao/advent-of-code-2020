fn main() {
    println!(
        "Part 1: {}",
        solve(
            include_str!("input")
                .lines()
                .map(|line| line.parse().unwrap())
                .collect()
        )
    );
    println!(
        "Part 2: {}",
        solve_2(
            include_str!("input")
                .lines()
                .map(|line| line.parse().unwrap())
                .collect()
        )
    );
}

// O(n**x), but it's all about coding speed, right?
fn solve(numbers: Vec<i32>) -> i32 {
    for i in &numbers {
        for j in &numbers {
            if i + j == 2020 {
                return i * j;
            }
        }
    }
    0
}

fn solve_2(numbers: Vec<i32>) -> i32 {
    for i in &numbers {
        for j in &numbers {
            for k in &numbers {
                if i + j + k == 2020 {
                    return i * j * k;
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::{solve, solve_2};

    #[test]
    fn tests() {
        assert_eq!(solve(vec!(1721, 979, 366, 299, 675, 1456)), 514579);
        assert_eq!(solve_2(vec!(1721, 979, 366, 299, 675, 1456)), 241861950);
    }
}
