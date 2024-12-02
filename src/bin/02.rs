advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    let lines = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .filter(is_safe)
        .collect::<Vec<Vec<isize>>>();

    Some(lines.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .filter(|x| is_safe(x) || remove_any_is_safe(x))
        .collect::<Vec<Vec<isize>>>();

    Some(lines.len())
}

fn remove_any_is_safe(v: &Vec<isize>) -> bool {
    v.iter().enumerate().any(|(i, _)| {
        let mut vector_without = v.clone();
        vector_without.remove(i);

        is_safe(&vector_without)
    })
}

fn is_safe(v: &Vec<isize>) -> bool {
    let piece_wise_diffs = v.windows(2).map(|w| w[0] - w[1]).collect::<Vec<isize>>();

    between_one_and_three(&piece_wise_diffs) && monotonic(&piece_wise_diffs)
}

fn between_one_and_three(v: &Vec<isize>) -> bool {
    v.iter().all(|&el| el.abs() >= 1 && el.abs() <= 3)
}

fn monotonic(v: &Vec<isize>) -> bool {
    v.iter().all(|&el| el >= 0) || v.iter().all(|&el| el <= 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
