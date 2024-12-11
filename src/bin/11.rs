advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let numbers = parse(input);

    Some(blink_n(&numbers, 25).len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let numbers = parse(input);

    Some(blink_n(&numbers, 75).len())
}

fn blink_n(initial: &Vec<usize>, times: usize) -> Vec<usize> {
    // todo each stone is operated on independently so we can basically ask the question:
    // "how many stones will stone A turn into after 'x' blinks
    // and each time we find that stone we know the answer

    let mut numbers = initial.clone();
    for _ in 0..times {
        numbers = numbers
            .iter()
            .map(|num| {
                if num == &0 {
                    vec![1]
                } else if num.ilog10() % 2 == 1 {
                    // even number of digits
                    let divisor = 10_i32.pow(1 + num.ilog10() / 2) as usize;

                    // split digits up
                    let left = num / divisor;
                    let right = num - (left * divisor);

                    vec![left, right]
                } else {
                    vec![*num * 2024]
                }
            })
            .flatten()
            .collect::<Vec<usize>>();
    }

    numbers
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
        assert_eq!(result, None);
    }
}
