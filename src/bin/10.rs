use itertools::Itertools;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let height_map = parse_map(input);

    let trail_head_total = height_map
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, height)| **height == 0)
                .map(|(c, _)| trail_head_score((r, c), &height_map, true))
                .sum::<u32>()
        })
        .sum::<u32>();

    Some(trail_head_total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let height_map = parse_map(input);

    let trail_head_total = height_map
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, height)| **height == 0)
                .map(|(c, _)| trail_head_score((r, c), &height_map, false))
                .sum::<u32>()
        })
        .sum::<u32>();

    Some(trail_head_total)
}

fn trail_head_score(pos: (usize, usize), height_map: &Vec<Vec<u8>>, unique: bool) -> u32 {
    let mut paths = vec![pos];

    let mut value = 0;
    while !paths.is_empty() && value != 9 {
        paths = paths
            .into_iter()
            .map(|node| get_successors(node, height_map))
            .filter(|p| !p.is_empty())
            .flatten()
            .collect();

        if unique {
            paths = paths.into_iter().unique().collect()
        }

        value += 1;
    }

    paths.len() as u32
}

fn get_successors(pos: (usize, usize), height_map: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let pos_val = height_map[pos.0][pos.1];

    let deltas: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    deltas
        .iter()
        .filter_map(|&(dr, dc)| {
            if dr == -1 && pos.0 == 0 {
                return None;
            }

            if dc == -1 && pos.1 == 0 {
                return None;
            }

            let (nr, nc) = (
                (pos.0 as isize + dr) as usize,
                (pos.1 as isize + dc) as usize,
            );

            if nr >= height_map.len() {
                None
            } else if nc >= height_map[nr].len() {
                None
            } else if height_map[nr][nc] != pos_val + 1 {
                None
            } else {
                Some((nr, nc))
            }
        })
        .collect::<Vec<(usize, usize)>>()
}

fn parse_map(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|ch| ch.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
