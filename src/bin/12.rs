use std::collections::{HashMap, HashSet};

advent_of_code::solution!(12);

type Garden = HashMap<(i32, i32), char>;

pub fn part_one(input: &str) -> Option<i32> {
    let garden = parse(input);
    let mut visited = HashSet::new();
    let mut garden_score = 0;
    for plot in garden.keys() {
        if visited.contains(plot) {
            continue;
        }

        let (region, perimeter) = get_region(plot, &garden);
        garden_score += region.len() as i32 * perimeter;

        for plot in region {
            visited.insert(plot);
        }
    }

    Some(garden_score)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn get_region(initial: &(i32, i32), garden: &Garden) -> (HashSet<(i32, i32)>, i32) {
    let mut region = HashSet::new();
    region.insert(*initial);
    let mut wave = HashSet::new();
    wave.insert(*initial);

    let mut perimeter = 0;
    while wave.len() > 0 {
        let mut new_wave = HashSet::new();

        for plot in wave {
            let neighbours = get_matching_neighbours(&plot, &garden);

            // the perimeter of a given plot is the number of neighbouring elements that _don't_ match its type
            perimeter += 4 - neighbours.len() as i32;

            for neighbour in neighbours {
                // if the neighbour is already in our region, ignore it and carry on
                if region.contains(&neighbour) {
                    continue;
                }

                // we've not seen this plot yet, add it to the next wave and region
                new_wave.insert(neighbour);
                region.insert(neighbour);
            }
        }

        wave = new_wave;
    }

    (region, perimeter)
}

fn parse(input: &str) -> Garden {
    input
        .lines()
        .enumerate()
        .map(|(r, row)| {
            row.chars()
                .enumerate()
                .map(|(c, ch)| ((r as i32, c as i32), ch))
                .collect::<Vec<((i32, i32), char)>>()
        })
        .flatten()
        .collect::<Garden>()
}

fn get_matching_neighbours(of: &(i32, i32), garden: &Garden) -> Vec<(i32, i32)> {
    const DELTAS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let plot_type = garden.get(&of).expect("Couldn't find plot type");

    DELTAS
        .iter()
        .filter_map(|delta| {
            let pos = (of.0 + delta.0, of.1 + delta.1);
            if garden.get(&pos) == Some(plot_type) {
                Some(pos)
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
