use std::collections::HashSet;

use advent_of_code_2024::utils::file::read_file;

#[derive(PartialEq, Copy, Clone, Hash, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn turn_right(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    }
}

fn parse_map(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_starting_pos(map: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '^' {
                return (i, j);
            }
        }
    }
    panic!("No starting position found");
}

fn get_next_pos(
    pos: &(usize, usize),
    direction: &Direction,
    max_pos: &(usize, usize),
) -> Option<(usize, usize)> {
    match direction {
        Direction::Up => {
            if pos.0 == 0 {
                return None;
            }
            return Some((pos.0 - 1, pos.1));
        }
        Direction::Down => {
            if pos.0 == max_pos.0 {
                return None;
            }
            return Some((pos.0 + 1, pos.1));
        }
        Direction::Left => {
            if pos.1 == 0 {
                return None;
            }
            return Some((pos.0, pos.1 - 1));
        }
        Direction::Right => {
            if pos.1 == max_pos.1 {
                return None;
            }
            return Some((pos.0, pos.1 + 1));
        }
    }
}

fn part1(input: &str) -> usize {
    let mut map = parse_map(input);
    let mut pos = find_starting_pos(&map);
    let max_pos = (map.len() - 1, map[0].len() - 1);
    let mut direction = Direction::Up;

    map[pos.0][pos.1] = '.';

    let mut results = HashSet::new();
    results.insert(pos.clone());

    loop {
        let next_pos = get_next_pos(&pos, &direction, &max_pos);
        match next_pos {
            Some(n_pos) => {
                let n_char = map[n_pos.0][n_pos.1];
                match n_char {
                    '.' => {
                        pos = n_pos;
                        results.insert(pos.clone());
                    }
                    '#' => {
                        direction = turn_right(&direction);
                    }
                    _ => panic!("Invalid character: {}", n_char),
                }
            }
            None => break,
        }
    }
    results.len()
}

fn part2(input: &str) -> usize {
    let mut map = parse_map(input);
    let mut traveled: HashSet<(Direction, (usize, usize))> = HashSet::new();
    let mut result = 0;

    let max_pos = (map.len() - 1, map[0].len() - 1);
    let starting_pos = find_starting_pos(&map);
    map[starting_pos.0][starting_pos.1] = '.';

    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == '#' {
                continue;
            }
            map[row][col] = '#';
            let mut pos = starting_pos.clone();
            let mut direction = Direction::Up;
            traveled.clear();
            loop {
                traveled.insert((direction.clone(), pos.clone()));
                let next_pos = get_next_pos(&pos, &direction, &max_pos);
                match next_pos {
                    Some(n_pos) => {
                        let n_char = map[n_pos.0][n_pos.1];
                        match n_char {
                            '.' => {
                                pos = n_pos;
                                if traveled.contains(&(direction.clone(), pos.clone())) {
                                    result += 1;
                                    break;
                                }
                            }
                            '#' => {
                                direction = turn_right(&direction);
                            }
                            _ => panic!("Invalid character: {}", n_char),
                        }
                    }
                    None => break,
                }
            }

            map[row][col] = '.';
        }
    }
    result
}

fn main() {
    let input = read_file("files/day06/input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    use advent_of_code_2024::utils::file::read_file;
    use once_cell::sync::Lazy;

    static TEST_INPUT: Lazy<String> = Lazy::new(|| read_file("files/day06/test"));
    static REAL_INPUT: Lazy<String> = Lazy::new(|| read_file("files/day06/input"));

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT), 6);
    }

    #[test]
    fn input_part1() {
        assert_eq!(part1(&REAL_INPUT), 5086);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(&REAL_INPUT), 1770);
    }
}
