use advent_of_code_2024::utils::file::read_file;
use regex::Regex;

const MUL_REGEX: &str = r"mul\((\d{1,3}),(\d{1,3})\)";
const DO_DONT_REGEX: &str = r"do\(\)|don't\(\)";

fn parse_mul(cap: &regex::Captures) -> usize {
    let x = &cap[1].parse::<usize>().unwrap();
    let y = &cap[2].parse::<usize>().unwrap();
    x * y
}

fn part1(input: &str) -> usize {
    let regex = Regex::new(MUL_REGEX).unwrap();
    regex.captures_iter(input).map(|cap| parse_mul(&cap)).sum()
}

fn part2(input: &str) -> usize {
    let regex = Regex::new(&format!("{MUL_REGEX}|{DO_DONT_REGEX}")).unwrap();

    let mut do_ = true;
    let mut result = 0;
    for cap in regex.captures_iter(input) {
        match &cap[0] {
            "do()" => do_ = true,
            "don't()" => do_ = false,
            _ => {
                if do_ {
                    result += parse_mul(&cap);
                }
            }
        }
    }
    result
}

fn main() {
    let input = read_file("files/day03/input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    use advent_of_code_2024::utils::file::read_file;
    use once_cell::sync::Lazy;

    static REAL_INPUT: Lazy<String> = Lazy::new(|| read_file("files/day03/input"));

    #[test]
    fn test_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(&input), 161);
    }

    #[test]
    fn test_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(&input), 48);
    }

    #[test]
    fn input_part1() {
        assert_eq!(part1(&REAL_INPUT), 166905464);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(&REAL_INPUT), 72948684);
    }
}
