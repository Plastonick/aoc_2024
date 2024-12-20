use euclid::{point2, vec2, Point2D};
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

advent_of_code::solution!(20);

type Point = Point2D<isize, isize>;

pub fn part_one(input: &str) -> Option<usize> {
    let (start, end, save_minimum, racetrack) = parse(input);

    let time_map = generate_time_map(start, &end, &racetrack);
    let num_cheats = time_map.keys().map(|point| num_cheats(point, &time_map, save_minimum, 2))
        .sum::<usize>();

    Some(num_cheats)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (start, end, save_minimum, racetrack) = parse(input);

    let time_map = generate_time_map(start, &end, &racetrack);
    let num_cheats = time_map.keys().map(|point| num_cheats(point, &time_map, save_minimum, 20))
        .sum::<usize>();

    Some(num_cheats)
}

fn generate_time_map(
    start: Point,
    end: &Point,
    racetrack: &HashSet<Point>,
) -> HashMap<Point, usize> {
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

fn num_cheats(
    point: &Point,
    time_map: &HashMap<Point, usize>,
    save_minimum: usize,
    max_distance: usize,
) -> usize {
    let point_cost = time_map[point];

    time_map
        .iter()
        .filter(|(n, cost)| {
            let delta = n.sub(*point).abs();
            let within_min_distance = delta.x + delta.y <= max_distance as isize;

            let cheat_cost = point_cost + (delta.x + delta.y) as usize;
            let actual_saving = cheat_cost + save_minimum <= **cost;

            within_min_distance && actual_saving
        })
        .map(|(n, _)| format!("{}, {} to {}, {}", point.x, point.y, n.x, n.y))
        .count()
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
