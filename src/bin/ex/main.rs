use advent_of_code_2024::utils::file;

fn replace_string(s: &str) -> String {
    let replacements = vec![
        ("one", "one1one"),
        ("two", "two2two"),
        ("three", "three3three"),
        ("four", "four4four"),
        ("five", "five5five"),
        ("six", "six6six"),
        ("seven", "seven7seven"),
        ("eight", "eight8eight"),
        ("nine", "nine9nine"),
    ];

    replacements
        .iter()
        .fold(s.to_owned(), |acc, (key, value)| acc.replace(key, value))
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let digits: Vec<i32> = line
                .chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as i32))
                .collect();
            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    part1(&replace_string(&input))
}

fn main() {
    let part1_input = file::read_file("files/day01/input");
    let part1 = part1(&part1_input);
    println!("Part 1: {}", part1);

    let part2_input = file::read_file("files/day01/input");
    let part2 = part2(&part2_input);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use advent_of_code_2024::utils::file;

    use super::{part1, part2};

    #[test]
    fn part1_test() {
        assert_eq!(part1(&file::read_file("files/ex/test_part1")), 142);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(&file::read_file("files/ex/test_part2")), 281);
    }

    #[test]
    fn part1_input() {
        assert_eq!(part1(&file::read_file("files/ex/input")), 57346);
    }

    #[test]
    fn part2_input() {
        assert_eq!(part2(&file::read_file("files/ex/input")), 57345);
    }
}
