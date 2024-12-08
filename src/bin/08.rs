use euclid::{point2, Point2D, Vector2D};
use itertools::iproduct;
use num::Integer;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Div, Mul};

advent_of_code::solution!(8);

type Point = Point2D<isize, isize>;
type Vector = Vector2D<isize, isize>;

pub fn part_one(input: &str) -> Option<usize> {
    let (antennas, bounds) = parse(input);

    let all_anti_nodes = antennas
        .values()
        .map(|antennas| find_anti_nodes(&antennas, &bounds))
        .flatten()
        .collect::<HashSet<Point>>();

    Some(all_anti_nodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (antennas, bounds) = parse(input);

    let all_t_nodes = antennas
        .values()
        .map(|antennas| find_t_nodes(&antennas, &bounds))
        .flatten()
        .collect::<HashSet<Point>>();

    Some(all_t_nodes.len())
}

fn find_anti_nodes(antennas: &Vec<Point>, bounds: &Point) -> HashSet<Point> {
    let product = iproduct!(antennas.iter(), antennas.iter());

    product
        .filter(|(a, b)| a != b)
        .flat_map(|(&a, &b)| {
            let d = a - b;

            vec![a + d, b - d]
        })
        .filter(|&x| x.x >= 0 && x.y >= 0 && x.x <= bounds.x && x.y <= bounds.y)
        .collect::<HashSet<Point>>()
}

fn find_t_nodes(antennas: &Vec<Point>, bounds: &Point) -> HashSet<Point> {
    let product = iproduct!(antennas.iter(), antennas.iter());

    product
        .filter(|(a, b)| a != b)
        .flat_map(|(&a, &b)| {
            let d = a - b;
            let gcd = d.x.gcd(&d.y);
            let d_reduced = d.div(gcd);

            ray_points(&a, &d_reduced, &bounds)
        })
        .collect::<HashSet<Point>>()
}

fn ray_points(anchor: &Point, step: &Vector, bounds: &Point) -> Vec<Point> {
    if step.x == 0 && step.y == 0 {
        panic!("Didn't expect a 0-vector!")
    }

    assert_eq!(bounds.x, bounds.y);

    // find a point on the ray outside the bounds
    let num_points = if step.x > step.y {
        bounds.x / step.x
    } else {
        bounds.y / step.y
    };

    // TODO a little excessive, we check way more points than we need to...
    let start_point = anchor.add(step.mul(-num_points));

    (0..=num_points * 2)
        .into_iter()
        .map(|i| start_point.add(step.mul(i)))
        .filter(|x| within_bounds(&x, &bounds))
        .collect()
}

fn within_bounds(point: &Point, bounds: &Point) -> bool {
    point.x <= bounds.x && point.y <= bounds.y && point.x >= 0 && point.y >= 0
}

fn parse(input: &str) -> (HashMap<char, Vec<Point>>, Point) {
    let mut bounds = point2(0, 0);

    let antennas = input
        .lines()
        .enumerate()
        .map(|(r, row)| {
            row.chars()
                .enumerate()
                .filter_map(|(c, ch)| {
                    let point = point2(r as isize, c as isize);
                    bounds = bounds.max(point);

                    if ch == '.' {
                        None
                    } else {
                        Some((point, ch))
                    }
                })
                .collect::<Vec<(Point, char)>>()
        })
        .flatten()
        .fold(HashMap::new(), |mut acc, antenna| {
            let antennas = acc.entry(antenna.1).or_insert(Vec::new());
            antennas.push(antenna.0);

            acc
        });

    (antennas, bounds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
