use euclid::default::{Point2D, Vector2D};
use euclid::{point2, vec2};
use std::collections::HashMap;

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

pub fn part_two(input: &str) -> Option<usize> {
    let (robots, bounds) = parse(input);

    let x_offset = find_dim_frequency(
        robots
            .iter()
            .map(|robot| (robot.position.x, robot.velocity.x))
            .collect::<Vec<(isize, isize)>>(),
        bounds.x,
    );
    let y_offset = find_dim_frequency(
        robots
            .iter()
            .map(|robot| (robot.position.y, robot.velocity.y))
            .collect::<Vec<(isize, isize)>>(),
        bounds.y,
    );

    frequency_alignment(bounds.x as usize, x_offset, bounds.y as usize, y_offset)
}

fn frequency_alignment(
    freq_a: usize,
    offset_a: usize,
    freq_b: usize,
    offset_b: usize,
) -> Option<usize> {
    (0..freq_b).find_map(|bx| {
        let alignment = offset_a + (freq_a * bx);

        if (alignment - offset_a) % freq_a == 0 && (alignment - offset_b) % freq_b == 0 {
            Some(alignment)
        } else {
            None
        }
    })
}

fn find_dim_frequency(particles: Vec<(isize, isize)>, bound: isize) -> usize {
    let mut particles = particles;

    let mut best_grouping = (0, 0);
    for i in 0..bound {
        let mut row_density = vec![0_u8; bound as usize];

        particles
            .iter()
            .for_each(|particle| row_density[particle.0 as usize] += 1);

        let best_position = row_density.into_iter().max().unwrap();
        if best_position >= best_grouping.1 {
            best_grouping = (i, best_position)
        }

        particles = particles
            .into_iter()
            .map(|p| ((p.0 + p.1).rem_euclid(bound), p.1))
            .collect();
    }

    best_grouping.0 as usize
}

fn get_quadrants(robots: &Vec<Robot>, bounds: &Point2D<isize>) -> Vec<u32> {
    robots
        .iter()
        .fold([0, 0, 0, 0, 0], |mut acc: [u32; 5], robot| {
            acc[robot.quadrant(bounds)] += 1;

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

fn _print(robots: &Vec<Robot>, bounds: &Point2D<isize>) {
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
        assert_eq!(result, Some(31));
    }
}
