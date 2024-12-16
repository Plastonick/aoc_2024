use euclid::{point2, vec2, Point2D, Vector2D};
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

advent_of_code::solution!(15);

type Point = Point2D<isize, isize>;
type Vector = Vector2D<isize, isize>;

pub fn part_one(input: &str) -> Option<isize> {
    let (robot, walls, boxes, directions) = parse(input);
    let score = calculate_safety_score(robot, &walls, boxes, &directions);

    Some(score)
}

pub fn part_two(input: &str) -> Option<isize> {
    let expanded_input = input
        .replace(".", "..")
        .replace("@", "@.")
        .replace("#", "##")
        .replace("O", "[]");

    let (robot, walls, boxes, directions) = parse(&expanded_input);
    let score = calculate_safety_score(robot, &walls, boxes, &directions);

    Some(score)
}

fn calculate_safety_score(
    robot: Point,
    walls: &HashSet<Point>,
    boxes: HashMap<Point, Point>,
    directions: &Vec<Vector>,
) -> isize {
    let mut robot = robot;
    let mut boxes = boxes;

    for direction in directions {
        (robot, boxes) = move_robot(robot, direction, boxes, &walls);
    }

    boxes.iter().map(|(b, _)| b.x * 100 + b.y).sum()
}

fn move_robot(
    robot: Point,
    direction: &Vector,
    boxes: HashMap<Point, Point>,
    walls: &HashSet<Point>,
) -> (Point, HashMap<Point, Point>) {
    let Some(boxes_to_move) = moveable_system(robot, direction, &boxes, &walls) else {
        return (robot, boxes);
    };

    let mut new_boxes = boxes.clone();
    for (box_to_move, connected_box) in boxes_to_move {
        new_boxes.remove(&box_to_move);
        new_boxes.insert(box_to_move.add(*direction), connected_box.add(*direction));
    }

    (robot.add(*direction), new_boxes)
}

fn moveable_system(
    pushing_from: Point,
    direction: &Vector,
    boxes: &HashMap<Point, Point>,
    walls: &HashSet<Point>,
) -> Option<HashMap<Point, Point>> {
    let mut system = HashMap::new();
    let mut wave = vec![pushing_from.add(*direction)];

    // TODO non-deterministic... there's an issue here somewhere!
    while !wave.is_empty() {
        let mut new_wave = vec![];
        for point in wave {
            // we've already included this point in our system! Carry on...
            if system.contains_key(&point) {
                continue;
            }

            // we're trying to push a wall, the push will fail, return None
            if walls.contains(&point) {
                return None;
            }

            // we're trying to push a box! Great!
            // Add that point to our system, and add anything it's also pushing for consideration
            if let Some(&connected_box) = boxes.get(&point) {
                system.insert(point, connected_box);

                new_wave.push(connected_box); // the box piece our box is connected to
                new_wave.push(point.add(*direction)); // the space with which this box is trying to move into
            }
        }

        wave = new_wave;
    }

    Some(system)
}

fn parse(input: &str) -> (Point, HashSet<Point>, HashMap<Point, Point>, Vec<Vector>) {
    let (map, direction_string) = input.split_once("\n\n").unwrap();
    let mut walls = HashSet::new();
    let mut boxes = HashMap::new();
    let mut robot = None;
    let right_vector = vec2(0, 1);

    for (r, row) in map.lines().enumerate() {
        for (c, ch) in row.chars().enumerate() {
            let point = point2(r as isize, c as isize);

            match ch {
                '#' => {
                    walls.insert(point);
                }
                'O' => {
                    boxes.insert(point, point);
                }
                '[' => {
                    boxes.insert(point, point.add(right_vector));
                    boxes.insert(point, point.sub(right_vector));
                }
                '@' => {
                    robot = Some(point);
                }
                _ => {}
            }
        }
    }

    let directions = direction_string
        .lines()
        .map(|l| {
            l.chars().map(|ch| match ch {
                '>' => vec2(0, 1),
                '<' => vec2(0, -1),
                '^' => vec2(-1, 0),
                'v' => vec2(1, 0),
                _ => {
                    panic!("Unexpected character '{ch}'!");
                }
            })
        })
        .flatten()
        .collect::<Vec<Vector>>();

    (robot.unwrap(), walls, boxes, directions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
