use std::{convert::Infallible, str::FromStr};

#[derive(Debug)]
pub struct Card {
    pub id: u32,
    winners: Vec<u8>,
    numbers: Vec<u8>,
}

impl FromStr for Card {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s
            .split(|c: char| c.is_whitespace() || c == ':')
            .filter(|s| !s.is_empty());
        assert_eq!(words.next(), Some("Card"));
        let id = words.next().unwrap().parse().unwrap();

        let mut winners = Vec::new();
        loop {
            match words.next().unwrap() {
                "|" => break,
                winner => winners.push(winner.parse().unwrap()),
            }
        }

        let mut numbers = Vec::new();
        while let Some(number) = words.next() {
            numbers.push(number.parse().unwrap());
        }
        winners.sort();
        Ok(Self {
            id,
            winners,
            numbers,
        })
    }
}

impl Card {
    pub fn get_winners(&self) -> impl Iterator<Item = u8> + '_ {
        self.numbers
            .iter()
            .filter(|num| self.winners.contains(num))
            .copied()
    }
}
