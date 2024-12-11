use cached::proc_macro::cached;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let numbers = parse(input);

    let num_stones = numbers
        .into_iter()
        .map(|number| stones_after_blinks(number, 25))
        .sum::<usize>();

    Some(num_stones)
}

pub fn part_two(input: &str) -> Option<usize> {
    let numbers = parse(input);
    let num_stones = numbers
        .into_iter()
        .map(|number| stones_after_blinks(number, 75))
        .sum::<usize>();

    Some(num_stones)
}

#[cached]
fn stones_after_blinks(value: usize, blinks: usize) -> usize {
    if blinks == 0 {
        return 1;
    }

    if value == 0 {
        stones_after_blinks(1, blinks - 1)
    } else if value.ilog10() % 2 == 1 {
        // even number of digits
        let divisor = 10_i32.pow(1 + value.ilog10() / 2) as usize;

        // split digits up
        let left = value / divisor;
        let right = value - (left * divisor);

        let left_count = stones_after_blinks(left, blinks - 1);
        let right_count = stones_after_blinks(right, blinks - 1);

        left_count + right_count
    } else {
        stones_after_blinks(value * 2024, blinks - 1)
    }
}

fn parse(input: &str) -> Vec<usize> {
    input
        .split_once('\n')
        .expect("Failed to find lines")
        .0
        .split(' ')
        .map(|num| num.parse::<usize>().expect("Not a number!"))
        .collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
