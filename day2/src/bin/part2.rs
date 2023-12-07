use day2::Game;
use util::read_input;

fn main() {
    let mut sum = 0;
    for line in read_input() {
        let game: Game = line.unwrap().parse().unwrap();
        let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);
        for roll in game.sets {
            min_red = min_red.max(roll.red as u32);
            min_green = min_green.max(roll.green as u32);
            min_blue = min_blue.max(roll.blue as u32);
        }

        let power = min_red * min_green * min_blue;
        sum += power;
    }
    println!("{sum}");
}
