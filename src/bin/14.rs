use euclid::default::{Point2D, Vector2D};
use euclid::{point2, vec2};
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(14);

struct Robot {
    position: Point2D<isize>,
    velocity: Vector2D<isize>,
}

impl Robot {
    fn quadrant(&self, bounds: &Point2D<isize>) -> usize {
        let central_x = self.position.x == (bounds.x - 1) / 2;
        if central_x {
            return 4;
        }

        let central_y = self.position.y == (bounds.y - 1) / 2;
        if central_y {
            return 4;
        }

        let left = (2 * self.position.x) / bounds.x;
        let top = (2 * self.position.y) / bounds.y;

        ((left * 2) + top) as usize
    }
}

const SECONDS: isize = 100;

pub fn part_one(input: &str) -> Option<u32> {
    let (robots, bounds) = parse(input);

    let new_robots = robots
        .iter()
        .map(|robot| move_robot(robot, SECONDS, &bounds))
        .collect();

    let safety_score = get_quadrants(&new_robots, &bounds).iter().product();

    Some(safety_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (robots, bounds) = parse(input);

    let mut min_unique = robots
        .iter()
        .map(|robot| robot.position)
        .collect::<HashSet<Point2D<isize>>>()
        .len();

    let mut i = 0;

    loop {
        i += 1;

        if i > 8168 {
            break;
        }

        let new_robots: Vec<Robot> = robots
            .iter()
            .map(|robot| move_robot(&robot, i, &bounds))
            .collect();

        let new_unique = new_robots
            .iter()
            .map(|robot| robot.position)
            .collect::<HashSet<Point2D<isize>>>()
            .len();

        if new_unique < min_unique {
            continue;
        }

        // manually identified...
        min_unique = new_unique;

        // println!();
        // println!();
        // println!();
        // println!();
        // println!("{i} goes");
        // println!();
        // println!();

        // print(&new_robots, &bounds);
    }

    None
}

// fn is_symmetrical(robots: &Vec<Robot>, bounds: &Point2D<isize>) -> bool {
//     let unique_pos_robots = robots
//         .iter()
//         .map(|robot| (robot.position, robot))
//         .collect::<HashMap<Point2D<isize>, Robot>>();
//
//     let quadrants = get_quadrants(unique_pos_robots.values().collect(), &bounds);
//
//     quadrants.iter().fold(HashMap::new(), |mut acc, quadrant| {
//         *acc.entry(&quadrant).or_insert(0) += 1;
//
//
//     })
//
//     true
// }

fn get_quadrants(robots: &Vec<Robot>, bounds: &Point2D<isize>) -> Vec<u32> {
    robots
        .iter()
        .fold([0, 0, 0, 0, 0], |mut acc: [u32; 5], robot| {
            acc[robot.quadrant(&bounds)] += 1;

            acc
        })
        .into_iter()
        .take(4)
        .collect()
}

fn move_robot(robot: &Robot, seconds: isize, bounds: &Point2D<isize>) -> Robot {
    let new_x = (robot.position.x + (robot.velocity.x * seconds)).rem_euclid(bounds.x);
    let new_y = (robot.position.y + (robot.velocity.y * seconds)).rem_euclid(bounds.y);

    Robot {
        position: point2(new_x, new_y),
        velocity: robot.velocity,
    }
}

fn parse(input: &str) -> (Vec<Robot>, Point2D<isize>) {
    let robots: Vec<Robot> = input
        .lines()
        .map(|l| {
            let (pos, v) = l.split_once(' ').expect("Invalid line");

            Robot {
                position: numbers_to_vector(pos).to_point(),
                velocity: numbers_to_vector(v),
            }
        })
        .collect();

    // differentiate between the example and input
    let bounds = if robots.len() == 12 {
        point2(11, 7)
    } else {
        point2(101, 103)
    };

    (robots, bounds)
}

fn numbers_to_vector(input: &str) -> Vector2D<isize> {
    let (left, right) = input[2..].split_once(',').unwrap();

    vec2(left.parse().unwrap(), right.parse().unwrap())
}

fn print(robots: &Vec<Robot>, bounds: &Point2D<isize>) {
    let positions = robots.iter().fold(HashMap::new(), |mut acc, robot| {
        let key = (robot.position.x, robot.position.y);

        *acc.entry(key).or_insert(0) += 1;
        acc
    });

    for x in 0..bounds.x {
        for y in 0..bounds.y {
            let tile = match positions.get(&(x, y)) {
                Some(tile) => tile.to_string().chars().collect::<Vec<char>>()[0],
                None => ' ',
            };

            print!("{}", tile);
        }

        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
