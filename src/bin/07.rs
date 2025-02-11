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
            if is_valid(test, numbers[0], &numbers[1..], operations) {
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

trait Operable {
    fn operate(&self, a: usize, b: usize) -> usize;
}

impl Operable for Operation {
    fn operate(&self, a: usize, b: usize) -> usize {
        match self {
            Operation::Add => a + b,
            Operation::Mul => a * b,
            Operation::Concat => concat(a, b),
        }
    }
}

fn is_valid(test: usize, carry: usize, numbers: &[usize], operations: &[Operation]) -> bool {
    if numbers.is_empty() {
        return carry == test;
    }

    for operation in operations {
        let carry = operation.operate(carry, numbers[0]);

        if is_valid(test, carry, &numbers[1..], operations) {
            return true;
        }
    }

    false
}

fn concat(a: usize, b: usize) -> usize {
    a * 10_usize.pow(b.ilog10() + 1) + b
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
