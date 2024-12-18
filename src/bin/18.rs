use euclid::{point2, vec2, Point2D};
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::ops::Add;

advent_of_code::solution!(18);

type Point = Point2D<isize, isize>;

pub fn part_one(input: &str) -> Option<usize> {
    let (byte_points, bounds, take) = parse(input);

    min_path(
        point2(0, 0),
        &bounds,
        &byte_points
            .into_iter()
            .take(take)
            .collect::<HashSet<Point>>(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    // TODO, make this faster, at _least_ do a binary tree search...
    let (byte_points, bounds, take) = parse(input);

    for i in take..byte_points.len() {
        let cost = min_path(
            point2(0, 0),
            &bounds,
            &byte_points
                .clone()
                .into_iter()
                .take(i)
                .collect::<HashSet<Point>>(),
        );

        if cost.is_none() {
            return Some(format!("{},{}", byte_points[i - 1].x, byte_points[i - 1].y));
        }
    }

    None
}

fn min_path(start: Point, bounds: &Point, byte_points: &HashSet<Point>) -> Option<usize> {
    let mut visited = HashMap::new();
    let mut queue: PriorityQueue<Point, Reverse<usize>> = PriorityQueue::new();
    queue.push(start, Reverse(0));

    while let Some((element, Reverse(value))) = queue.pop() {
        visited.insert(element, value);
        if &element == bounds {
            break;
        }

        let neighbours = neighbours(&element, bounds, byte_points);

        neighbours
            .into_iter()
            .filter(|x| !visited.contains_key(x))
            .for_each(|n| {
                queue.push(n, Reverse(value + 1));
            });
    }

    visited.get(bounds).map(|x| x.to_owned())
}

fn neighbours(point: &Point, bounds: &Point, byte_points: &HashSet<Point>) -> Vec<Point> {
    [
        point.add(vec2(1, 0)),
        point.add(vec2(-1, 0)),
        point.add(vec2(0, 1)),
        point.add(vec2(0, -1)),
    ]
    .into_iter()
    .filter(|p| p.x <= bounds.x && p.y <= bounds.y && p.x >= 0 && p.y >= 0)
    .filter(|p| !byte_points.contains(p))
    .collect()
}

fn parse(input: &str) -> (Vec<Point>, Point, usize) {
    // differentiate between the example and the actual input
    let (bounds, take) = if input.lines().count() == 25 {
        (point2(6, 6), 12)
    } else {
        (point2(70, 70), 1024)
    };

    let byte_points = input
        .lines()
        .map(|p| {
            let (x, y) = p.split_once(',').expect("Unexpected line");

            point2(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect::<Vec<Point>>();

    (byte_points, bounds, take)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
