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

fn parse2(input: &str) -> VecDeque<(String, String)> {
    let (_, ops) = input.split_once("\n\n").unwrap();

    ops.lines()
        .map(|x| {
            let (op, address) = x.split_once(" -> ").unwrap();

            (op.to_owned(), address.to_owned())
        })
        .collect::<VecDeque<(String, String)>>()
}

pub fn part_two(input: &str) -> Option<String> {
    let (nodes, addresses, _) = parse(input);
    let invalid = find_wrong_nodes(&nodes, &addresses);

    Some(invalid.iter().sorted().join(","))
}

fn generate_correct_input(order: usize) -> HashMap<String, String> {
    let mut operations = HashMap::new();

    operations.insert("x00 XOR y00".to_owned(), "z00".to_owned());
    operations.insert("x00 AND y00".to_owned(), "carry01".to_owned());

    // z03 => ((x03 XOR y03) XOR ((x02 AND y02) OR ((x02 XOR y02) AND ((x01 AND y01) OR ((x01 XOR y01) AND (x00 AND y00))))))

    // z03 => (xor03 XOR (and02 OR (xor02 AND and

    for digit in 1..order {
        let prev = format!("{:0>2}", digit - 1);
        let curr = format!("{:0>2}", digit);
        let next = format!("{:0>2}", digit + 1);

        let xor_curr = format!("x{curr} XOR y{curr}");
        let output = format!("xor{curr} XOR carry{prev}");
        let and_curr = format!("x{curr} AND y{curr}");
        let yeet_through = format!("{output} XOR carry{prev}");
        let carry_next = format!("and{curr} OR yeet{curr}");

        operations.insert(xor_curr, format!("xor{curr}"));
        operations.insert(output, format!("z{curr}"));
        operations.insert(and_curr, format!("and{curr}"));
        operations.insert(yeet_through, format!("yeet{curr}"));
        operations.insert(carry_next, format!("carry{next}"));
    }

    operations
}

fn try_swap(input: &str, swap: (String, String)) -> Option<usize> {
    // swap the swaps and see what the new least-digit is
    let swap_0_token = format!("-> {}", swap.0);
    let swap_1_token = format!("-> {}", swap.1);

    let mut input = input.to_string();

    input = input.replace(swap_0_token.as_str(), "0_placeholder");
    input = input.replace(swap_1_token.as_str(), swap_0_token.as_str());
    input = input.replace("0_placeholder", swap_1_token.as_str());

    let (nodes, addresses, address_map) = parse(&input);

    let x = collated_values('x', &nodes, &address_map).unwrap();
    let y = collated_values('y', &nodes, &address_map).unwrap();
    let expected = x + y;
    let actual = collated_values('z', &nodes, &address_map).unwrap();

    let bit_diffs = to_bin(actual.bitxor(expected));

    // return the smallest wrong bit first
    bit_diffs
        .chars()
        .rev()
        .enumerate()
        .find_map(|(i, bit)| if bit == '1' { Some(i) } else { None })
}

fn find_swap(input: &str) -> Option<(String, String)> {
    let (nodes, addresses, address_map) = parse(input);
    let operation_to_address = build_operations_map(&nodes, &addresses);

    let mut check_digit = 0;
    while let Some(&check_address) = address_map.get(&format!("z{:0>2}", check_digit)) {
        let check_node = nodes[check_address].clone();
        let actual_eq = check_node.to_string(&nodes, &addresses);
        let expected_eq = digit_operation(check_digit, &mut HashSet::new());

        if actual_eq == expected_eq {
            println!("Digit {} is good", check_digit);
        } else if let Some(wanted_op) = operation_to_address.get(&expected_eq) {
            println!(
                "Digit {} is not good... checking for the correct operation",
                check_digit
            );

            return Some((wanted_op.clone(), addresses[check_address].clone()));
        } else {
            println!("Digit {} is not good... can't find correct op", check_digit);
        }

        check_digit += 1;
    }

    None
}

fn build_operations_map(nodes: &Vec<Node>, addresses: &Vec<String>) -> HashMap<String, String> {
    nodes
        .iter()
        .map(|n| (n.to_string(nodes, addresses), addresses[n.address].clone()))
        .collect::<HashMap<String, String>>()
}

fn digit_operation(digit: usize, operations: &mut HashSet<String>) -> String {
    if digit > 0 {
        let xor = format!("(x{:0>2} XOR y{:0>2})", digit, digit);

        operations.insert(xor.clone());
        format!("({xor} XOR {})", digit_overflow(digit - 1, operations))
    } else {
        let operation = format!("(x{:0>2} XOR y{:0>2})", digit, digit);

        operations.insert(operation.clone());
        operation
    }
}

fn digit_overflow(digit: usize, operations: &mut HashSet<String>) -> String {
    if digit > 0 {
        let and = format!("(x{:0>2} AND y{:0>2})", digit, digit);
        let xor = format!("(x{:0>2} XOR y{:0>2})", digit, digit);

        operations.insert(and.clone());
        operations.insert(xor.clone());

        format!(
            "({and} OR ({xor} AND {}))",
            digit_overflow(digit - 1, operations)
        )
    } else {
        let operation = format!("(x{:0>2} AND y{:0>2})", digit, digit);

        operations.insert(operation.clone());
        operation
    }
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

    // wrong = set()
    // for op1, op, op2, res in operations:
    //     if res[0] == "z" and op != "XOR" and res != highest_z:
    //         wrong.add(res)
    //     if (
    //         op == "XOR"
    //         and res[0] not in ["x", "y", "z"]
    //         and op1[0] not in ["x", "y", "z"]
    //         and op2[0] not in ["x", "y", "z"]
    //     ):
    //         wrong.add(res)
    //     if op == "AND" and "x00" not in [op1, op2]:
    //         for subop1, subop, subop2, subres in operations:
    //             if (res == subop1 or res == subop2) and subop != "OR":
    //                 wrong.add(res)
    //     if op == "XOR":
    //         for subop1, subop, subop2, subres in operations:
    //             if (res == subop1 or res == subop2) and subop == "OR":
    //                 wrong.add(res)
}

fn fix_nodes(input: &str) -> (String, String) {
    let (nodes, addresses, address_map) = parse(input);
    let x = collated_values('x', &nodes, &address_map).unwrap();
    let y = collated_values('y', &nodes, &address_map).unwrap();
    let expected = x + y;

    let actual_sum = collated_values('z', &nodes, &address_map).unwrap();
    println!("exp: {}\nact: {}", to_bin(actual_sum), to_bin(expected));

    let bit_diffs = to_bin(actual_sum.bitxor(expected));

    // fix the smallest wrong bit first
    let index = bit_diffs
        .chars()
        .rev()
        .enumerate()
        .find_map(|(i, bit)| if bit == '1' { Some(i) } else { None })
        .unwrap();

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

    let mut fixed = false;
    for a_node in broken_node.get_chain(&nodes).iter() {
        if fixed {
            break;
        }

        for b_node in nodes.iter() {
            if a_node.address == b_node.address {
                continue;
            }

            let swap_a = addresses[a_node.address].clone();
            let swap_b = addresses[b_node.address].clone();

            let swap = (swap_a, swap_b);
            if let Some(new_smallest) = try_swap(input, swap.clone()) {
                if new_smallest < index {
                    println!(
                        "Found a good swap at index {index}, swap {} for {}",
                        swap.0, swap.1
                    );

                    return swap;
                } else {
                    // bad swap.. keep trying!
                }
            } else {
                // fixed?
                fixed = true;
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
}
