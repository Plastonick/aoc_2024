use cached::proc_macro::cached;
use euclid::{point2, Point2D};
use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(21);

type Point = Point2D<isize, isize>;
type Keypad = HashMap<char, Point>;
type KeypadRoutes = HashMap<(char, char), Vec<Vec<char>>>;

pub fn part_one(input: &str) -> Option<usize> {
    Some(get_code_score(input, 2))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(get_code_score(input, 25))
}

fn get_code_score(input: &str, d_pad_robots: usize) -> usize {
    let codes: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let (input_map, directional_map) = build_route_maps();

    let mut score = 0;
    for code in codes.iter() {
        let numeric_score = code_numeric_score(code);

        let input_paths = paths_for(&code, &input_map);
        let code_best_human_length = input_paths
            .iter()
            .map(|p| {
                std::iter::once(&'A')
                    .chain(p.iter())
                    .tuple_windows()
                    .map(|(a, b)| solve_min_path(*a, *b, &directional_map, d_pad_robots - 1))
                    .sum::<usize>()
            })
            .min()
            .unwrap();

        score += numeric_score * code_best_human_length;
    }
    score
}

#[cached(key = "String", convert = r#"{ format!("{}{}{}", from, to, depth) }"#)]
fn solve_min_path(from: char, to: char, routes: &KeypadRoutes, depth: usize) -> usize {
    let next_paths = routes.get(&(from, to)).unwrap();
    let best_length = next_paths.iter().map(|p| p.len()).min().unwrap();

    if depth == 0 {
        best_length
    } else {
        next_paths
            .into_iter()
            .filter(|p| p.len() == best_length)
            .map(|p| {
                std::iter::once(&'A')
                    .chain(p.iter())
                    .tuple_windows()
                    .map(|(a, b)| solve_min_path(*a, *b, &routes, depth - 1))
                    .sum()
            })
            .min()
            .unwrap()
    }
}

fn paths_for(code: &Vec<char>, keypad_map: &KeypadRoutes) -> Vec<Vec<char>> {
    let mut paths: Vec<Vec<char>> = vec![vec![]];

    std::iter::once(&'A')
        .chain(code.iter())
        .tuple_windows()
        .for_each(|(&from, &to)| {
            let possible_segments = keypad_map.get(&(from, to)).unwrap();

            paths = possible_segments
                .into_iter()
                .flat_map(|segment| {
                    paths.clone().into_iter().map(move |p| {
                        p.clone()
                            .into_iter()
                            .chain(segment.iter().copied())
                            .collect()
                    })
                })
                .collect::<Vec<Vec<char>>>();
        });

    paths
}

fn generate_routes(keypad: &Keypad) -> KeypadRoutes {
    keypad
        .iter()
        .flat_map(|(&from, &from_pos)| {
            keypad
                .iter()
                .map(|(&to, &to_pos)| {
                    let delta = to_pos - from_pos;

                    let up_down_ch = if delta.x < 0 { '^' } else { 'v' };
                    let left_right_ch = if delta.y > 0 { '>' } else { '<' };

                    let vert = vec![up_down_ch; delta.x.abs() as usize];
                    let horz = vec![left_right_ch; delta.y.abs() as usize];

                    let horz_first = vec![horz.clone(), vert.clone(), vec!['A']].concat();
                    let vert_first = vec![vert, horz, vec!['A']].concat();

                    // avoid blank space?
                    let blank_pos = keypad.get(&' ').unwrap();
                    let paths = if from_pos.x == blank_pos.x && to_pos.y == blank_pos.y {
                        // currently on same row, going to same column as blank
                        vec![vert_first]
                    } else if from_pos.y == blank_pos.y && to_pos.x == blank_pos.x {
                        // currently on same column, going to same row as blank
                        vec![horz_first]
                    } else {
                        vec![horz_first, vert_first]
                    };

                    ((from, to), paths.into_iter().unique().collect())
                })
                .collect::<HashMap<(char, char), Vec<Vec<char>>>>()
        })
        .collect()
}

fn map_chars(chars: Vec<Vec<char>>) -> Keypad {
    chars
        .into_iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.into_iter()
                .enumerate()
                .map(|(c, ch)| (ch, point2(r as isize, c as isize)))
                .collect::<Vec<(char, Point)>>()
        })
        .collect::<Keypad>()
}

fn code_numeric_score(code: &Vec<char>) -> usize {
    code.iter()
        .filter(|ch| ('0'..='9').contains(ch))
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

fn build_route_maps() -> (KeypadRoutes, KeypadRoutes) {
    let input_keypad = map_chars(vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec![' ', '0', 'A'],
    ]);

    //   ^ A
    // < v >
    let directional_keypad = map_chars(vec![vec![' ', '^', 'A'], vec!['<', 'v', '>']]);

    let input_map = generate_routes(&input_keypad);
    let directional_map = generate_routes(&directional_keypad);

    (input_map, directional_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154115708116294));
    }
}
