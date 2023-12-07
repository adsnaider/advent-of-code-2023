use day4::Card;
use util::read_input;

fn main() {
    let mut sum = 0;
    for line in read_input() {
        let card: Card = line.unwrap().parse().unwrap();

        let mut score = 0;
        for _ in card.get_winners() {
            if score == 0 {
                score = 1;
            } else {
                score *= 2;
            }
        }

        sum += score;
    }
    println!("{sum}");
}
