extern crate core;

use euclid::{point2, vec2, Point2D, Vector2D};
use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(16);

type Point = Point2D<isize, isize>;
type Vector = Vector2D<isize, isize>;

#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy)]
struct State {
    position: Point,
    direction: Vector,
}

pub fn part_one(input: &str) -> Option<usize> {
    let (start, end, map) = parse(input);

    let (_, cost) = find_all_optimal_paths(
        State {
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
        State {
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
    from: State,
    end: Point,
    map: &HashSet<Point>,
) -> Option<(Vec<Vec<Point>>, usize)> {
    let mut best_cost_to_state: HashMap<State, (usize, Vec<State>)> = HashMap::new();
    let mut queue = PriorityQueue::new();
    queue.push(from.clone(), Reverse(0));

    let mut optimal_cost = None;
    let mut end_states = vec![];

    while let Some((particle, Reverse(particle_cost))) = queue.pop() {
        if optimal_cost.unwrap_or(usize::MAX) < particle_cost {
            // we've found all the optimal paths, discontinue!
            break;
        }

        if particle.position == end {
            end_states.push(particle);
            optimal_cost = Some(particle_cost);
        }

        let neighbours = get_neighbours(&particle, &map);
        if neighbours.len() == 100000000 {
            panic!();
        }

        for (neighbour, move_cost) in neighbours {
            let neighbour_cost = particle_cost + move_cost;

            let (best_cost, prev_points) = best_cost_to_state
                .entry(neighbour.clone())
                .or_insert((usize::MAX, vec![]));

            if neighbour_cost > *best_cost {
                // this is more expensive way to get to the same state, ignore
                continue;
            }

            if neighbour_cost < *best_cost {
                // this is a cheaper way of getting here, reset the entry
                best_cost_to_state.insert(neighbour.clone(), (neighbour_cost, vec![particle]));
            } else if neighbour_cost == *best_cost {
                // println!("Adding duplicate prior");

                // this is an equivalently expensive way of getting here, include it!
                prev_points.push(particle)
            }

            queue.push(neighbour, Reverse(neighbour_cost));
        }
    }

    if let Some(cost) = optimal_cost {
        let paths = follow_paths_back(end_states, &best_cost_to_state);

        Some((
            paths
                .iter()
                .map(|path| path.iter().map(|state| state.position).collect())
                .collect(),
            cost,
        ))
    } else {
        None
    }
}

fn follow_paths_back(
    path: Vec<State>,
    best_paths_to_states: &HashMap<State, (usize, Vec<State>)>,
) -> Vec<Vec<State>> {
    let &last = path.last().unwrap();

    if let Some((_, prior)) = best_paths_to_states.get(&last) {
        prior
            .iter()
            .map(|p| {
                let mut new_path = path.clone();
                new_path.push(*p);

                follow_paths_back(new_path, &best_paths_to_states)
            })
            .flatten()
            .collect()
    } else {
        vec![path]
    }
}

fn get_neighbours(particle: &State, map: &HashSet<Point>) -> Vec<(State, usize)> {
    // move forward or turn 90º either direction
    let d1 = vec2(particle.direction.y, -particle.direction.x);
    let d2 = vec2(-particle.direction.y, particle.direction.x);

    [
        (
            State {
                position: particle.position + particle.direction,
                direction: particle.direction,
            },
            1,
        ),
        (
            State {
                position: particle.position + d1,
                direction: d1,
            },
            1001,
        ),
        (
            State {
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
    fn test_part_two_ex_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(64));
    }

    #[test]
    fn test_part_one_ex_3() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(3006));
    }

    #[test]
    fn test_part_two_ex_3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(10));
    }
}
