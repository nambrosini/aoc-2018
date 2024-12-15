use std::collections::{HashMap, HashSet};

use aoc_util::position::{v, Vec2};
use regex::Regex;

advent_of_code::solution!(3);

fn parse(input: &str) -> Vec<Square> {
    input.lines().map(|l| l.into()).collect()
}

pub fn part_one(input: &str) -> Option<i64> {
    let input = parse(input);
    let mut map: HashMap<Vec2, i64> = HashMap::new();

    for s in input {
        for i in 0..s.width {
            for j in 0..s.height {
                let e = map.entry(v(s.pos.x + i, s.pos.y + j)).or_insert(0);
                *e += 1;
            }
        }
    }

    Some(map.values().filter(|&&v| v > 1).count() as i64)
}

pub fn part_two(input: &str) -> Option<i64> {
    let input = parse(input);
    let mut map: HashMap<Vec2, String> = HashMap::new();

    let mut destroyed = HashSet::new();

    for s in &input {
        for i in 0..s.width {
            for j in 0..s.height {
                let e = map
                    .entry(v(s.pos.x + i, s.pos.y + j))
                    .or_insert(".".to_string());
                match e.as_str() {
                    "." => *e = format!("{}", s.id),
                    "X" => {
                        destroyed.insert(s.id);
                    }
                    _ => {
                        destroyed.insert(e.clone().parse().unwrap());
                        destroyed.insert(s.id);
                        *e = "X".to_string();
                    }
                }
            }
        }
    }

    Some(
        input
            .iter()
            .map(|s| s.id)
            .find(|s| !destroyed.contains(s))
            .unwrap(),
    )
}

#[derive(Debug)]
pub struct Square {
    id: i64,
    pos: Vec2,
    width: i64,
    height: i64,
}

impl From<&str> for Square {
    fn from(s: &str) -> Self {
        let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

        let cap = re.captures_iter(s).next().unwrap();

        Self {
            id: cap[1].parse().unwrap(),
            pos: Vec2::new(cap[2].parse().unwrap(), cap[3].parse().unwrap()),
            width: cap[4].parse().unwrap(),
            height: cap[5].parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
