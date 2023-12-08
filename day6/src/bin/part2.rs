use day6::Race;
use util::read_full_input;

fn main() {
    let race = Race::parse(&read_full_input());
    println!("{}", race.ways_to_win());
}
