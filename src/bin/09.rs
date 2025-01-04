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
    let mut output = create_drive(input);
    let mut free_spaces = build_free_spaces(&output);
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
        if let Some(free_space_index) = empty_space(&free_spaces, start_file_index, length) {
            let (left_index, _) = free_spaces[free_space_index];

            for i in 0..length {
                output[left_index + i] = output[start_file_index + i];
                output[start_file_index + i] = -1;
            }

            free_spaces[free_space_index].0 += length;
            free_spaces[free_space_index].1 -= length;
        }

        // move our scanner back a bit more
        start_file_index -= 1;
    }

    Some(calculate_checksum(&output))
}

fn find_next_file(output: &[i32], from: usize) -> Option<(usize, usize)> {
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

fn build_free_spaces(drive: &[i32]) -> Vec<(usize, usize)> {
    let mut spaces = vec![];
    let mut free_length = None;

    for (i, v) in drive.iter().enumerate() {
        if v == &-1 {
            free_length = Some(free_length.unwrap_or(0) + 1);
        } else {
            match free_length {
                None => {}
                Some(length) => spaces.push((i - length, length)),
            };

            free_length = None
        }
    }

    // if the last area in a drive is empty, there's no point in including it, since nothing could move there anyway

    spaces
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

fn empty_space(
    free_spaces: &Vec<(usize, usize)>,
    up_to: usize,
    desired_length: usize,
) -> Option<usize> {
    for (index, (starts_at, length)) in free_spaces.iter().enumerate() {
        if starts_at > &up_to {
            return None;
        }

        if length >= &desired_length {
            return Some(index);
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
