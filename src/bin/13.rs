use euclid::{point2, Point2D};
use regex::Regex;
use std::ops::Mul;

advent_of_code::solution!(13);

#[derive(Debug)]
struct Game {
    a_delta: Point2D<isize, isize>,
    b_delta: Point2D<isize, isize>,
    target: Point2D<isize, isize>,
}

const COSTS: (isize, isize) = (3, 1);

pub fn part_one(input: &str) -> Option<isize> {
    let games = parse(input);
    let total_cost = games.iter().filter_map(min_cost).sum::<isize>();

    Some(total_cost)
}

pub fn part_two(input: &str) -> Option<isize> {
    const CONVERSION_DELTA: isize = 10000000000000;

    let games = parse(input);
    let total_cost = games
        .into_iter()
        .map(|g| Game {
            target: point2(g.target.x + CONVERSION_DELTA, g.target.y + CONVERSION_DELTA),
            ..g
        })
        .filter_map(|g| min_cost(&g))
        .sum::<isize>();

    Some(total_cost)
}

fn min_cost(game: &Game) -> Option<isize> {
    // TODO, I'm not sure this is necessarily valid for cases where a_delta and b_delta are not co-prime vectors
    // luckily, that appears to not be the case here...
    let b_pushes = (game.a_delta.y * game.target.x - game.a_delta.x * game.target.y)
        / (game.a_delta.y * game.b_delta.x - game.a_delta.x * game.b_delta.y);
    let a_pushes = (game.target.y - b_pushes * game.b_delta.y) / game.a_delta.y;

    let total = game.a_delta.mul(a_pushes) + game.b_delta.mul(b_pushes).to_vector();

    if total == game.target {
        Some(a_pushes * COSTS.0 + b_pushes * COSTS.1)
    } else {
        None
    }
}

fn parse(input: &str) -> Vec<Game> {
    let re = Regex::new("\\d+").unwrap();

    input
        .split("\n\n")
        .map(|game| {
            let numbers = re
                .find_iter(game)
                .filter_map(|m| {
                    let parsed = m.as_str().parse::<isize>();

                    match parsed {
                        Ok(num) => Some(num),
                        _ => None,
                    }
                })
                .collect::<Vec<isize>>();

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
