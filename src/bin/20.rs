use euclid::{point2, vec2, Point2D};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

advent_of_code::solution!(20);

type Point = Point2D<isize, isize>;

pub fn part_one(input: &str) -> Option<usize> {
    let (start, end, save_minimum, racetrack) = parse(input);
    let track_vector = generate_racetrack_list(start, &end, &racetrack);

    Some(num_cheats(&track_vector, save_minimum, 2))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (start, end, save_minimum, racetrack) = parse(input);
    let track_vector = generate_racetrack_list(start, &end, &racetrack);

    Some(num_cheats(&track_vector, save_minimum, 20))
}

fn generate_racetrack_list(start: Point, end: &Point, racetrack: &HashSet<Point>) -> Vec<Point> {
    // TODO this is a hacky performance fix built on top of the old HashMap based solution
    let mut queue = vec![(start, 0)];
    let mut time_map = HashMap::new();

    while !queue.is_empty() && !time_map.contains_key(end) {
        for (point, cost) in &queue {
            time_map.insert(*point, *cost);
        }

        queue = queue
            .iter()
            .flat_map(|(point, cost)| {
                neighbours(point, racetrack)
                    .into_iter()
                    .filter(|n| !time_map.contains_key(n))
                    .map(|n| (n, cost + 1))
                    .collect::<HashSet<(Point, usize)>>()
            })
            .collect()
    }

    time_map
        .iter()
        .sorted_by(|a, b| a.1.cmp(b.1))
        .map(|x| *x.0)
        .collect::<Vec<Point>>()
}

fn neighbours(point: &Point, racetrack: &HashSet<Point>) -> Vec<Point> {
    [
        point.add(vec2(1, 0)),
        point.add(vec2(-1, 0)),
        point.add(vec2(0, 1)),
        point.add(vec2(0, -1)),
    ]
    .into_iter()
    .filter(|p| racetrack.contains(p))
    .collect()
}

fn num_cheats(racetrack: &[Point], save_minimum: usize, max_distance: isize) -> usize {
    racetrack
        .iter()
        .enumerate()
        .map(|(point_cost, point)| {
            racetrack
                .iter()
                .enumerate()
                .skip(point_cost + save_minimum)
                // map to manhattan distance between points, and cost of target point
                .map(|(cost, n)| (n.sub(*point).abs(), cost))
                // only target points within max distance from the source point
                .filter(|(delta, _)| delta.x + delta.y <= max_distance)
                // only points that would save enough to make the cheat worth it
                .filter(|(delta, cost)| {
                    let cheat_cost = point_cost + (delta.x + delta.y) as usize;

                    cheat_cost + save_minimum <= *cost
                })
                .count()
        })
        .sum::<usize>()
}

fn parse(input: &str) -> (Point, Point, usize, HashSet<Point>) {
    let mut start = None;
    let mut end = None;

    let racetrack = input
        .lines()
        .enumerate()
        .flat_map(|(r, row)| {
            row.chars()
                .enumerate()
                .filter_map(|(c, ch)| {
                    let point = point2(r as isize, c as isize);

                    match ch {
                        'S' => {
                            start = Some(point);
                            Some(point)
                        }
                        'E' => {
                            end = Some(point);
                            Some(point)
                        }
                        '.' => Some(point),
                        _ => None,
                    }
                })
                .collect::<HashSet<Point>>()
        })
        .collect::<HashSet<Point>>();

    let save_minimum = if start.unwrap().x == 3 { 50 } else { 100 };

    (start.unwrap(), end.unwrap(), save_minimum, racetrack)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(285));
    }
}
