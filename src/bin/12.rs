use std::collections::{HashMap, HashSet};

advent_of_code::solution!(12);

type Garden = HashMap<(i32, i32), char>;

pub fn part_one(input: &str) -> Option<i32> {
    let garden = parse(input);
    let garden_score = get_regions(&garden)
        .iter()
        .map(|(region, perimeter, _)| region.len() as i32 * perimeter)
        .sum();

    Some(garden_score)
}

pub fn part_two(input: &str) -> Option<i32> {
    let garden = parse(input);
    let garden_score = get_regions(&garden)
        .iter()
        .map(|(region, _, _)| {
            let num_sides = get_sides(region);

            region.len() as i32 * num_sides
        })
        .sum();

    Some(garden_score)
}

fn get_regions(garden: &Garden) -> Vec<(HashSet<(i32, i32)>, i32, char)> {
    let mut visited = HashSet::new();
    let mut regions = vec![];

    for plot in garden.keys() {
        if visited.contains(plot) {
            continue;
        }

        let ch = *garden
            .get(plot)
            .unwrap_or_else(|| panic!("No plot at position ({}, {})!", plot.0, plot.1));
        let (region, perimeter) = get_region(plot, garden);
        regions.push((region.clone(), perimeter, ch));

        for plot in region {
            visited.insert(plot);
        }
    }

    regions
}

fn get_sides(region: &HashSet<(i32, i32)>) -> i32 {
    let max_bounds = region
        .clone()
        .into_iter()
        .reduce(|a, b| (a.0.max(b.0), a.1.max(b.1)))
        .unwrap();

    let min_bounds = region
        .clone()
        .into_iter()
        .reduce(|a, b| (a.0.min(b.0), a.1.min(b.1)))
        .unwrap();

    let mut horz_sides = 0;
    for r in min_bounds.0 - 1..=max_bounds.0 {
        let mut previous_up_exists = false;
        let mut previous_down_exists = false;

        for c in min_bounds.1..=max_bounds.1 {
            // is there a border here?
            let up_exists = region.contains(&(r, c));
            let down_exists = region.contains(&(r + 1, c));
            let edge_exists = up_exists != down_exists;
            let new_edge = previous_up_exists != up_exists || previous_down_exists != down_exists;

            if new_edge && edge_exists {
                // we've just entered a new edge! Add it!
                horz_sides += 1;
            }

            previous_up_exists = up_exists;
            previous_down_exists = down_exists;
        }
    }

    // number of horizontal sides == number of vertical sides, so double this figure!
    horz_sides * 2
}

fn get_region(initial: &(i32, i32), garden: &Garden) -> (HashSet<(i32, i32)>, i32) {
    let mut region = HashSet::new();
    region.insert(*initial);
    let mut wave = vec![*initial];

    let mut perimeter = 0;
    while !wave.is_empty() {
        let mut new_wave = vec![];

        for plot in wave {
            let neighbours = get_matching_neighbours(&plot, garden);

            // the perimeter of a given plot is the number of neighbouring elements that _don't_ match its type
            perimeter += 4 - neighbours.len() as i32;

            for neighbour in neighbours {
                // if the neighbour is already in our region, ignore it and carry on
                if region.contains(&neighbour) {
                    continue;
                }

                // we've not seen this plot yet, add it to the next wave and region
                new_wave.push(neighbour);
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
        .flat_map(|(r, row)| {
            row.chars()
                .enumerate()
                .map(|(c, ch)| ((r as i32, c as i32), ch))
                .collect::<Vec<((i32, i32), char)>>()
        })
        .collect::<Garden>()
}

fn get_matching_neighbours(of: &(i32, i32), garden: &Garden) -> Vec<(i32, i32)> {
    const DELTAS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let plot_type = garden.get(of).expect("Couldn't find plot type");

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
