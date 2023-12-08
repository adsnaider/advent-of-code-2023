use day7::Hand;
use itertools::Itertools;
use util::read_full_input;

fn main() {
    let hands = Hand::parse_input(&read_full_input()).sorted().collect_vec();
    let answer: u64 = hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i as u64 + 1, hand.bid()))
        .map(|(rank, bid)| rank * bid)
        .sum();
    println!("{answer}");
}
