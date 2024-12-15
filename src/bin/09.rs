use regex::Regex;

advent_of_code::solution!(9);

fn parse(input: &str) -> (usize, usize) {
    let re = Regex::new(r"(?P<players>\d+)[\sa-z;]+(?P<points>\d+)").unwrap();

    let caps = re.captures(input.trim()).unwrap();

    let players = caps["players"].parse().unwrap();
    let points = caps["points"].parse().unwrap();

    (players, points)
}
pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    Some(solve(input.0, input.1))
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);
    Some(solve(input.0, input.1 * 100))
}

fn solve(players: usize, last_marble: usize) -> usize {
    let mut scores = vec![0; players];

    let mut played = vec![0];
    let mut current_index: usize = 0;
    let mut current_played = 1;

    while current_played <= last_marble {
        if current_played % 23 == 0 {
            let removed_marble_index = if let Some(sub) = current_index.checked_sub(7) {
                sub
            } else {
                played.len() - (7 - current_index)
            };
            let removed_marble = played.remove(removed_marble_index);
            scores[current_played % players] += current_played + removed_marble;
            current_index = removed_marble_index
        } else {
            let next_index = (current_index + 1) % played.len();
            if next_index == played.len() {
                played.push(current_played);
            } else {
                played.insert(next_index + 1, current_played);
            }
            current_index = next_index + 1;
        }
        current_played += 1;
    }

    *scores.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37305));
    }
}
