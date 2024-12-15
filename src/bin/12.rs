use std::{collections::HashMap, fmt::Display};

advent_of_code::solution!(12);

fn parse(input: &str) -> Pots {
    input.into()
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut pots = parse(input);
    for _ in 0..20 {
        pots.simulate();
    }

    Some(
        pots.pots
            .iter()
            .filter(|(_, c)| c == &&'#')
            .map(|(i, _)| i)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut pots = parse(input);
    Some(run(&mut pots, 50_000_000_000))
}

fn run(pots: &mut Pots, cycles: usize) -> i64 {
    for _ in 0..cycles {
        pots.simulate();
    }

    pots.calc()
}

#[derive(Clone)]
pub struct Pots {
    pots: HashMap<i64, char>,
    states: Vec<State>,
}

impl Pots {
    fn simulate(&mut self) {
        let mut pots_clone = self.pots.clone();

        let min = *self.pots.keys().min().unwrap();
        let max = *self.pots.keys().max().unwrap();

        for i in min - 2..=max + 2 {
            let mut string = String::new();
            for j in i - 2..=i + 2 {
                if let Some(c) = self.pots.get(&j) {
                    string.push(*c);
                } else {
                    string.push('.');
                }
            }
            let val = self.get_pot_from_states(string);
            if !((i < min || i > max) && val == '.') {
                let entry = pots_clone.entry(i).or_insert('.');
                *entry = val;
            }
        }

        self.pots = pots_clone;
    }

    fn get_pot_from_states(&self, string: String) -> char {
        if let Some(x) = self.states.iter().find(|state| state.left == string) {
            x.right
        } else {
            '.'
        }
    }

    fn calc(&self) -> i64 {
        self.pots
            .iter()
            .filter(|(_, c)| c == &&'#')
            .map(|(i, _)| i)
            .sum()
    }
}

impl From<&str> for Pots {
    fn from(s: &str) -> Self {
        let split: Vec<&str> = s.split("\n\n").collect();

        let pots = split[0]
            .split_whitespace()
            .last()
            .unwrap()
            .chars()
            .enumerate()
            .map(|(i, e)| (i as i64, e))
            .collect();
        let states = split[1].lines().map(|l| l.into()).collect();

        Self { pots, states }
    }
}

impl Display for Pots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min = *self.pots.keys().min().unwrap();
        let max = *self.pots.keys().max().unwrap();

        for i in min..=max {
            write!(f, "{}", self.pots[&i])?;
        }
        writeln!(f)
    }
}

#[derive(Clone)]
struct State {
    left: String,
    right: char,
}

impl From<&str> for State {
    fn from(s: &str) -> Self {
        let split: Vec<&str> = s.split(" => ").collect();
        let left = split[0].to_string();
        let right = split[1].chars().next().unwrap();

        Self { left, right }
    }
}
