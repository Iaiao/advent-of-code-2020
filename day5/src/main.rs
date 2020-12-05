fn main() {
    let results = include_str!("input")
        .lines()
        .map(|input| solve(input))
        .collect::<Vec<(i32, i32, i32)>>();
    let mut ids = results.iter().map(|res| res.2).collect::<Vec<i32>>();
    ids.sort();
    let highest = ids.last().unwrap();
    println!("Part 1: {}", highest);
    let possibly_my_seats = (0..1024)
        .into_iter()
        .filter(|id| ids.contains(&(id - 1)) && ids.contains(&(id + 1)) && !ids.contains(id))
        .collect::<Vec<i32>>();
    println!("Part 2: {:?}", possibly_my_seats);
}

fn solve(s: &str) -> (i32, i32, i32) {
    let mut row = 0..128;
    let mut column = 0..8;
    for ch in s.chars() {
        let row_d = row.end - row.start;
        let column_d = column.end - column.start;
        match ch {
            'F' => row = row.start..(row.start + row_d / 2),
            'B' => row = (row.start + row_d / 2)..row.end,
            'R' => column = (column.start + column_d / 2)..column.end,
            'L' => column = column.start..(column.start + column_d / 2),
            _ => panic!(),
        }
    }
    let row = row.nth(0).unwrap();
    let column = column.nth(0).unwrap();
    (row, column, row * 8 + column)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn tests() {
        assert_eq!(solve("BFFFBBFRRR"), (70, 7, 567));
        assert_eq!(solve("FFFBBBFRRR"), (14, 7, 119));
        assert_eq!(solve("BBFFBBFRLL"), (102, 4, 820));
    }
}
