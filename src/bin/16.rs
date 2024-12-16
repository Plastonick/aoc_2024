extern crate core;

use euclid::{point2, vec2, Point2D, Vector2D};
use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashSet;

advent_of_code::solution!(16);

type Point = Point2D<isize, isize>;
type Vector = Vector2D<isize, isize>;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Particle {
    position: Point,
    direction: Vector,
}

pub fn part_one(input: &str) -> Option<usize> {
    let (start, end, map) = parse(input);

    let (_, cost) = find_all_optimal_paths(
        Particle {
            position: start,
            direction: vec2(0, 1),
        },
        end,
        &map,
    )
    .unwrap();

    Some(cost)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (start, end, map) = parse(input);

    let (paths, _) = find_all_optimal_paths(
        Particle {
            position: start,
            direction: vec2(0, 1),
        },
        end,
        &map,
    )
    .unwrap();

    Some(paths.iter().flatten().unique().count())
}

fn find_all_optimal_paths(
    from: Particle,
    to: Point,
    map: &HashSet<Point>,
) -> Option<(Vec<Vec<Particle>>, usize)> {
    let mut queue = PriorityQueue::new();
    queue.push((from.clone(), vec![from]), Reverse(0));

    let mut optimal_cost = None;
    let mut best_paths = vec![];

    while let Some(((particle, path), Reverse(particle_cost))) = queue.pop() {
        if optimal_cost.is_some() && optimal_cost < Some(particle_cost) {
            // we've found all the optimal paths, discontinue!
            break;
        }

        if particle.position == to {
            optimal_cost = Some(particle_cost);
            best_paths.push(path.clone());
        }

        for (neighbour, move_cost) in get_neighbours(&particle, &map) {
            let neighbour_cost = particle_cost + move_cost;
            let mut new_path = path.clone();
            new_path.push(neighbour.clone());

            queue.push((neighbour, new_path), Reverse(neighbour_cost));
        }
    }

    if let Some(cost) = optimal_cost {
        Some((best_paths, cost))
    } else {
        None
    }
}

fn get_neighbours(particle: &Particle, map: &HashSet<Point>) -> Vec<(Particle, usize)> {
    // move forward or turn 90º either direction
    let d1 = vec2(particle.direction.y, -particle.direction.x);
    let d2 = vec2(-particle.direction.y, particle.direction.x);

    [
        (
            Particle {
                position: particle.position + particle.direction,
                direction: particle.direction,
            },
            1,
        ),
        (
            Particle {
                position: particle.position + d1,
                direction: d1,
            },
            1001,
        ),
        (
            Particle {
                position: particle.position + d2,
                direction: d2,
            },
            1001,
        ),
    ]
    .into_iter()
    .filter(|(p, _)| map.contains(&p.position))
    .collect()
}

fn parse(input: &str) -> (Point, Point, HashSet<Point>) {
    let mut start = None;
    let mut end = None;

    let map = input
        .lines()
        .enumerate()
        .map(|(r, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, ch)| ch != &'#')
                .map(|(c, ch)| {
                    let point = point2(r as isize, c as isize);
                    match ch {
                        'S' => {
                            start = Some(point);
                        }
                        'E' => {
                            end = Some(point);
                        }
                        _ => {}
                    };

                    point
                })
                .collect::<Vec<Point>>()
        })
        .flatten()
        .collect::<HashSet<Point>>();

    (start.unwrap(), end.unwrap(), map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one_alt() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_part_two_alt() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(64));
    }
}
