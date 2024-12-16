use euclid::{point2, vec2, Point2D, Vector2D};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::ops::Add;

advent_of_code::solution!(16);

type Point = Point2D<isize, isize>;
type Vector = Vector2D<isize, isize>;

#[derive(Hash, PartialEq, Eq)]
struct Particle {
    cost: isize,
    position: Point,
    direction: Vector,
}

impl Particle {
    fn state(&self) -> (Point, Vector) {
        (self.position, self.direction)
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let (start, end, map) = parse(input);

    find_best_path(
        Particle {
            cost: 0,
            position: start,
            direction: vec2(0, 1),
        },
        end,
        &map,
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn find_best_path(from: Particle, to: Point, map: &HashSet<Point>) -> Option<isize> {
    let mut cheapest_states: HashMap<(Point, Vector), isize> = HashMap::new();
    let mut wave = BTreeMap::new();
    wave.entry(from.cost).or_insert(HashSet::new()).insert(from);

    while !wave.is_empty() {
        let (value, particles) = wave.pop_first().unwrap();

        for particle in particles {
            if particle.position == to {
                return Some(value);
            }

            for neighbour in get_neighbours(particle, &map) {
                if let Some(&cost) = cheapest_states.get(&neighbour.state()) {
                    if cost < neighbour.cost {
                        // we've already been here cheaper, don't add the neighbour
                        continue;
                    } else {
                        cheapest_states.insert(neighbour.state(), neighbour.cost);
                    }
                }

                wave.entry(neighbour.cost)
                    .or_insert(HashSet::new())
                    .insert(neighbour);
            }
        }
    }

    None
}

fn get_neighbours(particle: Particle, map: &HashSet<Point>) -> Vec<Particle> {
    // move forward or turn 90º either CW or CCW
    [
        Particle {
            cost: particle.cost + 1,
            position: particle.position.add(particle.direction),
            ..particle
        },
        Particle {
            cost: particle.cost + 1000,
            direction: vec2(particle.direction.y, -particle.direction.x),
            ..particle
        },
        Particle {
            cost: particle.cost + 1000,
            direction: vec2(-particle.direction.y, particle.direction.x),
            ..particle
        },
    ]
    .into_iter()
    .filter(|p| map.contains(&p.position))
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
