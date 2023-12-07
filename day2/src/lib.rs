use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub sets: Vec<Roll>,
}

#[derive(Debug)]
pub struct Roll {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug)]
pub enum ParseError {
    RollError,
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static GAME_RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^Game (?<id>\d+): (?<rolls>.*)").unwrap());
        static DICE_RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(?<count>\d+) (?<color>\w+)").unwrap());

        let caps = GAME_RE.captures(s).unwrap();
        let id = caps["id"].parse().unwrap();

        let rolls_str = &caps["rolls"];

        let mut rolls = Vec::new();
        for roll_str in rolls_str.split(';') {
            let mut roll = Roll::default();
            for caps in DICE_RE.captures_iter(roll_str) {
                let count = caps["count"].parse().unwrap();
                let color = &caps["color"];
                roll.set(color, count)?;
            }
            rolls.push(roll)
        }
        Ok(Game { id, sets: rolls })
    }
}

impl Default for Roll {
    fn default() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl Roll {
    pub fn set(&mut self, color: &str, count: u8) -> Result<(), ParseError> {
        match color {
            "red" => self.red = count,
            "green" => self.green = count,
            "blue" => self.blue = count,
            _ => return Err(ParseError::RollError),
        }
        Ok(())
    }
    pub fn is_valid_for(&self, red: u8, green: u8, blue: u8) -> bool {
        self.red <= red && self.green <= green && self.blue <= blue
    }
}

impl Game {
    pub fn is_valid_for(&self, red: u8, green: u8, blue: u8) -> bool {
        for roll in self.sets.iter() {
            if !roll.is_valid_for(red, green, blue) {
                return false;
            }
        }
        return true;
    }
}
