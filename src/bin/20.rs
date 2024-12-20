use euclid::{point2, vec2, Point2D};
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

advent_of_code::solution!(20);

type Point = Point2D<isize, isize>;

pub fn part_one(input: &str) -> Option<usize> {
    let (start, end, save_atleast, racetrack) = parse(input);

    let mut n_cheats = 0;
    let time_map = generate_time_map(start, &end, &racetrack);
    for (point, _) in &time_map {
        n_cheats += num_cheats(point, &time_map, save_atleast);
    }

    Some(n_cheats)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        point.sub(vec2(1, 0)),
        point.add(vec2(0, 1)),
        point.sub(vec2(0, 1)),
    ]
    .into_iter()
    .filter(|p| racetrack.contains(p))
    .collect()
}

fn num_cheats(point: &Point, time_map: &HashMap<Point, usize>, save_atleast: usize) -> usize {
    let point_cost = time_map[point];

    [
        vec2(2, 0),
        vec2(-2, 0),
        vec2(0, 2),
        vec2(0, -2),
        vec2(1, 1),
        vec2(-1, 1),
        vec2(1, -1),
        vec2(-1, -1),
    ]
    .into_iter()
    .filter_map(|d| {
        let neighbour = point.add(d);

        if let Some(cost) = time_map.get(&neighbour) {
            if point_cost + save_atleast + 1 < *cost {
                Some(true)
            } else {
                None
            }
        } else {
            None
        }
    })
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
                            start = Some(point.clone());
                            Some(point)
                        }
                        'E' => {
                            end = Some(point.clone());
                            Some(point)
                        }
                        '.' => Some(point),
                        _ => None,
                    }
                })
                .collect::<HashSet<Point>>()
        })
        .collect::<HashSet<Point>>();

    let save_atleast = if start.unwrap().x == 3 { 50 } else { 100 };

    (start.unwrap(), end.unwrap(), save_atleast, racetrack)
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
        assert_eq!(result, Some(3));
    }
}
