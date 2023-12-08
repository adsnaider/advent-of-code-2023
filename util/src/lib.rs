use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn read_input() -> impl Iterator<Item = Result<String, io::Error>> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines()
}

pub fn read_full_input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}
