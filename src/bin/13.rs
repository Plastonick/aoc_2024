use euclid::{point2, Point2D};
use regex::Regex;
use std::ops::{Mul, Sub};

advent_of_code::solution!(13);

#[derive(Debug)]
struct Game {
    a_delta: Point2D<i32, i32>,
    b_delta: Point2D<i32, i32>,
    target: Point2D<i32, i32>,
}

pub fn part_one(input: &str) -> Option<i32> {
    let games = parse(input);

    let total_cost = games.iter().filter_map(min_cost).sum::<i32>();

    Some(total_cost)
}

pub fn part_two(_input: &str) -> Option<i32> {
    None
}

fn min_cost(game: &Game) -> Option<i32> {
    (0..=(game.target.y / game.a_delta.y))
        .filter_map(|a_presses| {
            let a_total = game.a_delta.mul(a_presses);
            let remaining_target = game.target.sub(a_total).to_point();

            let b_presses = remaining_target.x / game.b_delta.x;
            let b_total = game.b_delta.mul(b_presses);

            let is_valid = b_total == remaining_target;
            if is_valid {
                Some(a_presses * 3 + b_presses)
            } else {
                None
            }
        })
        .min()
}

fn parse(input: &str) -> Vec<Game> {
    let re = Regex::new("\\d+").unwrap();

    input
        .split("\n\n")
        .map(|game| {
            let numbers = re
                .find_iter(game)
                .filter_map(|m| {
                    let parsed = m.as_str().parse::<i32>();

                    match parsed {
                        Ok(num) => Some(num),
                        _ => None,
                    }
                })
                .collect::<Vec<i32>>();

            assert_eq!(numbers.len(), 6);

            Game {
                a_delta: point2(numbers[0], numbers[1]),
                b_delta: point2(numbers[2], numbers[3]),
                target: point2(numbers[4], numbers[5]),
            }
        })
        .collect::<Vec<Game>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
