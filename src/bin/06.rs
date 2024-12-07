use euclid::{point2, vec2, Point2D, Vector2D};
use std::collections::HashSet;
use std::ops::Add;

advent_of_code::solution!(6);

type Point = Point2D<isize, isize>;
type Direction = Vector2D<isize, isize>;

pub fn part_one(input: &str) -> Option<usize> {
    let (guard, bounds, walls) = parse(input);

    let (states_visited, _) = move_guard(&guard, &bounds, &walls, &None);

    Some(
        states_visited
            .into_iter()
            .map(|(a, _)| a)
            .collect::<HashSet<Point>>()
            .len(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (guard, bounds, walls) = parse(input);

    let (states_visited, _) = move_guard(&guard, &bounds, &walls, &None);

    Some(
        states_visited
            .into_iter()
            .map(|(a, _)| a)
            .collect::<HashSet<Point>>()
            .iter()
            .filter(|&&pos| move_guard(&guard, &bounds, &walls, &Some(pos)).1)
            .count(),
    )
}

fn within_bounds(guard: &Point, bounds: &Point) -> bool {
    guard.x <= bounds.x && guard.y <= bounds.y && guard.x >= 0 && guard.y >= 0
}

fn move_guard(
    initial: &Point,
    bounds: &Point,
    walls: &HashSet<Point>,
    obstruction: &Option<Point>,
) -> (HashSet<(Point, Direction)>, bool) {
    let mut moving_guard = initial.clone();
    let mut direction = vec2(-1, 0);
    let mut states_seen: HashSet<(Point, Direction)> = HashSet::new();

    while within_bounds(&moving_guard, &bounds) {
        let state = (moving_guard, direction);
        if states_seen.contains(&state) {
            return (states_seen, true);
        }

        states_seen.insert(state);

        let next = moving_guard.add(direction);
        if walls.contains(&next) || obstruction.eq(&Some(next)) {
            // we've hit a wall, turn 90º clockwise
            direction = vec2(direction.y, -direction.x)
        } else {
            // no wall, move!
            moving_guard = next;
        }
    }

    (states_seen, false)
}

fn parse(input: &str) -> (Point, Point, HashSet<Point>) {
    let mut guard: Option<Point> = None;
    let mut bounds: Point = point2(0, 0);

    let walls = input
        .lines()
        .enumerate()
        .map(|(r, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(c, ch)| {
                    bounds = bounds.max(point2(r as isize, c as isize));

                    if ch == '#' {
                        Some(point2(r as isize, c as isize))
                    } else {
                        if ch == '^' {
                            guard = Some(point2(r as isize, c as isize))
                        }

                        None
                    }
                })
                .collect::<HashSet<Point>>()
        })
        .flatten()
        .collect::<HashSet<Point>>();

    (guard.unwrap(), bounds, walls)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
