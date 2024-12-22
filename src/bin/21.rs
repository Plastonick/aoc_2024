use euclid::{point2, Point2D};
use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(21);

type Point = Point2D<isize, isize>;
type Keypad = HashMap<char, Point>;

pub fn part_one(input: &str) -> Option<usize> {
    let codes: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    // TODO cleverer way...
    // turn the two keypads into maps of ch -> (x, y)
    // directly turn going from 'a' to 'b' into a vector
    // turn that vector into a sequence of directions ^, v, <, >
    // feed it back in

    let input_keypad = map_chars(vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec![' ', '0', 'A'],
    ]);

    //   ^ A
    // < v >
    let directional_keypad = map_chars(vec![vec![' ', '^', 'A'], vec!['<', 'v', '>']]);

    let mut score = 0;
    for code in codes.iter() {
        let numeric_score = code_numeric_score(code);

        let code = vec![vec!['A'], code.clone()].concat(); // we want to start at 'A'

        let robot1_path = path_for(&code, &input_keypad);
        let robot2_path = path_for(
            &vec![vec!['A'], robot1_path.clone()].concat(),
            &directional_keypad,
        );
        let human_path = path_for(
            &vec![vec!['A'], robot2_path.clone()].concat(),
            &directional_keypad,
        );

        println!("Num: {} and path: {}", numeric_score, human_path.len());

        score += numeric_score * human_path.len();
    }

    // 225234 is WRONG
    // 221338 is WRONG
    // 213766 is WRONG

    Some(score)
}

fn path_for(code: &Vec<char>, keypad: &Keypad) -> Vec<char> {
    code.iter()
        .tuple_windows()
        .flat_map(|(from, to)| {
            let from_pos = *keypad.get(from).unwrap();
            let to_pos = *keypad.get(to).unwrap();
            let delta = to_pos - from_pos;

            let up_down_ch = if delta.x < 0 { '^' } else { 'v' };
            let left_right_ch = if delta.y > 0 { '>' } else { '<' };

            let vert = vec![up_down_ch; delta.x.abs() as usize];
            let horz = vec![left_right_ch; delta.y.abs() as usize];

            // avoid blank space?
            let blank_pos = keypad.get(&' ').unwrap();
            if from_pos.x == blank_pos.x && to_pos.y == blank_pos.y {
                // currently on same row, going to same column as blank
                vec![
                    vert,
                    horz,
                    vec!['A'], // need to select it too!
                ]
            } else if left_right_ch == '<' {
                // left before up/down before right
                vec![
                    horz,
                    vert,
                    vec!['A'], // need to select it too!
                ]
            } else {
                vec![
                    vert,
                    horz,
                    vec!['A'], // need to select it too!
                ]
            }
            .concat()
        })
        .collect::<Vec<char>>()
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
