#![feature(generators, generator_trait, iter_from_generator)]
use std::{collections::HashMap, convert::Infallible, str::FromStr};

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug)]
pub struct Grid {
    numbers: Vec<(Index, u32)>,
    symbols: HashMap<Index, char>,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Index {
    row: isize,
    col: isize,
}

impl FromStr for Grid {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = Vec::new();
        let mut symbols = HashMap::new();
        for (row, line) in s.split('\n').enumerate() {
            static NUM_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
            static SYMBOL_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^\.\d]").unwrap());

            for m in NUM_RE.find_iter(line) {
                let num = m.as_str().parse().unwrap();
                let col = m.start();
                numbers.push((
                    Index {
                        row: row.try_into().unwrap(),
                        col: col.try_into().unwrap(),
                    },
                    num,
                ));
            }
            for m in SYMBOL_RE.find_iter(line) {
                assert_eq!(m.as_str().len(), 1);
                let symbol = m.as_str().chars().next().unwrap();
                let col = m.start();
                assert!(symbols
                    .insert(
                        Index {
                            row: row.try_into().unwrap(),
                            col: col.try_into().unwrap()
                        },
                        symbol
                    )
                    .is_none());
            }
        }
        Ok(Grid { numbers, symbols })
    }
}

impl Grid {
    pub fn nums(&self) -> impl Iterator<Item = (Index, u32)> + '_ {
        self.numbers.iter().copied()
    }

    pub fn symbol_at(&self, idx: &Index) -> Option<char> {
        self.symbols.get(&idx).copied()
    }

    pub fn adjacent_symbols(&self, idx: Index, width: usize) -> impl Iterator<Item = Index> + '_ {
        let generator = move || {
            let top_left = idx.left().up();

            // Top row
            for i in 0..width + 2 {
                let idx = top_left.rightn(i.try_into().unwrap());
                if self.symbol_at(&idx).is_some() {
                    yield idx;
                }
            }

            // Bottom row
            for i in 0..width + 2 {
                let idx = top_left.downn(2).rightn(i.try_into().unwrap());
                if self.symbol_at(&idx).is_some() {
                    yield idx;
                }
            }

            // left
            let idx = top_left.downn(1);
            if self.symbol_at(&idx).is_some() {
                yield idx;
            }

            // right
            let idx = top_left.downn(1).rightn((width + 1).try_into().unwrap());
            if self.symbol_at(&idx).is_some() {
                yield idx;
            }
        };
        std::iter::from_generator(generator)
    }
}

impl Index {
    pub fn left(&self) -> Index {
        Self {
            row: self.row,
            col: self.col - 1,
        }
    }
    pub fn right(&self) -> Index {
        Self {
            row: self.row,
            col: self.col + 1,
        }
    }
    pub fn up(&self) -> Index {
        Self {
            row: self.row - 1,
            col: self.col,
        }
    }
    pub fn down(&self) -> Index {
        Self {
            row: self.row + 1,
            col: self.col,
        }
    }

    pub fn leftn(&self, n: isize) -> Index {
        Self {
            row: self.row,
            col: self.col - n,
        }
    }
    pub fn rightn(&self, n: isize) -> Index {
        Self {
            row: self.row,
            col: self.col + n,
        }
    }
    pub fn upn(&self, n: isize) -> Index {
        Self {
            row: self.row - n,
            col: self.col,
        }
    }
    pub fn downn(&self, n: isize) -> Index {
        Self {
            row: self.row + n,
            col: self.col,
        }
    }
}
