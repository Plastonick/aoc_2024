use itertools::Itertools;
use std::ops::BitXor;

advent_of_code::solution!(17);

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

pub fn part_one(input: &str) -> Option<String> {
    let (mut registers, program) = parse(input);
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

    Some(output.iter().map(|x| x.to_string()).join(","))
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
