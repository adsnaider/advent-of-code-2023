use day2::Game;
use util::read_input;

fn main() {
    let mut sum = 0;
    for line in read_input() {
        let game: Game = line.unwrap().parse().unwrap();
        if game.is_valid_for(12, 13, 14) {
            sum += game.id;
        }
    }
    println!("Possible games: {sum}");
}
