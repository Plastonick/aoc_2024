use crate::Value::{Derived, Raw};
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::ops::{BitAnd, BitOr, BitXor};

advent_of_code::solution!(24);

#[derive(Copy, Clone, Debug)]
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

    fn get_chain(self, nodes: &Vec<Node>) -> Vec<Node> {
        match self.value {
            Derived(process) => vec![
                nodes[process.input1].clone().get_chain(&nodes),
                nodes[process.input2].clone().get_chain(&nodes),
            ]
            .concat(),
            Raw(_) => {
                vec![self]
            }
        }
    }

    fn to_string(&self, nodes: &Vec<Node>, addresses: &Vec<String>) -> String {
        match &self.value {
            Derived(process) => {
                let node1 = nodes.get(process.input1).unwrap();
                let node2 = nodes.get(process.input2).unwrap();
                let op = match process.operation {
                    Operation::And => "AND",
                    Operation::Or => "OR",
                    Operation::Xor => "XOR",
                };

                format!(
                    "({} {} {})",
                    node1.to_string(nodes, addresses),
                    op,
                    node2.to_string(nodes, addresses)
                )
            }
            Raw(_) => addresses[self.address].to_string(),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (nodes, _, address_map) = parse(input);

    collated_values('z', &nodes, &address_map)
}

fn find_swap(input: &str) -> Option<(String, String)> {
    let (nodes, addresses, address_map) = parse(input);
    let operation_to_address = build_operations_map(&nodes, &addresses);

    let mut check_digit = 0;
    while let Some(&check_address) = address_map.get(&format!("z{:0>2}", check_digit)) {
        let check_node = nodes[check_address].clone();
        let actual_eq = check_node.to_string(&nodes, &addresses);
        let expected_eq = digit_operation(check_digit);

        if actual_eq == expected_eq {
            println!("Digit {} is good", check_digit);
        } else if let Some(wanted_op) = operation_to_address.get(&expected_eq) {
            println!(
                "Digit {} is not good... checking for the correct operation",
                check_digit
            );

            let wanted_op = operation_to_address
                .get(&expected_eq)
                .expect("hoped this would exist")
                .clone();

            return Some((wanted_op.clone(), addresses[check_address].clone()));
        } else {
            println!("Digit {} is not good... can't find correct op", check_digit);
        }

        check_digit += 1;
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input = input.to_string();
    let mut swaps = vec![];
    while let Some(swap) = find_swap(&input) {
        println!("Swapping {} and {}", swap.0, swap.1);

        let swap_0_token = format!("-> {}", swap.0);
        let swap_1_token = format!("-> {}", swap.1);

        input = input.replace(swap_0_token.as_str(), "0_placeholder");
        input = input.replace(swap_1_token.as_str(), swap_0_token.as_str());
        input = input.replace("0_placeholder", swap_1_token.as_str());

        swaps.push(swap);
    }

    dbg!(&swaps);

    None
}

fn build_operations_map(nodes: &Vec<Node>, addresses: &Vec<String>) -> HashMap<String, String> {
    nodes
        .iter()
        .map(|n| (n.to_string(nodes, addresses), addresses[n.address].clone()))
        .collect::<HashMap<String, String>>()
}

fn digit_operation(digit: usize) -> String {
    if digit > 0 {
        format!(
            "((x{:0>2} XOR y{:0>2}) XOR {})",
            digit,
            digit,
            digit_overflow(digit - 1)
        )
    } else {
        format!("(x{:0>2} XOR y{:0>2})", digit, digit)
    }
}

fn digit_overflow(digit: usize) -> String {
    if digit > 0 {
        format!(
            "((x{:0>2} AND y{:0>2}) OR ((x{:0>2} XOR y{:0>2}) AND {}))",
            digit,
            digit,
            digit,
            digit,
            digit_overflow(digit - 1)
        )
    } else {
        format!("(x{:0>2} AND y{:0>2})", digit, digit)
    }
}

fn fix_nodes(
    nodes: Vec<Node>,
    address_map: &HashMap<String, usize>,
    addresses: &Vec<String>,
    expected: usize,
) -> Option<Vec<Node>> {
    let actual_sum = collated_values('z', &nodes, &address_map).unwrap();
    println!("exp: {}\nact: {}", to_bin(actual_sum), to_bin(expected));

    let bit_diffs = to_bin(actual_sum.bitxor(expected));

    // fix the smallest wrong bit first
    let index =
        bit_diffs
            .chars()
            .rev()
            .enumerate()
            .find_map(|(i, bit)| if bit == '1' { Some(i) } else { None })?;

    // we have a smallest wrong bit... so something in the node-chain for this one is wrong, but the node-chain
    // below it _isn't_ wrong.
    // find the difference!

    println!(
        "smallest wrong bit {} shouldn't be {} in bit_xor {}",
        index,
        bit_diffs.get(index..index + 1).unwrap(),
        bit_diffs
    );

    let node_address_str = format!("z{:0>2}", index);
    println!("bad address: {}", node_address_str);

    let broken_node_address = address_map.get(&node_address_str).unwrap();
    let broken_node = nodes[*broken_node_address].clone();

    for a_node in broken_node.get_chain(&nodes).iter() {
        for b_node in nodes.iter() {
            if a_node.address == b_node.address {
                continue;
            }

            let new_a = Node {
                address: a_node.address,
                value: b_node.value.clone(),
            };
            let new_b = Node {
                address: b_node.address,
                value: a_node.value.clone(),
            };

            // try a swap?
            let mut new_nodes = nodes.clone();
            new_nodes[a_node.address] = new_a;
            new_nodes[b_node.address] = new_b;

            let Some(new_value) = collated_values('z', &new_nodes, &address_map) else {
                // possible invalid swap due to infinite loop?

                continue;
            };
            println!("new: {}\nact: {}", to_bin(new_value), to_bin(expected));

            let bit_diffs = to_bin(actual_sum.bitxor(expected));
            let new_least_index = bit_diffs.chars().rev().enumerate().find_map(|(i, bit)| {
                if bit == '1' {
                    Some(i)
                } else {
                    None
                }
            });

            let Some(new_least_index) = new_least_index else {
                // hurrah! We've totally fixed it. Return our new nodes!

                println!("We've fixed it????");
                return Some(new_nodes);
            };

            // we've not fixed it, but have we improved it?
            if new_least_index > index {
                // it's better! Consider it an improvement and move along...

                println!(
                    "Improved by switching {} and {}",
                    addresses[a_node.address], addresses[b_node.address]
                );
                panic!();
                return Some(new_nodes);
            } else {
                println!(
                    "it's worse! {} instead of {} Try again...",
                    new_least_index, index
                );
            }
        }
    }

    unreachable!();
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1));
    }
}
