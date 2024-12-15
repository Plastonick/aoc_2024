use euclid::default::{Point2D, Vector2D};
use euclid::{point2, vec2};

advent_of_code::solution!(14);

struct Robot {
    position: Point2D<isize>,
    velocity: Vector2D<isize>,
}

impl Robot {
    fn quadrant(self, bounds: &Point2D<isize>) -> usize {
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
    let robots = parse(input);

    // differentiate between the example and input
    let bounds = if robots.len() == 12 {
        point2(11, 7)
    } else {
        point2(101, 103)
    };

    let safety_score = robots
        .iter()
        .map(|robot| move_robot(robot, SECONDS, &bounds))
        .fold([0, 0, 0, 0, 0], |mut acc, robot| {
            acc[robot.quadrant(&bounds)] += 1;

            acc
        })
        .iter()
        .take(4)
        .product();

    Some(safety_score)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn move_robot(robot: &Robot, seconds: isize, bounds: &Point2D<isize>) -> Robot {
    let new_x = (robot.position.x + (robot.velocity.x * seconds)).rem_euclid(bounds.x);
    let new_y = (robot.position.y + (robot.velocity.y * seconds)).rem_euclid(bounds.y);

    Robot {
        position: point2(new_x, new_y),
        velocity: robot.velocity,
    }
}

fn parse(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|l| {
            let (pos, v) = l.split_once(' ').expect("Invalid line");

            Robot {
                position: numbers_to_vector(pos).to_point(),
                velocity: numbers_to_vector(v),
            }
        })
        .collect()
}

fn numbers_to_vector(input: &str) -> Vector2D<isize> {
    let (left, right) = input[2..].split_once(',').unwrap();

    vec2(left.parse().unwrap(), right.parse().unwrap())
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
