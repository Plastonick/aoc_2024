use crate::Value::{Derived, Raw};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{BitAnd, BitOr, BitXor};

advent_of_code::solution!(24);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Operation {
    And,
    Or,
    Xor,
}

#[derive(Clone, Debug)]
struct Process {
    input1: usize,
    input2: usize,
    operation: Operation,
}

#[derive(Clone, Debug)]
enum Value {
    Derived(Process),
    Raw(usize),
}

#[derive(Clone, Debug)]
struct Node {
    address: usize,
    value: Value,
}

impl Node {
    fn derive_value(&self, nodes: &Vec<Node>, depth: u8) -> Option<usize> {
        if depth == 0 {
            None
        } else {
            Some(match &self.value {
                Raw(value) => *value,
                Derived(process) => match process.operation {
                    Operation::And => nodes[process.input1]
                        .derive_value(&nodes, depth - 1)?
                        .bitand(nodes[process.input2].derive_value(&nodes, depth - 1)?),
                    Operation::Or => nodes[process.input1]
                        .derive_value(&nodes, depth - 1)?
                        .bitor(nodes[process.input2].derive_value(&nodes, depth - 1)?),
                    Operation::Xor => nodes[process.input1]
                        .derive_value(&nodes, depth - 1)?
                        .bitxor(nodes[process.input2].derive_value(&nodes, depth - 1)?),
                },
            })
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (nodes, _, address_map) = parse(input);

    collated_values('z', &nodes, &address_map)
}

pub fn part_two(input: &str) -> Option<String> {
    let (nodes, addresses, _) = parse(input);
    let invalid = find_wrong_nodes(&nodes, &addresses);

    Some(invalid.iter().sorted().join(","))
}

fn any_start_with(strings: Vec<String>, chars: Vec<char>) -> bool {
    strings
        .iter()
        .any(|str| chars.iter().any(|ch| str.starts_with(*ch)))
}

fn find_wrong_nodes(nodes: &Vec<Node>, addresses: &Vec<String>) -> HashSet<String> {
    let operations = nodes
        .iter()
        .filter_map(|n| match &n.value {
            Raw(_) => None,
            Derived(process) => {
                let target = addresses[n.address].clone();
                let input1 = addresses[process.input1].clone();
                let input2 = addresses[process.input1].clone();
                let operation = process.operation;

                Some((target, input1, input2, operation))
            }
        })
        .collect::<Vec<(String, String, String, Operation)>>();

    let mut invalid_addresses = HashSet::new();

    for (target, input1, input2, operation) in operations.iter() {
        let is_xor = operation == &Operation::Xor;

        if target.starts_with('z') && !is_xor && target != "z45" {
            invalid_addresses.insert(target.clone());
        }

        if is_xor
            && !any_start_with(
                vec![target.to_owned(), input1.to_owned(), input2.to_owned()],
                vec!['x', 'y', 'z'],
            )
        {
            invalid_addresses.insert(target.clone());
        }

        if operation == &Operation::And && input1 != "x00" && input2 != "x00" {
            for (_, sub_input1, sub_input2, sub_op) in operations.iter() {
                if (target == sub_input1 || target == sub_input2) && sub_op != &Operation::Or {
                    invalid_addresses.insert(target.clone());
                }
            }
        }

        if operation == &Operation::Xor {
            for (_, sub_input1, sub_input2, sub_op) in operations.iter() {
                if (target == sub_input1 || target == sub_input2) && sub_op == &Operation::Or {
                    invalid_addresses.insert(target.clone());
                }
            }
        }
    }

    invalid_addresses
}

fn collated_values(
    prefix: char,
    nodes: &Vec<Node>,
    address_map: &HashMap<String, usize>,
) -> Option<usize> {
    let values = address_map
        .iter()
        .filter(|(k, _)| k.starts_with(prefix))
        .sorted_by(|(a, _), (b, _)| a.cmp(b))
        .map(|(_, &address)| nodes[address].derive_value(&nodes, 100))
        .collect::<Vec<Option<usize>>>();

    if values.iter().any(|x| x.is_none()) {
        None
    } else {
        Some(
            values
                .iter()
                .enumerate()
                .map(|(ord, value)| value.unwrap() * 2_usize.pow(ord as u32))
                .sum(),
        )
    }
}

fn parse(input: &str) -> (Vec<Node>, Vec<String>, HashMap<String, usize>) {
    let mut address_map: HashMap<String, usize> = HashMap::new();
    let mut addresses = Vec::new();
    let mut nodes = Vec::new();
    let mut lines = input
        .lines()
        .filter(|l| !l.is_empty())
        .collect::<VecDeque<&str>>();

    while let Some(line) = lines.pop_front() {
        if line.contains(":") {
            let (address_str, value) = line.split_once(": ").unwrap();
            let address = nodes.len();
            address_map.insert(address_str.to_owned(), address);
            addresses.push(address_str.to_owned());

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

            // everyone loves a consistent ordering!
            let (address1, address2) = (address1.min(address2), address1.max(address2));

            let address = nodes.len();
            address_map.insert(parts[4].to_owned(), address);
            addresses.push(parts[4].to_owned());

            nodes.push(Node {
                address,
                value: Derived(Process {
                    input1: address1,
                    input2: address2,
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

    (nodes, addresses, address_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }
}
