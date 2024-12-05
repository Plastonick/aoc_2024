use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    Some(sum_updates(input).0)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(sum_updates(input).1)
}

fn sum_updates(input: &str) -> (u32, u32) {
    let (orderings, updates) = parse(input);

    let mut correctly_ordered = 0;
    let mut incorrectly_ordered = 0;
    for update in updates {
        let mut sorted_update = update.clone();

        sorted_update.sort_by(|&a, &b| {
            if let Some(order) = orderings.get(&(a.min(b), a.max(b))) {
                if order.1 == a {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            } else {
                std::cmp::Ordering::Equal
            }
        });

        let to_add = sorted_update.get(sorted_update.len() / 2).unwrap();

        if sorted_update != update {
            incorrectly_ordered += to_add
        } else {
            correctly_ordered += to_add
        }
    }

    (correctly_ordered, incorrectly_ordered)
}

fn parse(input: &str) -> (HashMap<(u32, u32), (u32, u32)>, Vec<Vec<u32>>) {
    let (ordering_lines, update_lines) = input.split_once("\n\n").expect("invalid input");
    let orderings = ordering_lines
        .lines()
        .map(|l| {
            let (first, last) = l.split_once('|').expect("Invalid ordering line");

            let first_int = first.parse::<u32>().unwrap();
            let last_int = last.parse::<u32>().unwrap();

            (
                (first_int.min(last_int), first_int.max(last_int)), // key
                (first_int, last_int),                              // value
            )
        })
        .collect::<HashMap<(u32, u32), (u32, u32)>>();
    let updates = update_lines
        .lines()
        .map(|l| {
            l.split(',')
                .map(|num| num.parse::<u32>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();
    (orderings, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
