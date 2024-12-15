use euclid::{point2, vec2, Point2D, Vector2D};
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

advent_of_code::solution!(15);

type Point = Point2D<isize, isize>;
type Vector = Vector2D<isize, isize>;

pub fn part_one(input: &str) -> Option<isize> {
    let (mut robot, walls, mut boxes, directions) = parse(input);

    for direction in directions {
        (robot, boxes) = move_robot(robot, direction, boxes, &walls);
    }

    let score = boxes.iter().map(|(b, _)| b.x * 100 + b.y).sum::<isize>();

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let expanded_input = input
        .replace(".", "..")
        .replace("#", "##")
        .replace("O", "[]");

    let (mut robot, walls, mut boxes, directions) = parse(&expanded_input);

    None
}

fn move_robot(
    robot: Point,
    direction: Vector,
    boxes: HashMap<Point, Point>,
    walls: &HashSet<Point>,
) -> (Point, HashMap<Point, Point>) {
    // list everything in order between the robot and the nearest wall or empty, in direction
    let mut boxes_to_move = vec![];
    let mut pos = robot.clone();

    loop {
        pos = pos.add(direction);

        if boxes.contains_key(&pos) {
            boxes_to_move.push(pos);
        } else if walls.contains(&pos) {
            // there's a wall, can't move, return the existing state
            return (robot, boxes);
        } else {
            // found an "empty" space, that's the end of our ray, return!
            break;
        }
    }

    if boxes_to_move.len() == 0 {
        (robot.add(direction), boxes)
    } else {
        let mut new_boxes = boxes.clone();
        new_boxes.remove(&boxes_to_move[0]);
        let d = boxes_to_move.last().unwrap().add(direction);
        new_boxes.insert(d, d);

        (robot.add(direction), new_boxes)
    }
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
        assert_eq!(result, None);
    }
}
