use day8::Wasteland;
use util::read_full_input;

fn main() {
    let input = read_full_input();
    let wasteland = Wasteland::parse(&input);
    let answer = wasteland.find_zzz();
    println!("{answer}");
}
