use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<usize> {
    let lines = input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .take(2)
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<Vec<isize>>>();

    let mut list_one = lines.iter().map(|x| x[0]).collect::<Vec<isize>>();
    let mut list_two = lines.iter().map(|x| x[1]).collect::<Vec<isize>>();

    list_one.sort();
    list_two.sort();

    let total_distance: usize = list_one
        .iter()
        .enumerate()
        .map(|(indx, el)| el.abs_diff(*list_two.get(indx).unwrap()))
        .sum();

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<isize> {
    let lines = input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .take(2)
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<Vec<isize>>>();

    let list_one = lines.iter().map(|x| x[0]).collect::<Vec<isize>>();
    let list_two_freq = lines
        .iter()
        .map(|x| x[1])
        .fold(HashMap::new(), |mut acc, el| {
            *acc.entry(el).or_insert(0) += 1;
            acc
        });

    let similarity = list_one
        .iter()
        .map(|x| x * list_two_freq.get(x).unwrap_or(&0))
        .sum();

    Some(similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
