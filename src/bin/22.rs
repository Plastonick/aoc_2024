use itertools::Itertools;
use std::collections::HashMap;
use std::ops::BitXor;

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<isize> {
    let numbers = input
        .lines()
        .map(|x| x.parse::<isize>().unwrap())
        .map(|n| process(n, 2000))
        .collect::<Vec<isize>>();

    Some(numbers.into_iter().sum::<isize>())
}

pub fn part_two(input: &str) -> Option<isize> {
    let sequences = input
        .lines()
        .map(|x| x.parse::<isize>().unwrap())
        .map(|n| {
            (1..2000)
                .fold(vec![n], |mut list, _| {
                    let last = list.last().unwrap();

                    list.push(process(*last, 1));
                    list
                })
                .iter()
                .map(|x| x % 10)
                .tuple_windows()
                .map(|(a, b)| (b, b - a))
                .collect()
        })
        .collect::<Vec<Vec<(isize, isize)>>>();

    // found the mappings of 4-sequence => value for each large sequence
    let sequence_value_maps = sequences
        .iter()
        .map(build_4_sequence_value_map)
        .collect::<Vec<HashMap<isize, isize>>>();

    // now iterate over all the possible unique 4-sequences and calculate scores
    sequence_value_maps
        .iter()
        .flat_map(|x| x.keys())
        .map(|x| x.to_owned())
        .unique()
        .map(|seq| {
            sequence_value_maps
                .iter()
                .filter_map(|value_map| value_map.get(&seq))
                .sum::<isize>()
        })
        .max()
}

fn build_4_sequence_value_map(sequence: &Vec<(isize, isize)>) -> HashMap<isize, isize> {
    let mut sequence_values = HashMap::new();

    for window in sequence.windows(4).rev() {
        let key = window
            .iter()
            .enumerate()
            .map(|(order, el)| el.1 * 20_isize.pow(order as u32))
            .sum::<isize>();

        sequence_values.insert(key, window[3].0);
    }

    sequence_values
}

fn process(number: isize, times: isize) -> isize {
    if times <= 0 {
        return number;
    }

    let number = (number * 64).bitxor(number) % 16777216;
    let number = (number / 32).bitxor(number) % 16777216;
    let number = (number * 2048).bitxor(number) % 16777216;

    process(number, times - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_ten() {
        let cases = [
            123, 15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484,
            7753432, 5908254,
        ];

        for (i, expected) in cases.into_iter().enumerate() {
            let secret = process(123, i as isize);

            assert_eq!(secret, expected);
        }
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
