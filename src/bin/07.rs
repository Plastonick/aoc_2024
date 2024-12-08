use std::cmp::PartialEq;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<usize> {
    let operations = vec![Operation::Add, Operation::Mul];

    find_test_sum(input, &operations)
}

pub fn part_two(input: &str) -> Option<usize> {
    let operations = vec![Operation::Add, Operation::Mul, Operation::Concat];

    find_test_sum(input, &operations)
}

fn find_test_sum(input: &str, operations: &Vec<Operation>) -> Option<usize> {
    let equations = parse(input);
    let valid = equations
        .into_iter()
        .filter_map(|(test, numbers)| {
            if is_valid(test, &numbers, &operations) {
                Some(test)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

    Some(valid.iter().sum())
}

#[derive(Debug, Clone, PartialEq)]
enum Operation {
    Add,
    Mul,
    Concat,
}

fn is_valid(test: usize, numbers: &Vec<usize>, operations: &Vec<Operation>) -> bool {
    if numbers.len() == 1 {
        return numbers[0] == test;
    }

    for operation in operations {
        let new_first = match operation {
            Operation::Add => numbers[0] + numbers[1],
            Operation::Mul => numbers[0] * numbers[1],
            Operation::Concat => format!("{}{}", numbers[0], numbers[1])
                .parse::<usize>()
                .unwrap(),
        };
        let mut new_numbers = vec![new_first];
        for number in numbers.iter().skip(2) {
            new_numbers.push(*number)
        }

        if is_valid(test, &new_numbers, &operations) {
            return true;
        }
    }

    false
}

fn parse(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|l| {
            let (test, partial) = l.split_once(": ").unwrap();

            (
                test.parse::<usize>().unwrap(),
                partial
                    .split(' ')
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
