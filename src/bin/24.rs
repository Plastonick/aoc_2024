use crate::Value::{Derived, Raw};
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::ops::{BitAnd, BitOr, BitXor};

advent_of_code::solution!(24);

#[derive(Copy, Clone)]
enum Operation {
    And,
    Or,
    Xor,
}

#[derive(Clone)]
struct Process {
    input1: Box<Node>,
    input2: Box<Node>,
    operation: Operation,
}

#[derive(Clone)]
enum Value {
    Derived(Process),
    Raw(usize),
}

#[derive(Clone)]
struct Node {
    address: usize,
    value: Value,
}

impl Node {
    fn derive_value(&self) -> usize {
        match &self.value {
            Raw(value) => *value,
            Derived(process) => match process.operation {
                Operation::And => process
                    .input1
                    .derive_value()
                    .bitand(process.input2.derive_value()),
                Operation::Or => process
                    .input1
                    .derive_value()
                    .bitor(process.input2.derive_value()),
                Operation::Xor => process
                    .input1
                    .derive_value()
                    .bitxor(process.input2.derive_value()),
            },
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (nodes, address_map) = parse(input);

    Some(collated_values('z', &nodes, &address_map))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (nodes, address_map) = parse(input);
    let x = collated_values('x', &nodes, &address_map);
    let y = collated_values('y', &nodes, &address_map);
    let z = collated_values('z', &nodes, &address_map);
    let expected_sum = x + y;
    let actual_sum = z;

    println!(
        "Expected {} but got {} for a diff of {}",
        expected_sum,
        actual_sum,
        to_bin(expected_sum.abs_diff(actual_sum))
    );

    // let composed_values = composed_values(values.clone(), processes.clone());
    //
    // dbg!(composed_values
    //     .into_iter()
    //     .filter(|(k, _)| k.starts_with('z'))
    //     .map(|(k, v)| format!("{k} => {v}"))
    //     .sorted()
    //     .collect::<Vec<String>>());
    //
    // let values = run_program(values, processes);
    // let z = calc_value(&values, 'z');
    //
    // let expected = x + y;
    // let diff = expected.bitxor(z);
    //
    // println!("{x} + {y} = {z}");
    // println!("Gives us a diff of {}", to_bin(diff));

    // let values = run_program(values, processes.clone());
    // let expected = x + y;
    //
    // let mut processes = processes;
    // while let Some((p1, p2, improvement)) = find_best_switch(&values, &processes, expected) {
    //     println!(
    //         "Switching {} and {} for an improvement of {}",
    //         p1, p2, improvement
    //     );
    //     let p2_buffer = processes.remove(p2);
    //     let p1_buffer = processes.remove(p1);
    //
    //     let p2_target = p2_buffer.target.clone();
    //     let p1_target = p1_buffer.target.clone();
    //
    //     let process1 = Process {
    //         target: p2_target,
    //         ..p1_buffer
    //     };
    //     let process2 = Process {
    //         target: p1_target,
    //         ..p2_buffer
    //     };
    //
    //     processes.push(process2);
    //     processes.push(process1);
    // }

    None
}

fn collated_values(prefix: char, nodes: &Vec<Node>, address_map: &HashMap<String, usize>) -> usize {
    address_map
        .iter()
        .filter(|(k, _)| k.starts_with(prefix))
        .sorted_by(|(a, _), (b, _)| a.cmp(b))
        .map(|(_, &address)| nodes[address].derive_value())
        .enumerate()
        .map(|(ord, value)| value * 2_usize.pow(ord as u32))
        .sum()
}

// fn find_best_switch(
//     values: &HashMap<String, usize>,
//     processes: &Vec<Process>,
//     expected: usize,
// ) -> Option<(usize, usize, usize)> {
//     let mut best: Option<(usize, usize, usize)> = None;
//
//     for (p1, process1) in processes.iter().enumerate() {
//         for (p2, process2) in processes.iter().enumerate() {
//             if p1 == p2 {
//                 continue;
//             }
//
//             let mut new_processes = processes.clone();
//             let process1 = Process {
//                 input1: process1.input1.to_owned(),
//                 input2: process1.input2.to_owned(),
//                 operation: process1.operation.to_owned(),
//                 target: process2.target.to_owned(),
//             };
//             let process2 = Process {
//                 input1: process2.input1.to_owned(),
//                 input2: process2.input2.to_owned(),
//                 operation: process2.operation.to_owned(),
//                 target: process1.target.to_owned(),
//             };
//
//             new_processes[p1] = process1;
//             new_processes[p2] = process2;
//
//             let values = run_program(values.clone(), new_processes);
//
//             let new_z = calc_value(&values, 'z');
//             let new_diff = expected.abs_diff(new_z);
//
//             if new_diff < best.map_or(usize::MAX, |x| x.2) {
//                 best = Some((p1.min(p2), p1.max(p2), new_diff));
//             }
//         }
//     }
//
//     best
// }

// fn composed_values(
//     values: HashMap<String, usize>,
//     processes: Vec<Vec<String>>,
// ) -> HashMap<String, String> {
//     let mut processes = processes;
//     let mut composed_values = values
//         .keys()
//         .map(|k| (k.to_owned(), k.to_owned()))
//         .collect::<HashMap<String, String>>();
//
//     while let Some(process) = processes.pop() {
//         let Some(arg1) = composed_values.get(&process[0]) else {
//             processes.insert(0, process);
//
//             continue;
//         };
//         let Some(arg2) = composed_values.get(&process[2]) else {
//             processes.insert(0, process);
//
//             continue;
//         };
//
//         let operation = &process[1];
//         let target = &process[4];
//
//         composed_values.insert(target.to_string(), format!("({arg1} {operation} {arg2})"));
//     }
//
//     composed_values
// }

fn to_bin(int: usize) -> String {
    let mut binary = String::new();
    let mut int = int;

    while int > 0 {
        let digit = if int % 2 == 0 { '0' } else { '1' };
        binary.insert(0, digit);
        int /= 2;
    }

    binary
}

fn calc_value(values: &HashMap<String, usize>, prefix: char) -> usize {
    values
        .keys()
        .sorted()
        .filter(|k| k.starts_with(prefix))
        .enumerate()
        .map(|(ord, k)| {
            let bin_digit = values.get(k).unwrap();
            bin_digit * 2_usize.pow(ord as u32)
        })
        .sum::<usize>()
}

fn parse(input: &str) -> (Vec<Node>, HashMap<String, usize>) {
    let mut address_map: HashMap<String, usize> = HashMap::new();
    let mut nodes = Vec::with_capacity(input.lines().count());
    let mut lines = input
        .lines()
        .filter(|l| !l.is_empty())
        .collect::<VecDeque<&str>>();

    while let Some(line) = lines.pop_front() {
        if line.contains(":") {
            let (address_str, value) = line.split_once(": ").unwrap();
            let address = nodes.len();
            address_map.insert(address_str.to_owned(), address);

            nodes.push(Node {
                address,
                value: Raw(value.parse::<usize>().unwrap()),
            })
        } else {
            let parts = line.split(" ").collect::<Vec<&str>>();

            let Some(&address1) = address_map.get(parts[0]) else {
                lines.push_back(line);

                continue;
            };
            let Some(&address2) = address_map.get(parts[2]) else {
                lines.push_back(line);

                continue;
            };

            let address = nodes.len();
            address_map.insert(parts[4].to_owned(), address);

            nodes.push(Node {
                address,
                value: Derived(Process {
                    input1: Box::from(nodes[address1].clone()),
                    input2: Box::from(nodes[address2].clone()),
                    operation: match parts[1] {
                        "AND" => Operation::And,
                        "OR" => Operation::Or,
                        "XOR" => Operation::Xor,
                        _ => {
                            unreachable!()
                        }
                    },
                }),
            })
        }
    }

    (nodes, address_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bin() {
        let cases = [(10, "1010"), (8, "1000"), (100, "1100100")];

        for (int, expected) in cases {
            assert_eq!(to_bin(int), expected.to_string());
        }
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
