use euclid::{point2, vec2, Point2D, Vector2D};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::ops::Add;

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
        (robot, boxes) = move_robot(robot, direction, boxes, walls);

        // uncomment to view the robot progression
        // _print_map(&robot, &boxes, &walls);
        // std::thread::sleep(std::time::Duration::from_millis(200));
    }

    boxes
        .keys()
        .map(|p| get_left_most_box_in_system(p, &boxes))
        .unique()
        .map(|b| b.x * 100 + b.y)
        .sum()
}

fn move_robot(
    robot: Point,
    direction: &Vector,
    boxes: HashMap<Point, Point>,
    walls: &HashSet<Point>,
) -> (Point, HashMap<Point, Point>) {
    let Some(boxes_to_move) = moveable_system(robot, direction, &boxes, walls) else {
        return (robot, boxes);
    };

    let mut new_boxes = boxes.clone();
    for (box_to_move, _) in boxes_to_move.iter() {
        new_boxes.remove(box_to_move);
    }
    for (box_to_move, connected_box) in boxes_to_move {
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

            // empty space, nothing to push here!
            if !boxes.contains_key(&point) {
                continue;
            }

            // we're trying to push a box! Great!
            // Add that point to our system, and add anything it's also pushing for consideration
            for (box_piece, connected_piece) in get_system_for_box_segment(&point, boxes) {
                system.insert(box_piece, connected_piece);

                new_wave.push(box_piece.add(*direction)); // the space with which this box piece is trying to move into
            }
        }

        wave = new_wave;
    }

    Some(system)
}

// finds all boxes which are in a connected system, as in they pull on each other as well as push
fn get_system_for_box_segment(
    segment: &Point,
    boxes: &HashMap<Point, Point>,
) -> Vec<(Point, Point)> {
    let mut next_segment = boxes.get(segment).unwrap();
    let mut segments = vec![(*segment, *next_segment)];

    while next_segment != segment {
        let connected_segment = boxes.get(next_segment).unwrap();

        segments.push((*next_segment, *connected_segment));
        next_segment = connected_segment;
    }

    segments
}

fn get_left_most_box_in_system(segment: &Point, boxes: &HashMap<Point, Point>) -> Point {
    let system = get_system_for_box_segment(segment, boxes);

    system
        .into_iter()
        .map(|(p, _)| p)
        .sorted_by(|a, b| {
            if a.x == b.x {
                a.y.cmp(&b.y)
            } else {
                a.x.cmp(&b.x)
            }
        })
        .next()
        .unwrap()
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
                    let right_point = point.add(right_vector);

                    boxes.insert(point, right_point);
                    boxes.insert(right_point, point);
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
        .flat_map(|l| {
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
        .collect::<Vec<Vector>>();

    (robot.unwrap(), walls, boxes, directions)
}

fn _print_map(robot: &Point, boxes: &HashMap<Point, Point>, walls: &HashSet<Point>) {
    let bounds = walls
        .iter()
        .map(|x| x.to_owned())
        .reduce(|a, b| a.max(b))
        .unwrap();

    for x in 0..=bounds.x {
        for y in 0..=bounds.y {
            let point = point2(x, y);
            let tile = if walls.contains(&point) {
                '#'
            } else if boxes.contains_key(&point) {
                '['
            } else if &point == robot {
                '@'
            } else {
                '.'
            };

            print!("{tile}");
        }

        println!();
    }

    // flush the stdout buffer
    println!("\n\n\n\n");
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
