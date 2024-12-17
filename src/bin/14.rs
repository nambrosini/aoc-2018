advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<String> {
    let input = input.trim().parse().unwrap();
    let mut score = String::from("37");
    let mut elf1: usize = 0;
    let mut elf2: usize = 1;

    for i in 0..input {
        print!("{}\r", i);
        let first = score.chars().nth(elf1).unwrap().to_digit(10).unwrap();
        let second = score.chars().nth(elf2).unwrap().to_digit(10).unwrap();
        score.push_str(&(first + second).to_string());
        elf1 = (elf1 + first as usize + 1) % score.len();
        elf2 = (elf2 + second as usize + 1) % score.len();
    }

    Some(score[input..input + 10].to_string())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut score = String::from("37");
    let mut elf1: usize = 0;
    let mut elf2: usize = 1;

    while score.len() < input.len() || !score[score.len() - input.len()..].contains(input) {
        let first = score.chars().nth(elf1).unwrap().to_digit(10).unwrap();
        let second = score.chars().nth(elf2).unwrap().to_digit(10).unwrap();
        score.push_str(&(first + second).to_string());
        elf1 = (elf1 + first as usize + 1) % score.len();
        elf2 = (elf2 + second as usize + 1) % score.len();
    }

    Some(score.len() - input.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("5941429882".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2018));
    }
}
