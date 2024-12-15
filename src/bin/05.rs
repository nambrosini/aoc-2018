use std::collections::HashSet;

advent_of_code::solution!(5);

fn parse(input: &str) -> Vec<char> {
    input.chars().collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    Some(collapse(&input).len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);
    let elements: HashSet<char> = input
        .iter()
        .copied()
        .filter(|&c| c.is_ascii_lowercase())
        .collect();

    let mut min = usize::MAX;

    for c in elements {
        let input: Vec<char> = input
            .iter()
            .filter(|&&x| x != c && x as usize != c as usize - 32)
            .copied()
            .collect();
        let collapsed_len = collapse(&input).len();

        if collapsed_len < min {
            min = collapsed_len;
        }
    }

    Some(min)
}

pub fn collapse(input: &[char]) -> Vec<char> {
    let mut input = input.to_vec();
    let mut destroyed = true;

    while destroyed {
        destroyed = false;

        let mut v = vec![];
        let mut count = 0;

        while count < input.len() - 1 {
            if (input[count] as i32 - input[count + 1] as i32).abs() != 32 {
                v.push(input[count]);
            } else {
                destroyed = true;
                count += 1;
            }

            count += 1;
        }

        if count == input.len() - 1 {
            v.push(input[count]);
        }

        input = v.clone();
    }

    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
