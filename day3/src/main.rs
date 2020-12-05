fn main() {
    println!(
        "Part 1: {}",
        solve(&include_str!("input").lines().collect(), (3, 1))
    );
    let mut multiplied = 1;
    let map = include_str!("input").lines().collect();
    for slope in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        multiplied *= solve(&map, *slope);
    }
    println!("Part 2: {}", multiplied);
}

fn solve(map: &Vec<&str>, slope: (usize, usize)) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;
    let max_y = map.len();
    loop {
        x = x + slope.0;
        y = y + slope.1;
        if y >= max_y {
            break
        }
        match map[y].chars().nth(x % map[y].len()).unwrap() {
            '.' => {}
            '#' => trees += 1,
            _ => panic!("Invalid cell!")
        }
    }
    trees
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn tests() {
        assert_eq!(
            solve(&vec!(
                "..##.......",
                "#...#...#..",
                ".#....#..#.",
                "..#.#...#.#",
                ".#...##..#.",
                "..#.##.....",
                ".#.#.#....#",
                ".#........#",
                "#.##...#...",
                "#...##....#",
                ".#..#...#.#"
            ), (3, 1)),
            7
        );
    }
}
