use advent_of_code::transpose2;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<usize> {
    let (locks, keys) = parse(input);

    Some(
        locks
            .iter()
            .map(|l| keys.iter().filter(|k| fits(l, k)).count())
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None // no part 2 on 25
}

fn fits(lock: &Vec<usize>, key: &Vec<usize>) -> bool {
    lock.iter().enumerate().all(|(i, &gap)| key[i] <= gap) // tooth <= gap
}

fn parse(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let key_locks = input.split("\n\n").collect::<Vec<&str>>();

    let locks = key_locks
        .iter()
        .filter(|m| m.starts_with('#'))
        .fold(Vec::new(), |acc: Vec<Vec<usize>>, k| keyed(acc, k));

    let keys = key_locks
        .iter()
        .filter(|m| m.starts_with('.'))
        .fold(Vec::new(), |acc: Vec<Vec<usize>>, k| keyed(acc, k));

    (locks, keys)
}

fn keyed(acc: Vec<Vec<usize>>, grid: &str) -> Vec<Vec<usize>> {
    let transposed = transpose2(
        grid.lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
    );

    let mut acc = acc;
    acc.push(
        transposed
            .iter()
            .map(|l| {
                // we want the empty spaces (.) for locks, and the # for keys
                l.into_iter().filter(|ch| Some(*ch) == l.last()).count()
            })
            .collect::<Vec<usize>>(),
    );

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
