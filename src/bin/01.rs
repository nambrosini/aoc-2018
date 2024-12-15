use std::collections::HashSet;

advent_of_code::solution!(1);

fn parse(input: &str) -> Vec<i64> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}
pub fn part_one(input: &str) -> Option<i64> {
    let input = parse(input);
    Some(input.iter().sum())
}

pub fn part_two(input: &str) -> Option<i64> {
    let input = parse(input);
    let mut set = HashSet::new();
    let mut counter = 1;

    let mut current = input[0];
    while !set.contains(&current) {
        set.insert(current);
        current += input[counter];
        counter = (counter + 1) % input.len();
    }

    Some(current)
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
        assert_eq!(result, Some(2));
    }
}
