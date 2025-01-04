use regex::Regex;
use std::collections::BTreeMap;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    Some(get_products(input).iter().map(|(_, p)| p).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(do(n't)?\(\))").unwrap();
    let ablers = re
        .find_iter(input)
        .map(|mat| (mat.start(), mat.as_str() == "do()"))
        .collect::<BTreeMap<usize, bool>>();
    let products = get_products(input);

    let sum = products
        .iter()
        .filter_map(|(i, p)| {
            let add = ablers.range(..*i).next_back().unwrap_or((&0, &true));

            if *add.1 {
                Some(p)
            } else {
                None
            }
        })
        .sum::<u32>();

    Some(sum)
}

fn get_products(input: &str) -> Vec<(usize, u32)> {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();

    re.find_iter(input)
        .map(|mat| (mat.start(), product_from_mul(mat.as_str())))
        .collect::<Vec<(usize, u32)>>()
}

fn product_from_mul(mul: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(mul)
        .map(|cap| cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
