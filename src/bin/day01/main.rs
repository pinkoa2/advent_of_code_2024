use advent_of_code_2024::utils::file::read_file;
use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .lines()
        .map(|line| {
            let split = line
                .split_whitespace()
                .filter_map(|ele| ele.parse::<usize>().ok())
                .collect::<Vec<usize>>();
            (split[0], split[1])
        })
        .unzip()
}

fn part1(input: &str) -> i32 {
    let (mut a1, mut a2) = parse_input(input);
    a1.sort();
    a2.sort();

    a1.into_iter()
        .zip(a2.into_iter())
        .map(|(e1, e2)| e1.abs_diff(e2) as i32)
        .sum()
}

fn part2(input: &str) -> i32 {
    let (a1, a2) = parse_input(input);

    let mut hm = HashMap::<usize, usize>::new();
    for &value in &a2 {
        *hm.entry(value).or_insert(0) += 1;
    }

    let mut result = 0;
    for ele in a1 {
        let number_of_occurrences = hm.get(&ele).unwrap_or(&0);
        result += ele * number_of_occurrences;
    }

    result as i32
}

fn main() {
    let part1_input = read_file("files/day01/input");
    println!("Part 1: {}", part1(&part1_input));

    let part2_input = read_file("files/day01/input");
    println!("Part 2: {}", part2(&part2_input));
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use advent_of_code_2024::utils::file::read_file;

    #[test]
    fn test_part1() {
        let input = read_file("files/day01/test");
        assert_eq!(part1(&input), 11);
    }

    #[test]
    fn test_part2() {
        let input = read_file("files/day01/test");
        assert_eq!(part2(&input), 31);
    }

    #[test]
    fn input_part1() {
        let input = read_file("files/day01/input");
        assert_eq!(part1(&input), 2264607);
    }

    #[test]
    fn input_part2() {
        let input = read_file("files/day01/input");
        assert_eq!(part2(&input), 19457120);
    }
}
