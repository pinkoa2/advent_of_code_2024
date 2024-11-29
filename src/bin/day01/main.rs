use advent_of_code_2024::utils::file;

fn part1(file_path: &str) -> String {
    let output = file::read_file(file_path);
    output
}

fn main() {
    let part1 = part1("files/day01/input");
    println!("Part 1: {}", part1);
}

// Define tests here
#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn test_example() {
        assert_eq!(part1("files/day01/test"), "test");
    }
}
