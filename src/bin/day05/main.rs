use advent_of_code_2024::utils::file::read_file;
use std::collections::{HashMap, HashSet};

fn dedup_without_sorting(numbers: &Vec<usize>) -> Vec<usize> {
    let mut hash_set = HashSet::new();
    let mut deduped = vec![];

    for number in numbers {
        if hash_set.insert(number.clone()) {
            deduped.push(number.clone());
        }
    }

    deduped
}

fn order_numbers(numbers: &Vec<usize>, ordering: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let mut ordered = numbers.clone();
    let mut index: isize = 0;

    while (index as usize) < ordered.len() {
        let mut n_ordered = ordered.clone();
        let (left, _right) = ordered.split_at(index as usize);

        let current = ordered[index as usize];
        let parents = ordering.get(&current).unwrap_or(&vec![]).clone();
        for parent in parents {
            if !left.contains(&parent) && numbers.contains(&parent) {
                n_ordered.insert(0, parent);
                index = -1;
            }
        }
        index += 1;
        ordered = dedup_without_sorting(&n_ordered);
    }
    ordered
}

fn parse_ordering(input: &str) -> HashMap<usize, Vec<usize>> {
    let mut hash_map: HashMap<usize, Vec<usize>> = HashMap::new();
    let first_part = input.split("\n\n").next().unwrap();

    first_part.lines().for_each(|line| {
        let values = line
            .split("|")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let (value, key) = (values[0], values[1]);
        hash_map.entry(key).or_insert(vec![]).push(value);
    });

    hash_map
}

fn parse_updates(input: &str) -> Vec<Vec<usize>> {
    let second_part = input.split("\n\n").nth(1).unwrap();
    second_part
        .lines()
        .map(|line| {
            line.split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

fn part1(input: &str) -> usize {
    let ordering = parse_ordering(input);
    let updates = parse_updates(input);

    let ordered = updates
        .iter()
        .map(|update| order_numbers(update, &ordering))
        .collect::<Vec<Vec<usize>>>();

    ordered
        .iter()
        .enumerate()
        .map(|(i, row)| {
            if row == &updates[i] {
                let middle = row.len() / 2;
                return row[middle];
            }
            0
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let ordering = parse_ordering(input);
    let updates = parse_updates(input);

    let ordered = updates
        .iter()
        .map(|update| order_numbers(update, &ordering))
        .collect::<Vec<Vec<usize>>>();

    ordered
        .iter()
        .enumerate()
        .map(|(i, row)| {
            if row != &updates[i] {
                let middle = row.len() / 2;
                return row[middle];
            }
            0
        })
        .sum()
}

fn main() {
    let input = read_file("files/day05/input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    use advent_of_code_2024::utils::file::read_file;
    use once_cell::sync::Lazy;

    static TEST_INPUT: Lazy<String> = Lazy::new(|| read_file("files/day05/test"));
    static REAL_INPUT: Lazy<String> = Lazy::new(|| read_file("files/day05/input"));

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT), 123);
    }

    #[test]
    fn input_part1() {
        assert_eq!(part1(&REAL_INPUT), 5964);
    }

    #[test]
    fn input_part2() {
        assert_eq!(part2(&REAL_INPUT), 4719);
    }
}
