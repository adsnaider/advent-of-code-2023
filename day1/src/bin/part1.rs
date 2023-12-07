use day1::read_input;

fn main() {
    let mut sum: u64 = 0;
    for line in read_input() {
        let line = line.unwrap();
        let first_digit = line.find(|c: char| c.is_digit(10)).unwrap();
        let first_digit = line.bytes().nth(first_digit).unwrap() - b'0';
        let last_digit = line.rfind(|c: char| c.is_digit(10)).unwrap();
        let last_digit = line.bytes().nth(last_digit).unwrap() - b'0';

        sum += first_digit as u64 * 10 + last_digit as u64;
    }
    println!("The sum is {sum}");
}
