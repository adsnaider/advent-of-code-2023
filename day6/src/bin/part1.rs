use day6::Races;
use util::read_full_input;

fn main() {
    let races = Races::parse(&read_full_input());
    let mut answer: u64 = 1;
    for race in races.races.iter() {
        answer *= race.ways_to_win();
    }
    println!("{answer}");
}
