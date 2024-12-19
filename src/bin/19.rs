use cached::proc_macro::cached;

advent_of_code::solution!(19);

type Pattern = String;

pub fn part_one(input: &str) -> Option<usize> {
    let (types, wanted) = parse(input);

    Some(
        wanted
            .into_iter()
            .map(|p| num_combinations(p, &types) > 0)
            .filter(|x| *x)
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (types, wanted) = parse(input);

    Some(
        wanted
            .into_iter()
            .map(|p| num_combinations(p, &types))
            .sum(),
    )
}

#[cached(key = "String", convert = r#"{ pattern.to_owned() }"#)]
fn num_combinations(pattern: Pattern, from_prefixes: &Vec<Pattern>) -> usize {
    if pattern.is_empty() {
        return 1;
    }

    from_prefixes
        .iter()
        .map(|prefix| {
            if pattern.starts_with(prefix) {
                let remaining = pattern[prefix.len()..].to_owned();

                num_combinations(remaining, from_prefixes)
            } else {
                0
            }
        })
        .sum()
}

fn parse(input: &str) -> (Vec<Pattern>, Vec<Pattern>) {
    let (types, wanted) = input.split_once("\n\n").unwrap();

    (
        types.split(", ").map(|str| str.chars().collect()).collect(),
        wanted.lines().map(|str| str.chars().collect()).collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
