use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use regex::Regex;

#[derive(Debug)]
pub struct Wasteland<'a> {
    path: Directions,
    map: Map<'a>,
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Directions {
    directions: Vec<Direction>,
}

#[derive(Debug)]
pub struct Map<'a> {
    map: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> Wasteland<'a> {
    pub fn parse(input: &'a str) -> Self {
        let mut lines = input.lines();
        let directions = Directions::parse(lines.next().unwrap());

        assert!(lines.next().unwrap().is_empty());

        let re = Regex::new(r"(?<start>\w+)\s*=\s*\((?<left>\w+),\s*(?<right>\w+)\)").unwrap();

        let mut map = HashMap::new();

        while let Some(line) = lines.next() {
            let caps = re.captures(line).unwrap();
            let start = caps.name("start").unwrap().as_str();
            let left = caps.name("left").unwrap().as_str();
            let right = caps.name("right").unwrap().as_str();

            assert!(map.insert(start, (left, right)).is_none());
        }

        Self {
            path: directions,
            map: Map { map },
        }
    }

    pub fn find_zzz(&self) -> u64 {
        let mut current = "AAA";
        let mut steps = 0;

        while current != "ZZZ" {
            current = match self.path[steps as usize % self.path.len()] {
                Direction::Left => self.map[current].0,
                Direction::Right => self.map[current].1,
            };
            steps += 1;
        }
        steps
    }

    fn find_any_z(&self, start: &str) -> u64 {
        let mut current = start;
        let mut steps = 0;

        while !current.ends_with('Z') {
            current = match self.path[steps as usize % self.path.len()] {
                Direction::Left => self.map[current].0,
                Direction::Right => self.map[current].1,
            };
            steps += 1;
        }
        steps
    }

    pub fn find_all_z(&self) -> u64 {
        self.map
            .keys()
            .copied()
            .filter(|s| s.ends_with('A'))
            .map(|start| self.find_any_z(start))
            .reduce(lcm)
            .unwrap()
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

impl Directions {
    pub fn parse(input: &str) -> Self {
        let directions = input
            .chars()
            .map(|c| match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                c => panic!("Unknown direction {c}"),
            })
            .collect();
        Self { directions }
    }
}

impl Deref for Directions {
    type Target = Vec<Direction>;

    fn deref(&self) -> &Self::Target {
        &self.directions
    }
}
impl DerefMut for Directions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.directions
    }
}

impl<'a> Deref for Map<'a> {
    type Target = HashMap<&'a str, (&'a str, &'a str)>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}
impl<'a> DerefMut for Map<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
