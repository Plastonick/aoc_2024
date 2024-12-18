advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let mut output = create_drive(input);

    let mut left_index = 0;
    let mut right_index = output.len() - 1;

    while left_index < right_index {
        // find the next -1 entry
        while output[left_index] != -1 {
            left_index += 1;
        }

        // find the next not -1 entry
        while output[right_index] == -1 {
            right_index -= 1;
        }

        // move elements over
        output[left_index] = output[right_index];
        output[right_index] = -1;

        // scan next
        left_index += 1;
        right_index -= 1;
    }

    Some(calculate_checksum(&output))
}

pub fn part_two(input: &str) -> Option<usize> {
    // TODO it's probably nicer to store the "empty" sequence positions + lengths, to facilitate much faster scanning!

    let mut output = create_drive(input);
    let mut start_file_index = output.len() - 1;

    loop {
        // find the next contiguous sequence of positive integers
        let next_file = find_next_file(&output, start_file_index);
        if next_file.is_none() {
            // nothing else to move over, so stop looking!
            break;
        }

        start_file_index = next_file.unwrap().0;
        let end_file_index = next_file.unwrap().1;

        // now we need to find a space that will fit it!
        let length = (end_file_index - start_file_index) + 1;
        if let Some(left_index) = empty_space(&output, start_file_index, length) {
            for i in 0..length {
                // juggle the values around
                output[left_index + i] = output[start_file_index + i];
                output[start_file_index + i] = -1;
            }
        }

        // move our scanner back a bit more
        start_file_index -= 1;
    }

    Some(calculate_checksum(&output))
}

fn find_next_file(output: &Vec<i32>, from: usize) -> Option<(usize, usize)> {
    let mut right_index = from;

    // find the next not -1 entry
    while output[right_index] == -1 {
        if right_index == 0 {
            return None;
        }

        right_index -= 1;
    }

    // how long is our file?
    let right_value = output[right_index];
    let start_right_index = right_index;
    while output[right_index] == right_value {
        if right_index == 0 {
            return None;
        }

        right_index -= 1;
    }

    Some((right_index + 1, start_right_index))
}

fn create_drive(input: &str) -> Vec<i32> {
    let digits = input
        .chars()
        .filter_map(|ch| ch.to_digit(10))
        .collect::<Vec<u32>>();

    let mut output = vec![-1; digits.iter().map(|x| *x as usize).sum()];
    let mut index = 0;
    for (digit_index, number) in digits.iter().enumerate() {
        // as opposed to empty space on the disk
        let is_file = digit_index % 2 == 0;
        for _ in 0..*number {
            output[index] = if is_file {
                (digit_index / 2) as i32
            } else {
                -1
            };
            index += 1;
        }
    }
    output
}

fn calculate_checksum(output: &Vec<i32>) -> usize {
    output
        .iter()
        .enumerate()
        .filter_map(|(i, x)| {
            if *x == -1 {
                None
            } else {
                Some(i * *x as usize)
            }
        })
        .sum()
}

fn empty_space(output: &Vec<i32>, up_to: usize, desired_length: usize) -> Option<usize> {
    let mut start_of_block = None;
    for i in 0..up_to {
        let value = output[i];

        if value != -1 {
            start_of_block = None;

            continue;
        }

        if start_of_block.is_none() {
            start_of_block = Some(i)
        }

        let start = start_of_block.expect("Expected some start!");
        if (i - start) + 1 >= desired_length {
            return start_of_block;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
