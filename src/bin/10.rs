use std::fmt::Display;

use aoc_util::position::Vec2;
use regex::Regex;

advent_of_code::solution!(10);

fn parse(input: &str) -> Map {
    let points: Vec<Point> = input.lines().map(|l| l.into()).collect();

    Map::new(points)
}
pub fn part_one(input: &str) -> Option<i64> {
    let mut map: Map = parse(input);
    for _ in 1..1_000_000 {
        map.simulate();

        if map.get_height() <= 100 && map.get_width() <= 100 {
            println!("{}", map);
        }
    }
    Some(0)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut map: Map = parse(input);
    for i in 1..1_000_000 {
        map.simulate();

        if map.get_height() <= 80 && map.get_width() <= 80 {
            println!("{}", i);
            println!("{}", map);
        }
    }
    Some(0)
}

#[derive(Clone)]
pub struct Map {
    points: Vec<Point>,
}

impl Map {
    fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    fn simulate(&mut self) {
        self.points.iter_mut().for_each(|p| p.tick())
    }

    fn get_width(&self) -> i64 {
        let min_x = self
            .points
            .iter()
            .min_by_key(|p| p.position.x)
            .unwrap()
            .position
            .x;
        let max_x = self
            .points
            .iter()
            .max_by_key(|p| p.position.x)
            .unwrap()
            .position
            .x;

        max_x - min_x
    }

    fn get_height(&self) -> i64 {
        let min_y = self
            .points
            .iter()
            .min_by_key(|p| p.position.y)
            .unwrap()
            .position
            .y;
        let max_y = self
            .points
            .iter()
            .max_by_key(|p| p.position.y)
            .unwrap()
            .position
            .y;

        max_y - min_y
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_x = self
            .points
            .iter()
            .min_by_key(|p| p.position.x)
            .unwrap()
            .position
            .x;
        let max_x = self
            .points
            .iter()
            .max_by_key(|p| p.position.x)
            .unwrap()
            .position
            .x;
        let min_y = self
            .points
            .iter()
            .min_by_key(|p| p.position.y)
            .unwrap()
            .position
            .y;
        let max_y = self
            .points
            .iter()
            .max_by_key(|p| p.position.y)
            .unwrap()
            .position
            .y;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let vec = Vec2::new(x, y);
                if self.points.iter().any(|p| p.position == vec) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct Point {
    position: Vec2,
    velocity: Vec2,
}

impl Point {
    fn tick(&mut self) {
        self.position += self.velocity;
    }
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let re = Regex::new(r"position=<\s*(?P<p_x>-?\d+),\s*(?P<p_y>-?\d+)> velocity=<\s*(?P<v_x>-?\d+),\s*(?P<v_y>-?\d+)>").unwrap();

        let caps = re.captures(s).unwrap();

        let position: Vec2 = Vec2::new(caps["p_x"].parse().unwrap(), caps["p_y"].parse().unwrap());
        let velocity: Vec2 = Vec2::new(caps["v_x"].parse().unwrap(), caps["v_y"].parse().unwrap());

        Self { position, velocity }
    }
}
