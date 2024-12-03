use advent_of_code_2024::utils::file::read_file;

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect()
}

fn does_line_fit_condition_2(line: &str) -> bool {
    let parsed = parse_line(line);

    parsed.iter().enumerate().any(|(i, _)| {
        let copy: Vec<String> = parsed
            .iter()
            .enumerate()
            .filter_map(|(j, k)| if i != j { Some(k.to_string()) } else { None })
            .collect();
        does_line_fit_condition_1(&copy.join(" "))
    })
}

fn does_line_fit_condition_1(line: &str) -> bool {
    let parsed = parse_line(line);

    let diff: Vec<i32> = parsed.windows(2).map(|w| w[0] - w[1]).collect();

    let all_inc_or_dec = diff.iter().all(|&x| x > 0) || diff.iter().all(|&x| x < 0);
    let abs_diff = diff.iter().map(|x| x.abs()).collect::<Vec<i32>>();

    all_inc_or_dec && abs_diff.iter().min().unwrap() >= &1 && abs_diff.iter().max().unwrap() <= &3
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| does_line_fit_condition_1(line))
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| does_line_fit_condition_2(line))
        .count()
}

fn main() {
    let input = read_file("files/day02/input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    use advent_of_code_2024::utils::file::read_file;
    use once_cell::sync::Lazy;

    static TEST_INPUT: Lazy<String> = Lazy::new(|| read_file("files/day02/test"));
    static REAL_INPUT: Lazy<String> = Lazy::new(|| read_file("files/day02/input"));

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT), 4);
    }

    #[test]
    fn input_part1() {
        assert_eq!(part1(&REAL_INPUT), 624);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(&REAL_INPUT), 658);
    }
}
