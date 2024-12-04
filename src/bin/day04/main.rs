use advent_of_code_2024::utils::file::read_file;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>()
}

fn does_it_spell_xmas(
    grid: &Vec<Vec<char>>,
    i: isize,
    j: isize,
    (i0, j0): (isize, isize),
    (i1, j1): (isize, isize),
    (i2, j2): (isize, isize),
    (i3, j3): (isize, isize),
) -> bool {
    let x = grid
        .get((i + i0) as usize)
        .and_then(|row| row.get((j + j0) as usize));
    let m = grid
        .get((i + i1) as usize)
        .and_then(|row| row.get((j + j1) as usize));
    let a = grid
        .get((i + i2) as usize)
        .and_then(|row| row.get((j + j2) as usize));
    let s = grid
        .get((i + i3) as usize)
        .and_then(|row| row.get((j + j3) as usize));
    x == Some(&'X') && m == Some(&'M') && a == Some(&'A') && s == Some(&'S')
}

fn is_it_an_x(grid: &Vec<Vec<char>>, i: isize, j: isize) -> bool {
    let middle = grid.get(i as usize).and_then(|row| row.get(j as usize));
    let top_left = grid
        .get((i - 1) as usize)
        .and_then(|row| row.get((j - 1) as usize));
    let top_right = grid
        .get((i - 1) as usize)
        .and_then(|row| row.get((j + 1) as usize));
    let bottom_left = grid
        .get((i + 1) as usize)
        .and_then(|row| row.get((j - 1) as usize));
    let bottom_right = grid
        .get((i + 1) as usize)
        .and_then(|row| row.get((j + 1) as usize));

    let diag1 = &format!(
        "{}{}{}",
        top_left.unwrap_or(&'Z'),
        middle.unwrap_or(&'Z'),
        bottom_right.unwrap_or(&'Z')
    );
    let diag2 = &format!(
        "{}{}{}",
        top_right.unwrap_or(&'Z'),
        middle.unwrap_or(&'Z'),
        bottom_left.unwrap_or(&'Z')
    );

    (diag1 == "MAS" || diag1 == "SAM") && (diag2 == "MAS" || diag2 == "SAM")
}

fn part1(input: &str) -> usize {
    let grid = parse_input(input);

    let mut result = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let directions = vec![
                vec![(0, 0), (0, 1), (0, 2), (0, 3)],
                vec![(0, 0), (0, -1), (0, -2), (0, -3)],
                vec![(0, 0), (1, 0), (2, 0), (3, 0)],
                vec![(0, 0), (-1, 0), (-2, 0), (-3, 0)],
                vec![(0, 0), (1, 1), (2, 2), (3, 3)],
                vec![(0, 0), (-1, -1), (-2, -2), (-3, -3)],
                vec![(0, 0), (1, -1), (2, -2), (3, -3)],
                vec![(0, 0), (-1, 1), (-2, 2), (-3, 3)],
            ];
            for direction in directions {
                if does_it_spell_xmas(
                    &grid,
                    i as isize,
                    j as isize,
                    direction[0],
                    direction[1],
                    direction[2],
                    direction[3],
                ) {
                    result += 1;
                }
            }
        }
    }
    result
}

fn part2(input: &str) -> usize {
    let grid = parse_input(input);

    let mut result = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if is_it_an_x(&grid, i as isize, j as isize) {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    let input = read_file("files/day04/input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    use advent_of_code_2024::utils::file::read_file;
    use once_cell::sync::Lazy;

    static TEST_INPUT: Lazy<String> = Lazy::new(|| read_file("files/day04/test"));
    static REAL_INPUT: Lazy<String> = Lazy::new(|| read_file("files/day04/input"));

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT), 9);
    }

    #[test]
    fn input_part1() {
        assert_eq!(part1(&REAL_INPUT), 2618);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(&REAL_INPUT), 2011);
    }
}
