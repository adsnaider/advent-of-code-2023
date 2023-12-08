use std::{cmp::Ordering, collections::HashMap, ops::AddAssign};

#[cfg(feature = "part2")]
use cfg_if::cfg_if;
#[cfg(feature = "part2")]
use itertools::Itertools;

#[derive(Debug)]
pub struct Hand {
    cards: [Card; 5],
    bid: u64,
}

#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Card {
    value: u8,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    Pair = 2,
    HighCard = 1,
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}

impl Hand {
    pub fn parse_input(s: &str) -> impl Iterator<Item = Self> + '_ {
        s.lines().map(Hand::parse)
    }

    pub fn parse(s: &str) -> Self {
        let mut parts = s.split_whitespace();
        let cards = parts
            .next()
            .unwrap()
            .chars()
            .map(|c| Card::from_char(c).unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let bid = parts.next().unwrap().parse().unwrap();
        Self { cards, bid }
    }

    #[cfg(not(feature = "part2"))]
    pub fn hand_type(&self) -> HandType {
        let mut map: HashMap<Card, u8> = HashMap::new();
        for card in self.cards.iter().copied() {
            map.entry(card).or_default().add_assign(1);
        }
        let values: Vec<u8> = map.values().copied().collect();
        if values.len() == 1 {
            HandType::FiveOfAKind
        } else if values.len() == 2 {
            if values.contains(&4) {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        } else if values.len() == 3 {
            if values.contains(&3) {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        } else if values.len() == 4 {
            HandType::Pair
        } else {
            HandType::HighCard
        }
    }

    #[cfg(feature = "part2")]
    pub fn hand_type(&self) -> HandType {
        let mut map: HashMap<Card, u8> = HashMap::new();
        let mut jacks = 0;
        for card in self.cards.iter().copied() {
            if card.is_jack() {
                jacks += 1;
            } else {
                map.entry(card).or_default().add_assign(1);
            }
        }
        if jacks == 5 {
            return HandType::FiveOfAKind;
        }
        let mut values: Vec<u8> = map.values().copied().sorted().rev().collect();
        values[0] += jacks;
        if values.len() == 1 {
            HandType::FiveOfAKind
        } else if values.len() == 2 {
            if values.contains(&4) {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        } else if values.len() == 3 {
            if values.contains(&3) {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        } else if values.len() == 4 {
            HandType::Pair
        } else {
            HandType::HighCard
        }
    }

    pub fn bid(&self) -> u64 {
        self.bid
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}
impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().partial_cmp(&other.hand_type()).unwrap() {
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => {}
            Ordering::Greater => return Ordering::Greater,
        }
        for (us, them) in self.cards.iter().zip(other.cards.iter()) {
            match us.cmp(them) {
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => {}
                Ordering::Greater => return Ordering::Greater,
            }
        }
        Ordering::Equal
    }
}

impl Card {
    pub fn is_jack(&self) -> bool {
        let j = if cfg!(feature = "part2") { 0 } else { 11 };
        self.value == j
    }

    pub fn from_char(c: char) -> Option<Self> {
        let j = if cfg!(feature = "part2") { 0 } else { 11 };
        let value = match c {
            c if c.is_digit(10) => c.to_digit(10).unwrap() as u8,
            'T' => 10,
            'J' => j,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => return None,
        };
        Some(Card { value })
    }
}
