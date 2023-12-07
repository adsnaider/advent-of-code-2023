use day5::Garden;
use util::read_full_input;

fn main() {
    let garden = Garden::parse(&read_full_input(), day5::Part::One);
    let min = garden
        .seeds()
        .map(|s| garden.location_for(s))
        .min()
        .unwrap();
    println!("{min}");
}
