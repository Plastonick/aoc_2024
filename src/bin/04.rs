use std::collections::HashSet;

advent_of_code::solution!(4);

type Grid = Vec<Vec<char>>;

pub fn part_one(input: &str) -> Option<usize> {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Grid>();

    let word = vec!['X', 'M', 'A', 'S'];

    let instances = find_instances(&grid, &word);

    Some(instances)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Grid>();

    // find 'A' positions
    let a_positions = grid
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(c, ch)| if *ch == 'A' { Some((r, c)) } else { None })
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .filter(|(r, c)| {
            // only As just off the edge of the grid are possible
            *r > 0 && *c > 0 && *r < grid.len() - 1 && *c < grid[*r].len() - 1
        })
        .collect::<Vec<(usize, usize)>>();

    let mut x_mas_count = 0;

    let expected_char_set = HashSet::from(['M', 'S']);

    // grab corners and verify mmss in some order
    for (r, c) in a_positions {
        let diag1 = HashSet::from([grid[r - 1][c - 1], grid[r + 1][c + 1]]);
        let diag2 = HashSet::from([grid[r + 1][c - 1], grid[r - 1][c + 1]]);

        if diag1.eq(&expected_char_set) && diag2.eq(&expected_char_set) {
            x_mas_count += 1;
        }
    }

    Some(x_mas_count)
}

fn find_instances(grid: &Grid, word: &Vec<char>) -> usize {
    let mut total = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            total += matching_directions(&grid, (i, j), &word);
        }
    }

    total
}

fn matching_directions(grid: &Grid, starts_from: (usize, usize), word: &Vec<char>) -> usize {
    let directions = (-1..=1)
        .map(|r| (-1..=1).map(|c| (r, c)).collect::<Vec<(isize, isize)>>())
        .flatten()
        .filter(|(r, c)| *r != 0 || *c != 0)
        .collect::<Vec<(isize, isize)>>();

    directions
        .iter()
        .filter(|d| word_found_at(&grid, starts_from, **d, word))
        .count()
}

fn word_found_at(
    grid: &Grid,
    starts_from: (usize, usize),
    direction: (isize, isize),
    word: &Vec<char>,
) -> bool {
    for i in 0..word.len() {
        let row = starts_from.0 as isize + (direction.0 * i as isize);
        let col = starts_from.1 as isize + (direction.1 * i as isize);

        if row < 0
            || col < 0
            || row >= grid.len() as isize
            || col >= grid[row as usize].len() as isize
        {
            return false;
        }

        if grid[row as usize][col as usize] != word[i] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
