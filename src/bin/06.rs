use euclid::{point2, vec2, Point2D, Vector2D};
use itertools::Itertools;
use std::collections::HashSet;
use std::ops::Add;

advent_of_code::solution!(6);

type Point = Point2D<isize, isize>;
type Direction = Vector2D<isize, isize>;

#[derive(Clone, Hash, Eq, PartialEq)]
struct Guard {
    position: Point,
    direction: Direction,
}

impl Guard {
    fn move_one(&self) -> Guard {
        Guard {
            position: self.position.add(self.direction),
            direction: self.direction,
        }
    }

    fn turn_90cw(&self) -> Guard {
        Guard {
            position: self.position,
            direction: vec2(self.direction.y, -self.direction.x),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (guard, bounds, walls) = parse(input);
    let (states_visited, _, _) = move_guard(&guard, &bounds, &walls, &None);

    Some(
        states_visited
            .into_iter()
            .map(|a| a.position)
            .unique()
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (guard, bounds, walls) = parse(input);
    let (_, path, _) = move_guard(&guard, &bounds, &walls, &None);

    Some(
        path.into_iter()
            .unique_by(|a| a.position)
            .tuple_windows()
            .filter_map(|(new_guard, obstr)| {
                let (_, _, does_loop) =
                    move_guard(&new_guard, &bounds, &walls, &Some(obstr.position));

                if does_loop {
                    Some(obstr.position)
                } else {
                    None
                }
            })
            .count(),
    )
}

fn within_bounds(guard: &Point, bounds: &Point) -> bool {
    guard.x <= bounds.x && guard.y <= bounds.y && guard.x >= 0 && guard.y >= 0
}

fn move_guard(
    guard: &Guard,
    bounds: &Point,
    walls: &HashSet<Point>,
    obstruction: &Option<Point>,
) -> (HashSet<Guard>, Vec<Guard>, bool) {
    let mut guard = guard.to_owned();
    let mut states_seen: HashSet<Guard> = HashSet::new();
    let mut path = vec![guard.to_owned()];

    while within_bounds(&guard.position, bounds) {
        if states_seen.contains(&guard) {
            return (states_seen, path, true);
        }

        path.push(guard.to_owned());
        states_seen.insert(guard.to_owned());

        let next = guard.move_one();
        guard = if walls.contains(&next.position) || obstruction.eq(&Some(next.position)) {
            // we would hit a wall, instead let's turn 90º clockwise
            guard.turn_90cw()
        } else {
            // no wall, move!
            next
        }
    }

    (states_seen, path, false)
}

fn parse(input: &str) -> (Guard, Point, HashSet<Point>) {
    let mut guard_start: Option<Point> = None;
    let mut bounds: Point = point2(0, 0);

    let walls = input
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(c, ch)| {
                    bounds = bounds.max(point2(r as isize, c as isize));

                    if ch == '#' {
                        Some(point2(r as isize, c as isize))
                    } else {
                        if ch == '^' {
                            guard_start = Some(point2(r as isize, c as isize))
                        }

                        None
                    }
                })
                .collect::<HashSet<Point>>()
        })
        .collect::<HashSet<Point>>();

    let guard = Guard {
        position: guard_start.unwrap(),
        direction: vec2(-1, 0),
    };

    (guard, bounds, walls)
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
