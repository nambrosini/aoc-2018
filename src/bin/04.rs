use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(4);

fn parse(input: &str) -> HashMap<usize, Guard> {
    let mut input: Vec<&str> = input.lines().collect();
    input.sort();

    let input: Vec<Message> = input.iter().copied().map(|line| line.into()).collect();
    parse_guards(&input)
}

pub fn part_one(input: &str) -> Option<usize> {
    let guards = parse(input);

    let guards: HashMap<usize, Vec<usize>> =
        guards.iter().map(|(&k, g)| (k, g.get_freq_min())).collect();

    let minutes_asleep = guards
        .iter()
        .max_by_key(|(_, g)| g.iter().sum::<usize>())
        .unwrap();

    Some(
        minutes_asleep.0
            * minutes_asleep
                .1
                .iter()
                .enumerate()
                .max_by_key(|x| x.1)
                .unwrap()
                .0,
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let guards = parse(input);

    let guards: HashMap<usize, Vec<usize>> =
        guards.iter().map(|(&k, g)| (k, g.get_freq_min())).collect();

    let minutes_asleep = guards
        .iter()
        .max_by_key(|(_, g)| g.iter().sum::<usize>())
        .unwrap();

    Some(
        minutes_asleep.0
            * minutes_asleep
                .1
                .iter()
                .enumerate()
                .max_by_key(|x| x.1)
                .unwrap()
                .0,
    )
}
#[derive(Debug, Clone)]
struct Guard {
    id: usize,
    map: HashMap<(usize, usize), [bool; 60]>,
}

impl Guard {
    fn new(id: usize) -> Self {
        Self {
            id,
            map: HashMap::new(),
        }
    }

    fn update(&mut self, message: &Message) {
        let key = (message.day, message.month);
        match message.action {
            Action::Asleep => {
                let e = self.map.entry(key).or_insert([false; 60]);
                for m in e.iter_mut().take(60).skip(message.minute) {
                    *m = true;
                }
            }
            Action::Wakes => {
                let e = self.map.entry(key).or_insert([false; 60]);
                for m in e.iter_mut().take(60).skip(message.minute) {
                    *m = false;
                }
            }
            _ => {}
        }
    }

    fn get_freq_min(&self) -> Vec<usize> {
        let mut v: [usize; 60] = [0; 60];

        for (_, d) in self.map.iter() {
            for i in 0..d.len() {
                v[i] += d[i] as usize;
            }
        }

        v.to_vec()
    }
}

fn parse_guards(input: &[Message]) -> HashMap<usize, Guard> {
    let input = input.iter();
    let mut guards: HashMap<usize, Guard> = HashMap::new();

    let mut guard: Option<Guard> = None;

    for next in input {
        match next.action {
            Action::Begins => {
                let id = next.id.unwrap();
                if let Some(g) = guard.take() {
                    guards.insert(g.id, g);
                }

                guard = Some(guards.entry(id).or_insert_with(|| Guard::new(id)).clone());
            }
            _ => {
                if let Some(g) = guard.take() {
                    let mut g = g.clone();
                    g.update(next);
                    guard = Some(g);
                }
            }
        }
    }

    if let Some(g) = guard.take() {
        guards.insert(g.id, g);
    }

    guards
}

#[derive(Debug, PartialEq)]
enum Action {
    Begins,
    Asleep,
    Wakes,
}

impl From<&str> for Action {
    fn from(s: &str) -> Self {
        if s.contains("begins") {
            Action::Begins
        } else if s.contains("wakes") {
            Action::Wakes
        } else if s.contains("asleep") {
            Action::Asleep
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug)]
pub struct Message {
    month: usize,
    day: usize,
    minute: usize,
    action: Action,
    id: Option<usize>,
}

impl From<&str> for Message {
    fn from(s: &str) -> Self {
        let re =
            Regex::new(r"\[1518-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (Guard #(\d+).+|.+)").unwrap();

        let cap = re.captures_iter(s).next().unwrap();

        let id = cap.get(6).map(|x| x.as_str().parse::<usize>().unwrap());

        Self {
            month: cap[1].parse().unwrap(),
            day: cap[2].parse().unwrap(),
            minute: cap[4].parse().unwrap(),
            action: cap[5].into(),
            id,
        }
    }
}
