use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(23);

type Network = HashMap<usize, HashSet<usize>>;

pub fn part_one(input: &str) -> Option<usize> {
    let network = parse(input);
    let t_three_sets = find_t_three_sets(&network);

    Some(t_three_sets.len())
}

pub fn part_two(input: &str) -> Option<String> {
    let network = parse(input);

    let password = bron_kerbosch1(
        HashSet::new(),
        network.keys().copied().collect(),
        HashSet::new(),
        &network,
    )
    .into_iter()
    .reduce(|a, b| if a.len() > b.len() { a } else { b })
    .map(|x| {
        x.into_iter()
            .map(|x| int_to_label(x, ""))
            .sorted()
            .join(",")
    })
    .unwrap();

    Some(password)
}

// ref: https://stackoverflow.com/questions/13904636/implementing-bron-kerbosch-algorithm-in-python
fn bron_kerbosch1(
    r: HashSet<usize>,
    p: HashSet<usize>,
    x: HashSet<usize>,
    g: &Network,
) -> Vec<HashSet<usize>> {
    if p.is_empty() && x.is_empty() {
        return vec![r];
    }

    let mut p_iter = p.clone();
    let mut x = x;

    p.into_iter()
        .flat_map(|v| {
            let nv = g.get(&v).unwrap();

            let mut r_v = r.clone();
            r_v.insert(v);
            let p_nv = p_iter
                .clone()
                .intersection(&nv)
                .copied()
                .collect::<HashSet<usize>>();
            let x_nv = x
                .clone()
                .intersection(&nv)
                .copied()
                .collect::<HashSet<usize>>();

            x.insert(v);
            p_iter.remove(&v);
            bron_kerbosch1(r_v, p_nv, x_nv, &g)
        })
        .collect()
}

fn find_t_three_sets(network: &Network) -> HashSet<Vec<usize>> {
    let t_comps = network
        .keys()
        .filter(|&&computer| starts_with(computer, 't'))
        .copied()
        .collect::<Vec<usize>>();

    // TODO this is ugly... re-create as a dynamic function looking for loops of size n, n-1, ...
    let mut three_sets = HashSet::new();
    for t_comp in t_comps {
        let neighbours = network.get(&t_comp).unwrap();

        // which of these neighbours match each other?
        for neighbour in neighbours.iter() {
            let dist_neighbours = network.get(neighbour).unwrap();

            for dist_neighbour in dist_neighbours {
                if neighbours.contains(dist_neighbour) {
                    // found our three-set
                    three_sets.insert(
                        vec![t_comp, *neighbour, *dist_neighbour]
                            .into_iter()
                            .sorted()
                            .collect(),
                    );
                }
            }
        }
    }

    three_sets
}

fn parse(input: &str) -> Network {
    input
        .lines()
        .flat_map(|l| {
            let (left, right) = l.split_once('-').unwrap();
            let left = label_to_int(left);
            let right = label_to_int(right);

            vec![(left, right), (right, left)]
        })
        .fold(HashMap::new(), |mut acc, (left, right)| {
            let entry = acc.entry(left).or_insert(HashSet::new());
            entry.insert(right);

            acc
        })
}

fn label_to_int(label: &str) -> usize {
    label
        .chars()
        .enumerate()
        .map(|(i, ch)| ch as usize * (u8::MAX as usize).pow(i as u32))
        .sum()
}

fn int_to_label(int: usize, label: &str) -> String {
    let last_digit = int % u8::MAX as usize;
    let last_char = last_digit as u8 as char;

    let mut label = label.to_string();
    label.push(last_char);

    if last_digit == int {
        // that's it!
        label.to_string()
    } else {
        int_to_label(int / u8::MAX as usize, &label)
    }
}

fn starts_with(label: usize, ch: char) -> bool {
    let last = label % u8::MAX as usize;

    last as u8 == ch as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_to_int_and_reverse() {
        let label = "abc";
        let int = label_to_int(label);
        let processed_label = int_to_label(int, "");

        assert_eq!(label, processed_label);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_owned()));
    }
}
