use itertools::Itertools;
use std::ops::BitXor;

advent_of_code::solution!(17);

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

pub fn part_one(input: &str) -> Option<String> {
    let (registers, program) = parse(input);

    Some(
        run_program(&registers, &program)
            .iter()
            .map(|x| x.to_string())
            .join(","),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    // "cheating" by abusing properties of the input... I'm not sure how easy an input-independent performant solution is
    // noticed that:
    // - the program outputs one character per "iteration"
    // - the values of B and C are determined entirely by the value of A
    // - A decreased by a factor of 8 on each loop (which is why it's useful to think of A as an number in base-8)
    // - so the _last_ digit of our input will necessarily be outputted for a value between 0 and 7
    //      AND will be the _first_ digit of A (in base-8)
    // - and the _second last_ digit outputted will be the _second_ digit of A, and so on
    // - so build up the paths of what these characters can be, in BFS-style

    let (_, program) = parse(input);
    let program_list = program
        .clone()
        .into_iter()
        .flat_map(|(a, b)| [a, b])
        .collect::<Vec<usize>>();

    let mut octal_paths = vec![vec![]];
    for bit_index in 0..program_list.len() {
        let expected_output_digit = program_list[program_list.len() - bit_index - 1];

        octal_paths = octal_paths
            .iter()
            .flat_map(|octal| {
                // find all possible digits, and return these as new octal lists
                find_valid_digits(&program, octal, expected_output_digit)
                    .iter()
                    .map(|d| {
                        // insert this potentially digit as the new first digit in our octal
                        let mut octal = octal.clone();
                        octal.insert(0, *d);

                        octal
                    })
                    .collect::<Vec<Vec<usize>>>()
            })
            .collect()
    }

    octal_paths.iter().min().map(octals_to_int)
}

fn find_valid_digits(
    program: &Vec<(usize, usize)>,
    octals: &Vec<usize>,
    expected: usize,
) -> Vec<usize> {
    let mut octals = octals.clone();
    octals.insert(0, 0);

    (0..8_usize)
        .filter(|&i| {
            octals[0] = i;

            assert!(octals.len() <= 16);

            let registers = vec![octals_to_int(&octals), 0, 0];
            let output = run_program(&registers, program);

            output[0] == expected
        })
        .collect()
}

fn octals_to_int(octals: &Vec<usize>) -> usize {
    octals
        .iter()
        .enumerate()
        .map(|(exp, val)| val * 8_isize.pow(exp as u32) as usize)
        .sum()
}

fn run_program(registers: &Vec<usize>, program: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut registers = registers.clone();
    let mut instruction_pointer = 0;
    let mut output = vec![];

    while let Some((opcode, operand)) = program.get(instruction_pointer) {
        let combo_operand: usize = match operand {
            0..=3 => *operand,
            4 => registers[A],
            5 => registers[B],
            6 => registers[C],
            _ => {
                panic!("Unexpected operand {operand}!")
            }
        };

        match opcode {
            0 => registers[A] = adv(&registers, combo_operand),
            1 => registers[B] = registers[B].bitxor(operand),
            2 => registers[B] = combo_operand % 8,
            3 => {
                if registers[A] != 0 {
                    instruction_pointer = operand / 2;

                    // do not increase the instruction pointer
                    continue;
                }
            }
            4 => registers[B] = registers[B].bitxor(registers[C]),
            5 => output.push(combo_operand % 8),
            6 => registers[B] = adv(&registers, combo_operand),
            7 => registers[C] = adv(&registers, combo_operand),
            _ => {
                panic!("Unexpected instruction {opcode}!")
            }
        };

        instruction_pointer += 1;
    }

    output
}

fn adv(registers: &Vec<usize>, combo_operand: usize) -> usize {
    registers[A] / 2_usize.pow(combo_operand as u32)
}

fn parse(input: &str) -> (Vec<usize>, Vec<(usize, usize)>) {
    let (registers, program) = input.split_once("\n\n").unwrap();

    let registers = registers
        .lines()
        .take(3)
        .flat_map(|l| l[12..].parse())
        .collect::<Vec<usize>>();

    let program = program[9..]
        .split([',', '\n'])
        .flat_map(|d| d.parse())
        .collect::<Vec<usize>>();

    let instructions = (0..program.len() / 2)
        .map(|i| (program[i * 2], program[(i * 2) + 1]))
        .collect::<Vec<(usize, usize)>>();

    (registers, instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));

        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}
