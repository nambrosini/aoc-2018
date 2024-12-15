use std::collections::HashMap;

use aoc_util::position::Vec2;

advent_of_code::solution!(6);

fn parse(input: &str) -> Vec<Vec2> {
    input
        .lines()
        .map(|l| {
            let mut split = l.split(", ");
            Vec2 {
                x: split.next().unwrap().parse().unwrap(),
                y: split.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

fn bounding_box(input: &[Vec2]) -> (i64, i64, i64, i64) {
    let first = &input[0];
    input
        .iter()
        .fold((first.x, first.y, first.x, first.y), |acc, i| {
            (
                acc.0.min(i.x),
                acc.1.min(i.y),
                acc.2.max(i.x),
                acc.3.max(i.y),
            )
        })
}

pub fn part_one(input: &str) -> Option<i64> {
    let input = parse(input);
    let bbox = bounding_box(&input);
    let final_bounds = (bbox.0 - 400, bbox.1 - 400, bbox.2 + 400, bbox.3 + 400);

    let input_filtered: Vec<Vec2> = input
        .iter()
        .filter_map(|p| {
            if p.x == bbox.0 || p.x == bbox.2 || p.y == bbox.1 || p.y == bbox.3 {
                None
            } else {
                Some(*p)
            }
        })
        .collect();

    let mut hash_map: HashMap<Vec2, i64> = HashMap::default();
    (final_bounds.0..=final_bounds.2)
        .flat_map(|x| (final_bounds.1..=final_bounds.3).map(move |y| Vec2::new(x, y)))
        .for_each(|i| {
            let (point, _, count) = input.iter().map(|p| (p, p.distance(&i))).fold(
                (Vec2::new(0, 0), 100000, 0),
                |acc, i| match i.1 {
                    x if x < acc.1 => (*i.0, i.1, 1),
                    x if x == acc.1 => (*i.0, i.1, acc.2 + 1),
                    _ => acc,
                },
            );
            if count == 1 {
                let p_val = hash_map.entry(point).or_insert(0);
                *p_val += 1;
            }
        });

    Some(
        *hash_map
            .iter()
            .filter_map(|(k, v)| {
                if input_filtered.iter().filter(|p| *p == k).count() == 1 {
                    if *v < 10000 {
                        Some(v)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .max()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);
    let bbox = bounding_box(&input);

    Some(
        (bbox.0..=bbox.2)
            .flat_map(|x| (bbox.1..=bbox.3).map(move |y| Vec2::new(x, y)))
            .filter(|p| input.iter().map(|pp| pp.distance(p)).sum::<u64>() < 10000)
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(17));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(72));
    }
}
