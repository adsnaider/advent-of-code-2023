use std::collections::HashMap;

use day4::Card;
use util::read_input;

fn main() {
    let mut sum = 0;
    let mut copies = HashMap::new();
    for line in read_input() {
        let card: Card = line.unwrap().parse().unwrap();

        let winners = card.get_winners().count() as u32;
        let current_copies = *copies.entry(card.id).or_insert(1);
        for i in 0u32..winners {
            *copies.entry(card.id + i + 1).or_insert(1) += current_copies;
        }

        sum += current_copies;
    }
    println!("{sum}");
}
