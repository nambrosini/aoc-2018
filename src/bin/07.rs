use std::collections::HashMap;

advent_of_code::solution!(7);

const WORKERS_NUM: usize = 5;

fn parse(input: &str) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for l in input.lines() {
        let split: Vec<&str> = l.split_whitespace().collect();

        let first = split[1].to_string();
        let second = split[7].to_string();

        let entry = map.entry(first).or_default();
        entry.push(second);
    }

    map
}

pub fn part_one(input: &str) -> Option<String> {
    let mut map = parse(input);

    let mut queue = find_start(&map);
    queue.sort_by(|a, b| b.cmp(a));
    let mut done: Vec<String> = vec![];

    while let Some(x) = queue.pop() {
        if done.contains(&x) || map.values().flatten().any(|v| v == &x) {
            continue;
        }
        done.push(x.clone());

        if let Some(v) = map.get(&x) {
            let mut v = v.to_vec();
            queue.append(&mut v);
            map.remove(&x);
        }

        queue.sort_by(|a, b| b.cmp(a));
    }

    Some(done.iter().map(|d| d.as_str()).collect())
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut map = parse(input);
    let mut queue = find_start(&map);
    queue.sort_by(|a, b| b.cmp(a));

    let mut done: Vec<String> = vec![];
    let mut workers: Vec<Worker> = vec![];

    let mut ticks = 0;

    while !map.is_empty() || !queue.is_empty() || !workers.is_empty() {
        let mut index_remove: Vec<usize> = vec![];

        for (i, w) in workers.iter_mut().enumerate() {
            if let Some(letter) = w.step() {
                done.push(letter.to_string());
                if let Some(v) = map.get(&letter) {
                    let mut v = v.to_vec();
                    queue.append(&mut v);
                    map.remove(&letter);
                }
                index_remove.push(i);
                queue.sort_by(|a, b| b.cmp(a));
            }
        }

        for i in &index_remove {
            workers.remove(*i);
        }

        while workers.len() < WORKERS_NUM {
            if let Some(x) = queue.pop() {
                if done.contains(&x)
                    || map.values().flatten().any(|v| v == &x)
                    || workers.iter().map(|w| w.letter.clone()).any(|l| l == x)
                {
                    continue;
                }
                workers.push(Worker::new(x));
            } else {
                break;
            }
        }
        ticks += 1;
    }

    Some(ticks - 1)
}

#[derive(Debug)]
struct Worker {
    letter: String,
    max_time: usize,
    tick: usize,
}

impl Worker {
    fn new(letter: String) -> Self {
        let max_time = 60 + letter.chars().next().unwrap() as usize - 64;
        Self {
            letter,
            max_time,
            tick: 0,
        }
    }

    fn step(&mut self) -> Option<String> {
        self.tick += 1;
        if self.tick >= self.max_time {
            Some(self.letter.clone())
        } else {
            None
        }
    }
}

fn find_start(map: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut start = vec![];

    for k in map.keys() {
        if !map.values().flatten().any(|v| v == k) {
            start.push(k.to_string());
        }
    }

    start
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("CABDFE".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(253));
    }
}
